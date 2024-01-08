use std::borrow::Cow;
use std::time::Duration;

use chrono::{NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

use super::event::Event;
use super::send_to::{sendto_serde, SendTo};
use super::sender::Sender;
use crate::in_place::InPlace;

const NANOS: u64 = 1_000_000_000;

fn duration_from_hms(hour: u64, minute: u64, second: f64) -> Duration {
    Duration::from_nanos((NANOS as f64 * second) as u64 + NANOS * 60 * (minute + 60 * hour))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Timer {
    // Note: this is at the top for Ord-deriving purposes.
    pub send: Sender,
    pub event: Event,
    pub active_closed: bool,
}

impl_deref!(Timer, Sender, send);
impl_asref!(Timer, Sender);

impl Default for Timer {
    fn default() -> Self {
        Self {
            event: Event::Interval(Duration::new(0, 0)),
            send: Sender::default(),
            active_closed: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(default)]
pub struct TimerXml<'a> {
    #[serde(
        borrow,
        default,
        rename = "@name",
        skip_serializing_if = "str::is_empty"
    )]
    label: Cow<'a, str>,
    #[serde(
        borrow,
        default,
        rename = "@script",
        skip_serializing_if = "str::is_empty"
    )]
    script: Cow<'a, str>,
    #[serde(rename = "@enabled")]
    enabled: bool,
    #[serde(rename = "@at_time")]
    at_time: bool,
    #[serde(
        borrow,
        default,
        rename = "@group",
        skip_serializing_if = "str::is_empty"
    )]
    group: Cow<'a, str>,
    #[serde(
        borrow,
        default,
        rename = "@variable",
        skip_serializing_if = "str::is_empty"
    )]
    variable: Cow<'a, str>,
    #[serde(rename = "@one_shot")]
    one_shot: bool,
    #[serde(rename = "@active_closed")]
    active_closed: bool,
    #[serde(with = "sendto_serde", rename = "@send_to")]
    send_to: SendTo,
    #[serde(rename = "@omit_from_output")]
    omit_from_output: bool,
    #[serde(rename = "@omit_from_log")]
    omit_from_log: bool,
    #[serde(rename = "@hour")]
    hour: u64,
    #[serde(rename = "@minute")]
    minute: u64,
    #[serde(rename = "@second")]
    second: f64,
    #[serde(rename = "@temporary")]
    temporary: bool,
    #[serde(borrow, default, rename = "send")]
    text: Vec<Cow<'a, str>>,
}
impl From<TimerXml<'_>> for Timer {
    fn from(value: TimerXml) -> Self {
        let event = if value.at_time {
            Event::Time(
                NaiveTime::from_hms_opt(value.hour as u32, value.minute as u32, value.second as u32)
                    .unwrap(),
            )
        } else {
            Event::Interval(duration_from_hms(value.hour, value.minute, value.second))
        };
        let send = in_place!(
            value,
            Sender {
                    ..label,
                    ..text,
                    ..send_to,
                    ..script,
                    ..group,
                    ..variable,
                    ..enabled,
                    ..one_shot,
                    ..temporary,
                    ..omit_from_output,
                    ..omit_from_log,
            }
        );
        Self {
            event,
            send,
            active_closed: value.active_closed,
        }
    }
}
impl<'a> From<&'a Timer> for TimerXml<'a> {
    fn from(value: &'a Timer) -> Self {
        let (at_time, hour, minute, second) = match value.event {
            Event::Interval(every) => {
                let secs = every.as_secs();
                let hour = secs / 3600;
                let minute = (secs % 3600) / 3600;
                let second = every.subsec_nanos() as f64 / NANOS as f64;
                (false, hour, minute, second)
            }
            Event::Time(time) => (
                true,
                time.hour() as u64,
                time.minute() as u64,
                time.second() as f64,
            ),
        };
        in_place!(
            value,
            Self {
                at_time,
                hour,
                minute,
                second,
                ..label,
                ..text,
                ..send_to,
                ..script,
                ..group,
                ..variable,
                ..enabled,
                ..one_shot,
                ..temporary,
                ..omit_from_output,
                ..omit_from_log,
                ..active_closed,
            }
        )
    }
}
