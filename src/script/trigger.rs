use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use qt_gui::q_palette::ColorRole;
use regex::{self, Regex};
use serde::{Deserialize, Serialize};

use super::{SendTo, Sender};
use crate::binding::RColor;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "AliasSerde", into = "AliasSerde")]
pub struct Alias {
    pub pattern: String,
    pub regex: Regex,
    pub send: Sender,
    pub sequence: i16,

    pub ignore_case: bool,
    pub keep_evaluating: bool,
    pub is_regex: bool,
    pub repeat: bool,
    pub expand: bool,
    pub multi_line: bool,
    pub lines_to_match: u8,
    pub lowercase_wildcard: bool,
}

impl Deref for Alias {
    type Target = Sender;

    fn deref(&self) -> &Self::Target {
        &self.send
    }
}

impl DerefMut for Alias {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.send
    }
}

impl PartialEq for Alias {
    fn eq(&self, other: &Self) -> bool {
        macro_rules! eq {
            ($field:ident) => {
                self.$field == other.$field
            };
        }

        eq!(sequence)
            && eq!(pattern)
            && eq!(lowercase_wildcard)
            && eq!(lines_to_match)
            && eq!(multi_line)
            && eq!(expand)
            && eq!(repeat)
            && eq!(is_regex)
            && eq!(keep_evaluating)
            && eq!(ignore_case)
            && eq!(send)
            && self.regex.as_str() == other.regex.as_str()
    }
}

impl Eq for Alias {}

impl PartialOrd for Alias {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sequence.partial_cmp(&other.sequence)
    }
}

impl Ord for Alias {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sequence.cmp(&other.sequence)
    }
}

impl Default for Alias {
    fn default() -> Self {
        Self::new()
    }
}

impl Alias {
    pub fn new() -> Self {
        Self {
            pattern: String::new(),
            regex: Regex::new("").unwrap(),
            send: Sender::new(),
            sequence: 5,

            ignore_case: false,
            keep_evaluating: false,
            is_regex: false,
            repeat: false,
            expand: false,
            multi_line: false,
            lines_to_match: 0,
            lowercase_wildcard: false,
        }
    }

    pub fn _set_pattern(&mut self, pattern: String) -> Result<(), regex::Error> {
        self.regex = if self.is_regex {
            Regex::new(&pattern)
        } else {
            Regex::new(
                &pattern
                    .split('*')
                    .map(regex::escape)
                    .collect::<Vec<_>>()
                    .join(".*"),
            )
        }?;
        self.pattern = pattern;
        Ok(())
    }
}

fn default_fg() -> RColor {
    RColor::from(ColorRole::Text)
}
fn default_bg() -> RColor {
    RColor::from(ColorRole::Base)
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(from = "TriggerSerde", into = "TriggerSerde")]
pub struct Trigger {
    pub alias: Alias,
    pub change_foreground: bool,
    pub foreground: RColor,
    pub change_background: bool,
    pub background: RColor,
    pub make_bold: bool,
    pub make_italic: bool,
    pub make_underline: bool,
    pub sound: Option<PathBuf>,
    pub sound_if_inactive: bool,
}

impl PartialOrd for Trigger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.alias.sequence.partial_cmp(&other.alias.sequence)
    }
}

impl Ord for Trigger {
    fn cmp(&self, other: &Self) -> Ordering {
        self.alias.sequence.cmp(&other.alias.sequence)
    }
}

impl Deref for Trigger {
    type Target = Alias;

    fn deref(&self) -> &Self::Target {
        &self.alias
    }
}

impl DerefMut for Trigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.alias
    }
}

impl Default for Trigger {
    fn default() -> Self {
        Self::new()
    }
}

impl Trigger {
    pub fn new() -> Self {
        Self {
            alias: Alias::new(),
            change_foreground: false,
            foreground: default_fg(),
            change_background: false,
            background: default_bg(),
            make_bold: false,
            make_italic: false,
            make_underline: false,
            sound: None,
            sound_if_inactive: false,
        }
    }
}

sender_fields! {
    #[derive(Default, Deserialize, Serialize)]
    #[serde(default)]
    struct AliasSerde {
        #[serde(rename = "match")]
        pattern: String,
        #[serde(default, rename = "`regex")] // not allowed in .xml
        regex: String,
        sequence: i16,

        ignore_case: bool,
        keep_evaluating: bool,
        regexp: bool,
        repeat: bool,
        expand: bool,
        multi_line: bool,
        lines_to_match: u8,
        lowercase_wildcard: bool,
    }
}

