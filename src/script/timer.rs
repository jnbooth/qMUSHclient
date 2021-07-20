use std::ops::{Deref, DerefMut};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{SendTo, Sender};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Event {
    Repeat { every: Duration, offset: Duration },
    Once(Duration),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(from = "TimerSerde", into = "TimerSerde")]
pub struct Timer {
    pub event: Event,
    pub send: Sender,
    pub active_closed: bool,
}

impl Deref for Timer {
    type Target = Sender;

    fn deref(&self) -> &Self::Target {
        &self.send
    }
}

impl DerefMut for Timer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.send
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        Self {
            event: Event::Repeat {
                every: Duration::new(0, 0),
                offset: Duration::new(0, 0),
            },
            send: Sender::new(),
            active_closed: false,
        }
    }
}

sender_fields! {
    #[derive(Default, Deserialize, Serialize)]
    #[serde(default)]
    struct TimerSerde {
        hour: u64,
        minute: u64,
        second: u64,
        offset_hour: u64,
        offset_minute: u64,
        offset_second: u64,
        at_time: bool,
        active_closed: bool,
    }
}

fn pack_dur(hour: u64, minute: u64, second: u64) -> Duration {
    Duration::from_secs(hour * 3600 + minute * 60 + second)
}
fn unpack_dur(duration: Duration) -> (u64, u64, u64) {
    let secs = duration.as_secs();
    (secs / 3600, (secs % 3600) / 60, secs % 60)
}

impl From<TimerSerde> for Timer {
    fn from(value: TimerSerde) -> Self {
        let event = if value.at_time {
            Event::Once(pack_dur(value.hour, value.minute, value.second))
        } else {
            Event::Repeat {
                every: pack_dur(value.hour, value.minute, value.second),
                offset: pack_dur(value.offset_hour, value.offset_minute, value.offset_second),
            }
        };
        Self {
            event,
            send: sender!(value, Sender {}),
            active_closed: value.active_closed,
        }
    }
}

impl From<Timer> for TimerSerde {
    fn from(value: Timer) -> Self {
        let (at_time, (hour, minute, second), (offset_hour, offset_minute, offset_second)) =
            match value.event {
                Event::Repeat { every, offset } => (false, unpack_dur(every), unpack_dur(offset)),
                Event::Once(dur) => (true, unpack_dur(dur), (0, 0, 0)),
            };

        sender!(
            value.send,
            Self {
                hour,
                minute,
                second,
                offset_hour,
                offset_minute,
                offset_second,
                at_time,
                active_closed: value.active_closed,
            }
        )
    }
}
