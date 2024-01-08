use std::fmt::{self, Display, Formatter};
use std::time::Duration;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Event {
    // Note: this is at the top for Ord-deriving purposes.
    Time(NaiveTime),
    Interval(Duration),
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Interval(every) => {
                let secs = every.as_secs();
                let millis = every.subsec_millis();
                if millis == 0 {
                    write!(
                        f,
                        "{:02}:{:02}:{:02}",
                        secs / 3600,
                        (secs % 3600) / 60,
                        secs % 60
                    )
                } else {
                    write!(
                        f,
                        "{:02}:{:02}:{:02}.{:03}",
                        secs / 3600,
                        (secs % 3600) / 60,
                        secs % 60,
                        millis
                    )
                }
            }
            Self::Time(at) => write!(f, "{}", at.format("%-I:%M %p")),
        }
    }
}