impl From<AliasSerde> for Alias {
    fn from(value: AliasSerde) -> Self {
        Self {
            send: sender!(value, Sender {}),
            pattern: value.pattern,
            regex: Regex::new(&value.regex).expect("failed to compile regex"),
            sequence: value.sequence,
            ignore_case: value.ignore_case,
            keep_evaluating: value.keep_evaluating,
            is_regex: value.regexp,
            repeat: value.repeat,
            expand: value.expand,
            multi_line: value.multi_line,
            lines_to_match: value.lines_to_match,
            lowercase_wildcard: value.lowercase_wildcard,
        }
    }
}

impl From<Alias> for AliasSerde {
    fn from(value: Alias) -> Self {
        sender!(
            value.send,
            Self {
                pattern: value.pattern,
                regex: value.regex.as_str().to_owned(),
                sequence: value.sequence,
                ignore_case: value.ignore_case,
                keep_evaluating: value.keep_evaluating,
                regexp: value.is_regex,
                repeat: value.repeat,
                expand: value.expand,
                multi_line: value.multi_line,
                lines_to_match: value.lines_to_match,
                lowercase_wildcard: value.lowercase_wildcard,
            }
        )
    }
}

sender_fields! {
    #[derive(Default, Deserialize, Serialize)]
    #[serde(default)]
    struct TriggerSerde {
        #[serde(rename = "match")]
        pattern: String,
        #[serde(default, rename = "`regex")] // not allowed in .xml
        regex: String,
        sequence: i16,
        ignore_case: bool,
        keep_evaluating: bool,
        regexp: bool,
        repeat: bool,
        expand: bool,
        multi_line: bool,
        lines_to_match: u8,
        lowercase_wildcard: bool,

        change_foreground: bool,
        #[serde(default = "default_fg", rename = "`foreground")]
        foreground: RColor,
        change_background: bool,
        #[serde(default = "default_bg", rename = "`background")]
        background: RColor,
        make_bold: bool,
        make_italic: bool,
        make_underline: bool,
        sound: Option<PathBuf>,
        sound_if_inactive: bool,
    }
}

impl From<TriggerSerde> for Trigger {
    fn from(value: TriggerSerde) -> Self {
        let alias = Alias {
            send: sender!(value, Sender {}),
            pattern: value.pattern,
            regex: Regex::new(&value.regex).expect("failed to compile regex"),
            sequence: value.sequence,
            ignore_case: value.ignore_case,
            keep_evaluating: value.keep_evaluating,
            is_regex: value.regexp,
            repeat: value.repeat,
            expand: value.expand,
            multi_line: value.multi_line,
            lines_to_match: value.lines_to_match,
            lowercase_wildcard: value.lowercase_wildcard,
        };
        Self {
            alias,
            change_foreground: value.change_foreground,
            foreground: value.foreground,
            change_background: value.change_background,
            background: value.background,
            make_bold: value.make_bold,
            make_italic: value.make_italic,
            make_underline: value.make_underline,
            sound: value.sound,
            sound_if_inactive: value.sound_if_inactive,
        }
    }
}

impl From<Trigger> for TriggerSerde {
    fn from(value: Trigger) -> Self {
        sender!(
            value.alias.send,
            Self {
                pattern: value.alias.pattern,
                regex: value.alias.regex.as_str().to_owned(),
                sequence: value.alias.sequence,
                ignore_case: value.alias.ignore_case,
                keep_evaluating: value.alias.keep_evaluating,
                regexp: value.alias.is_regex,
                repeat: value.alias.repeat,
                expand: value.alias.expand,
                multi_line: value.alias.multi_line,
                lines_to_match: value.alias.lines_to_match,
                lowercase_wildcard: value.alias.lowercase_wildcard,

                change_foreground: value.change_foreground,
                foreground: value.foreground,
                change_background: value.change_background,
                background: value.background,
                make_bold: value.make_bold,
                make_italic: value.make_italic,
                make_underline: value.make_underline,
                sound: value.sound,
                sound_if_inactive: value.sound_if_inactive,
            }
        )
    }
}
