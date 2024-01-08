use std::borrow::Cow;
use std::path::PathBuf;

use qt::gui::{ColorRole, QColor};
use serde::{Deserialize, Serialize};

use super::reaction::Reaction;
use super::send_to::{sendto_serde, SendTo};
use super::sender::Sender;
use crate::in_place::InPlace;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Trigger {
    // Note: this is at the top for Ord-deriving purposes.
    pub reaction: Reaction,
    pub change_foreground: bool,
    pub foreground: QColor,
    pub change_background: bool,
    pub background: QColor,
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
            foreground: QColor::from(ColorRole::Text),
            change_background: false,
            background: QColor::from(ColorRole::Base),
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
    #[serde(rename = "@color_change_type", skip_serializing_if = "Option::is_none")]
    colour_change_type: Option<u8>,
    #[serde(rename = "@enabled")]
    enabled: bool,
    #[serde(rename = "@expand_variables")]
    expand_variables: bool,
    #[serde(
        borrow,
        default,
        rename = "@group",
        skip_serializing_if = "str::is_empty"
    )]
    group: Cow<'a, str>,
    #[serde(rename = "@ignore_case")]
    ignore_case: bool,
    #[serde(rename = "@lines_to_match")]
    lines_to_match: u8,
    #[serde(rename = "@keep_evaluating")]
    keep_evaluating: bool,
    #[serde(rename = "@make_bold")]
    make_bold: bool,
    #[serde(rename = "@make_italic")]
    make_italic: bool,
    #[serde(rename = "@make_underline")]
    make_underline: bool,
    #[serde(
        borrow,
        default,
        rename = "@match",
        skip_serializing_if = "str::is_empty"
    )]
    pattern: Cow<'a, str>,
    #[serde(rename = "@multi_line")]
    multi_line: bool,
    #[serde(
        borrow,
        default,
        rename = "@name",
        skip_serializing_if = "str::is_empty"
    )]
    label: Cow<'a, str>,
    #[serde(rename = "@one_shot")]
    one_shot: bool,
    #[serde(rename = "@omit_from_log")]
    omit_from_log: bool,
    #[serde(rename = "@omit_from_output")]
    omit_from_output: bool,
    #[serde(rename = "@regexp")]
    is_regex: bool,
    #[serde(rename = "@repeat")]
    repeat: bool,
    #[serde(
        borrow,
        default,
        rename = "@script",
        skip_serializing_if = "str::is_empty"
    )]
    script: Cow<'a, str>,
    #[serde(with = "sendto_serde", rename = "@send_to")]
    send_to: SendTo,
    #[serde(rename = "@sequence")]
    sequence: i16,
    #[serde(
        borrow,
        default,
        rename = "@sound",
        skip_serializing_if = "Option::is_none"
    )]
    sound: Option<Cow<'a, str>>,
    #[serde(rename = "@sound_if_inactive")]
    sound_if_inactive: bool,
    #[serde(rename = "@lowercase_wildcard")]
    lowercase_wildcard: bool,
    #[serde(rename = "@temporary")]
    temporary: bool,
    #[serde(
        borrow,
        default,
        rename = "@variable",
        skip_serializing_if = "str::is_empty"
    )]
    variable: Cow<'a, str>,
    #[serde(rename = "@other_text_colour", skip_serializing_if = "Option::is_none")]
    other_text_colour: Option<String>,
    #[serde(rename = "@other_back_colour", skip_serializing_if = "Option::is_none")]
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
    fn try_change_color(&self, change: Change, color: &Option<String>) -> Option<QColor> {
        if change.member(self.colour_change_type) {
            QColor::named(color.as_ref()?.as_str())
        } else {
            None
        }
    }

    fn change_color(&self, not: Change, color: &Option<String>, def: ColorRole) -> (bool, QColor) {
        match self.try_change_color(not, color) {
            Some(color) => (true, color),
            None => (false, QColor::from(def)),
        }
    }
}
impl TryFrom<TriggerXml<'_>> for Trigger {
    type Error = crate::regex::RegexError;

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
