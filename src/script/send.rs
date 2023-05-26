use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use std::time::Duration;

use chrono::{NaiveTime, Timelike};
use enumeration::Enum;
use qt_gui::q_palette::ColorRole;
use serde::{Deserialize, Serialize};

use super::Regex;
use crate::binding::RColor;
use crate::constants::config;
use crate::DurationExt;

macro_rules! impl_deref {
    ($t:ty, $target:ty, $field:ident) => {
        impl std::ops::Deref for $t {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl std::ops::DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

macro_rules! impl_asref {
    ($t:ty, $target:ty) => {
        impl AsRef<$target> for $t {
            fn as_ref(&self) -> &$target {
                self
            }
        }
        impl AsMut<$target> for $t {
            fn as_mut(&mut self) -> &mut $target {
                self
            }
        }
    };
}

/// Not to be confused with [`mxp::SendTo`](crate::mxp::SendTo).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum, Deserialize, Serialize)]
pub enum SendTo {
    World,
    //WorldDelay,
    //WorldImmediate,
    Command,
    Output,
    Status,
    //NotepadNew,
    NotepadAppend,
    NotepadReplace,
    //Log,
    Speedwalk,
    //Execute,
    Variable,
    Script,
    //ScriptAfterOmit,
}

impl Default for SendTo {
    fn default() -> Self {
        Self::World
    }
}

impl SendTo {
    pub const fn ignore_empty(self) -> bool {
        !matches!(
            self,
            Self::NotepadAppend | Self::NotepadReplace | Self::Output | Self::Variable // | Self::NotepadNew
                                                                                       //| Self::Log
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(default)]
pub struct Sender {
    pub send_to: SendTo,
    pub label: String,
    pub script: String,
    pub group: String,
    pub variable: String,
    pub text: String,

    pub enabled: bool,
    pub one_shot: bool,
    pub temporary: bool,
    pub omit_from_output: bool,
    pub omit_from_log: bool,
}

impl Default for Sender {
    fn default() -> Self {
        Self::new()
    }
}

impl Sender {
    pub const fn new() -> Self {
        Self {
            text: String::new(),
            send_to: SendTo::World,
            label: String::new(),
            script: String::new(),
            group: String::new(),
            variable: String::new(),
            enabled: true,
            one_shot: false,
            temporary: false,
            omit_from_output: false,
            omit_from_log: false,
        }
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Reaction {
    // Note: this is at the top for Ord-deriving purposes.
    pub sequence: i16,
    pub pattern: String,
    pub send: Sender,

    pub ignore_case: bool,
    pub keep_evaluating: bool,
    pub is_regex: bool,
    pub expand_variables: bool,
    pub repeat: bool,

    pub regex: Regex,
}

impl_deref!(Reaction, Sender, send);
impl_asref!(Reaction, Sender);

impl Default for Reaction {
    fn default() -> Self {
        Self {
            sequence: config::DEFAULT_SEQUENCE,
            pattern: String::new(),
            send: Sender::default(),
            ignore_case: false,
            keep_evaluating: false,
            is_regex: false,
            expand_variables: false,
            repeat: false,
            regex: Regex::new("^$").unwrap(),
        }
    }
}

impl Reaction {
    pub fn make_regex(pattern: &str, is_regex: bool) -> Result<Regex, fancy_regex::Error> {
        #[rustfmt::skip]
        fn is_special(c: char) -> bool {
            matches!(c, '\\'|'.'|'+'|'*'|'?'|'('|')'|'|'|'['|']'|'{'|'}'|'^'|'$'|'#')
        }
        if is_regex {
            Regex::new(pattern)
        } else {
            let extra_len = pattern.bytes().filter(|&b| is_special(b as char)).count();
            let mut buf = String::with_capacity(pattern.len() + extra_len + 2);
            buf.push('^');
            for c in pattern.chars() {
                if c == '*' {
                    buf.push('.');
                } else if is_special(c) {
                    buf.push('\\');
                }
                buf.push(c);
            }
            buf.push('$');
            Regex::new(&buf)
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Alias {
    // Note: This is at the top for Ord-deriving purposes.
    pub reaction: Reaction,
    pub echo_alias: bool,
    pub menu: bool,
    pub omit_from_command_history: bool,
}

impl_deref!(Alias, Reaction, reaction);
impl_asref!(Alias, Reaction);
impl_asref!(Alias, Sender);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Trigger {
    // Note: this is at the top for Ord-deriving purposes.
    pub reaction: Reaction,
    pub change_foreground: bool,
    pub foreground: RColor,
    pub change_background: bool,
    pub background: RColor,
    pub make_bold: bool,
    pub make_italic: bool,
    pub make_underline: bool,
    pub sound: Option<PathBuf>,
    pub sound_if_inactive: bool,
    pub lowercase_wildcard: bool,
    pub multi_line: bool,
    pub lines_to_match: u8,
}

impl_deref!(Trigger, Reaction, reaction);
impl_asref!(Trigger, Reaction);
impl_asref!(Trigger, Sender);

impl Default for Trigger {
    fn default() -> Self {
        Self::new()
    }
}

impl Trigger {
    pub fn new() -> Self {
        Self {
            reaction: Reaction::default(),
            change_foreground: false,
            foreground: RColor::from(ColorRole::Text),
            change_background: false,
            background: RColor::from(ColorRole::Base),
            make_bold: false,
            make_italic: false,
            make_underline: false,
            sound: None,
            sound_if_inactive: false,
            multi_line: false,
            lines_to_match: 0,
            lowercase_wildcard: false,
        }
    }
}

// XML SERDE

mod sendto_serde {
    use serde::de::{Error as _, Unexpected};
    use serde::{Deserializer, Serializer};

    use super::*;

    pub fn serialize<S: Serializer>(value: &SendTo, serializer: S) -> Result<S::Ok, S::Error> {
        match *value {
            SendTo::World => 0u8,
            SendTo::Command => 1,
            SendTo::Output => 2,
            SendTo::Status => 3,
            // SendTo::NotepadNew => 4,
            SendTo::NotepadAppend => 5,
            // SendTo::Log => 6,
            SendTo::NotepadReplace => 7,
            //SendTo::WorldDelay => 8,
            SendTo::Variable => 9,
            //SendTo::Execute => 10,
            SendTo::Speedwalk => 11,
            SendTo::Script => 12,
            //SendTo::WorldImmediate => 13,
            //SendTo::ScriptAfterOmit => 14,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<SendTo, D::Error> {
        let pos = <u8>::deserialize(deserializer)?;
        match pos {
            0 => Ok(SendTo::World),
            1 => Ok(SendTo::Command),
            2 => Ok(SendTo::Output),
            3 => Ok(SendTo::Status),
            4 => Ok(SendTo::NotepadAppend), // NotepadNew
            5 => Ok(SendTo::NotepadAppend),
            6 => Ok(SendTo::Variable), // Log
            7 => Ok(SendTo::NotepadReplace),
            8 => Ok(SendTo::World), // WorldDelay
            9 => Ok(SendTo::Variable),
            10 => Ok(SendTo::World), // Execute
            11 => Ok(SendTo::Speedwalk),
            12 => Ok(SendTo::Script),
            13 => Ok(SendTo::World),  // WorldImmediate
            14 => Ok(SendTo::Script), // ScriptAfterOmit
            _ => Err(D::Error::invalid_value(
                Unexpected::Unsigned(pos as u64),
                &"integer between 0 and 14",
            )),
        }
    }
}

pub trait InPlace<T> {
    fn in_place(self) -> T;
}
impl<T: Sized + Copy> InPlace<T> for T {
    fn in_place(self) -> T {
        self
    }
}
impl<'a> InPlace<Cow<'a, str>> for &'a String {
    fn in_place(self) -> Cow<'a, str> {
        Cow::Borrowed(self.as_str())
    }
}
impl InPlace<String> for Cow<'_, str> {
    fn in_place(self) -> String {
        self.into_owned()
    }
}
impl InPlace<String> for Vec<Cow<'_, str>> {
    fn in_place(self) -> String {
        let mut lines = self.into_iter();
        let mut s = match lines.next() {
            Some(s) => s.into_owned(),
            None => return String::new(),
        };
        for line in lines {
            s.push('\n');
            s.push_str(&line);
        }
        s
    }
}
impl<'a> InPlace<Vec<Cow<'a, str>>> for &'a String {
    fn in_place(self) -> Vec<Cow<'a, str>> {
        vec![Cow::Borrowed(self)]
    }
}

macro_rules! in_place {
    ($from:expr, $name:ident {
        $($field:ident $(: $val:expr)?,)* $(,)?
        $(..$flat:ident),* $(,)*
    }) => {
        $name {
            $($field $(: $val)?,)*
            $($flat: $from.$flat.in_place()),*
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(default)]
pub struct TimerXml<'a> {
    #[serde(borrow, default, rename = "name")]
    label: Cow<'a, str>,
    #[serde(borrow, default)]
    script: Cow<'a, str>,
    enabled: bool,
    at_time: bool,
    #[serde(borrow, default)]
    group: Cow<'a, str>,
    #[serde(borrow, default)]
    variable: Cow<'a, str>,
    one_shot: bool,
    active_closed: bool,
    #[serde(with = "sendto_serde")]
    send_to: SendTo,
    omit_from_output: bool,
    omit_from_log: bool,
    hour: u64,
    minute: u64,
    second: f64,
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
            Event::Interval(Duration::from_hms(value.hour, value.minute, value.second))
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
            Event::Interval(every) => (false, every.hour(), every.minute(), every.second()),
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

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(default = "AliasXml::template")]
pub struct AliasXml<'a> {
    #[serde(borrow, default, rename = "name")]
    label: Cow<'a, str>,
    #[serde(borrow, default)]
    script: Cow<'a, str>,
    #[serde(borrow, default, rename = "match")]
    pattern: Cow<'a, str>,
    enabled: bool,
    echo_alias: bool,
    expand_variables: bool,
    #[serde(borrow, default)]
    group: Cow<'a, str>,
    #[serde(borrow, default)]
    variable: Cow<'a, str>,
    omit_from_command_history: bool,
    omit_from_log: bool,
    #[serde(rename = "regexp")]
    is_regex: bool,
    #[serde(with = "sendto_serde")]
    send_to: SendTo,
    omit_from_output: bool,
    one_shot: bool,
    menu: bool,
    ignore_case: bool,
    keep_evaluating: bool,
    sequence: i16,
    temporary: bool,
    #[serde(borrow, default, rename = "send")]
    text: Vec<Cow<'a, str>>,
}
impl AliasXml<'_> {
    fn template() -> Self {
        Self {
            enabled: true,
            sequence: config::DEFAULT_SEQUENCE,
            ..Default::default()
        }
    }
}
impl TryFrom<AliasXml<'_>> for Alias {
    type Error = fancy_regex::Error;

    fn try_from(value: AliasXml) -> Result<Self, Self::Error> {
        let regex = Reaction::make_regex(&value.pattern, value.is_regex)?;
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
        let reaction = in_place!(
            value,
            Reaction {
                send,
                regex,
                repeat: false,
                ..sequence,
                ..pattern,
                ..is_regex,
                ..ignore_case,
                ..keep_evaluating,
                ..expand_variables,
            }
        );
        Ok(in_place!(
            value,
            Self {
                reaction,
                ..echo_alias,
                ..menu,
                ..omit_from_command_history,
            }
        ))
    }
}
impl<'a> From<&'a Alias> for AliasXml<'a> {
    fn from(value: &'a Alias) -> Self {
        in_place!(
            value,
            Self {
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
                ..sequence,
                ..pattern,
                ..is_regex,
                ..ignore_case,
                ..keep_evaluating,
                ..expand_variables,
                ..echo_alias,
                ..menu,
                ..omit_from_command_history,
            }
        )
    }
}

#[repr(u8)]
enum Change {
    Both,
    Fg,
    Bg,
}

impl Change {
    const fn member(self, set: Option<u8>) -> bool {
        match set {
            None => false,
            Some(0) => true,
            Some(x) => x == self as u8,
        }
    }

    const fn insert_into(self, set: Option<u8>) -> Option<u8> {
        match set {
            None => Some(self as u8),
            Some(i) if i == self as u8 => Some(i),
            _ => Some(Self::Both as u8),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(default = "TriggerXml::template")]
pub struct TriggerXml<'a> {
    /// See [`Change`].
    colour_change_type: Option<u8>,
    enabled: bool,
    expand_variables: bool,
    #[serde(borrow, default)]
    group: Cow<'a, str>,
    ignore_case: bool,
    lines_to_match: u8,
    keep_evaluating: bool,
    make_bold: bool,
    make_italic: bool,
    make_underline: bool,
    #[serde(borrow, default, rename = "match")]
    pattern: Cow<'a, str>,
    multi_line: bool,
    #[serde(borrow, default, rename = "name")]
    label: Cow<'a, str>,
    one_shot: bool,
    omit_from_log: bool,
    omit_from_output: bool,
    #[serde(rename = "regexp")]
    is_regex: bool,
    repeat: bool,
    #[serde(borrow, default)]
    script: Cow<'a, str>,
    #[serde(with = "sendto_serde")]
    send_to: SendTo,
    sequence: i16,
    #[serde(borrow, default)]
    sound: Option<Cow<'a, str>>,
    sound_if_inactive: bool,
    lowercase_wildcard: bool,
    temporary: bool,
    #[serde(borrow, default)]
    variable: Cow<'a, str>,
    other_text_colour: Option<String>,
    other_back_colour: Option<String>,
    #[serde(borrow, default, rename = "send")]
    text: Vec<Cow<'a, str>>,
}
impl TriggerXml<'_> {
    fn template() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }
    fn try_change_color(&self, change: Change, color: &Option<String>) -> Option<RColor> {
        if change.member(self.colour_change_type) {
            RColor::named(color.as_ref()?.as_str())
        } else {
            None
        }
    }

    fn change_color(&self, not: Change, color: &Option<String>, def: ColorRole) -> (bool, RColor) {
        match self.try_change_color(not, color) {
            Some(color) => (true, color),
            None => (false, RColor::from(def)),
        }
    }
}
impl TryFrom<TriggerXml<'_>> for Trigger {
    type Error = fancy_regex::Error;

    fn try_from(value: TriggerXml) -> Result<Self, Self::Error> {
        let regex = Reaction::make_regex(&value.pattern, value.is_regex)?;
        let (change_foreground, foreground) =
            value.change_color(Change::Fg, &value.other_text_colour, ColorRole::Text);
        let (change_background, background) =
            value.change_color(Change::Bg, &value.other_back_colour, ColorRole::Base);
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
        let reaction = in_place!(
            value,
            Reaction {
                send,
                regex,
                ..sequence,
                ..pattern,
                ..is_regex,
                ..ignore_case,
                ..keep_evaluating,
                ..expand_variables,
                ..repeat,
            }
        );
        Ok(in_place!(
            value,
            Self {
                reaction,
                change_foreground,
                foreground,
                change_background,
                background,
                sound: value.sound.map(|s| PathBuf::from(s.as_ref())),
                ..make_bold,
                ..make_italic,
                ..make_underline,
                ..sound_if_inactive,
                ..lowercase_wildcard,
                ..multi_line,
                ..lines_to_match,
            }
        ))
    }
}
impl<'a> From<&'a Trigger> for TriggerXml<'a> {
    fn from(value: &'a Trigger) -> Self {
        let mut colour_change_type = None;
        let other_text_colour = if value.change_foreground {
            colour_change_type = Change::Fg.insert_into(colour_change_type);
            Some(value.foreground.to_string())
        } else {
            None
        };
        let other_back_colour = if value.change_background {
            colour_change_type = Change::Bg.insert_into(colour_change_type);
            Some(value.background.to_string())
        } else {
            None
        };
        in_place!(
            value,
            Self {
                sound: value.sound.as_ref().and_then(|x| x.as_os_str().to_str()).map(Cow::Borrowed),
                colour_change_type,
                other_text_colour,
                other_back_colour,
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
                ..sequence,
                ..pattern,
                ..is_regex,
                ..ignore_case,
                ..keep_evaluating,
                ..expand_variables,
                ..make_bold,
                ..make_italic,
                ..make_underline,
                ..sound_if_inactive,
                ..repeat,
                ..lowercase_wildcard,
                ..multi_line,
                ..lines_to_match,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_regex() {
        let equivalents = &[
            ("", "^$"),
            ("abc", "^abc$"),
            ("*a", "^.*a$"),
            ("^?{a*bc**#d", r"^\^\?\{a.*bc.*.*\#d$"),
        ];
        let non_regex: Vec<_> = equivalents
            .iter()
            .map(|(pat, _)| Reaction::make_regex(pat, false).unwrap())
            .collect();
        let regex: Vec<_> = equivalents
            .iter()
            .map(|(_, pat)| Reaction::make_regex(pat, true).unwrap())
            .collect();
        assert_eq!(non_regex, regex);
    }
}
