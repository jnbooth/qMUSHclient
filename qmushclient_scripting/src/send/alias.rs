use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::reaction::Reaction;
use super::send_to::{sendto_serde, SendTo};
use super::sender::Sender;
use crate::constants::DEFAULT_SEQUENCE;
use crate::in_place::InPlace;

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

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(default = "AliasXml::template")]
pub struct AliasXml<'a> {
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
    #[serde(
        borrow,
        default,
        rename = "@match",
        skip_serializing_if = "str::is_empty"
    )]
    pattern: Cow<'a, str>,
    #[serde(rename = "@enabled")]
    enabled: bool,
    #[serde(rename = "@echo_alias")]
    echo_alias: bool,
    #[serde(rename = "@expand_variables")]
    expand_variables: bool,
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
    #[serde(rename = "@omit_from_command_history")]
    omit_from_command_history: bool,
    #[serde(rename = "@omit_from_log")]
    omit_from_log: bool,
    #[serde(rename = "@regexp")]
    is_regex: bool,
    #[serde(with = "sendto_serde", rename = "@send_to")]
    send_to: SendTo,
    #[serde(rename = "@omit_from_output")]
    omit_from_output: bool,
    #[serde(rename = "@one_shot")]
    one_shot: bool,
    #[serde(rename = "@menu")]
    menu: bool,
    #[serde(rename = "@ignore_case")]
    ignore_case: bool,
    #[serde(rename = "@keep_evaluating")]
    keep_evaluating: bool,
    #[serde(rename = "@sequence")]
    sequence: i16,
    #[serde(rename = "@temporary")]
    temporary: bool,
    #[serde(borrow, default, rename = "send")]
    text: Vec<Cow<'a, str>>,
}
impl AliasXml<'_> {
    fn template() -> Self {
        Self {
            enabled: true,
            sequence: DEFAULT_SEQUENCE,
            ..Default::default()
        }
    }
}
impl TryFrom<AliasXml<'_>> for Alias {
    type Error = crate::regex::RegexError;

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
