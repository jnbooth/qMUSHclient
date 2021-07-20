use std::fmt::{self, Formatter};

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::enums::Enum;
use crate::script::plugin::PluginIndex;

#[macro_export]
macro_rules! sender {
    ($send:expr, $name:ident { $( $param:ident $(: $val:expr)? ),* $(,)* } ) => (
        $name {
            text: $send.text,
            send_to: $send.send_to,
            name: $send.name,
            script: $send.script,
            group: $send.group,
            variable: $send.variable,
            enabled: $send.enabled,
            one_shot: $send.one_shot,
            temporary: $send.temporary,
            omit_from_output: $send.omit_from_output,
            omit_from_log: $send.omit_from_log,
            $($param $(: $val)?,)*
        }
    );
}

#[macro_export]
macro_rules! sender_fields {
    (
        $(#[derive($($derive:ident),*)])?
        $(#[serde($($souter:tt)+)])?
        $pub:vis struct $name:ident {
            $(
                $(#[serde($($sinner:tt)+)])?
                $pubinner:vis $param:ident : $t:ty
            ),* $(,)*
        }
    ) => {
        $(#[derive($($derive,)*)])?
        $(#[serde($($souter)+)])?
        $pub struct $name {
            $pub text: String,
            $pub send_to: SendTo,
            $pub name: String,
            $pub script: String,
            $pub group: String,
            $pub variable: String,

            $pub enabled: bool,
            $pub one_shot: bool,
            $pub temporary: bool,
            $pub omit_from_output: bool,
            $pub omit_from_log: bool,
            $(
                $(#[serde($($sinner)+)])?
                $pubinner $param: $t,
            )*
        }
    }
}

sender_fields! {
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    #[serde(default)]
    pub struct Sender {}
}

impl Default for Sender {
    fn default() -> Self {
        Self::new()
    }
}

impl Sender {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            send_to: SendTo::World,
            name: String::new(),
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

/// Not to be confused with [`mxp::SendTo`](crate::mxp::SendTo).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum SendTo {
    World,
    WorldDelay,
    WorldImmediate,
    Command,
    Output,
    Status,
    NotepadNew,
    NotepadAppend,
    NotepadReplace,
    Log,
    Execute,
    Speedwalk,
    Variable,
    Script,
    ScriptAfterOmit,
}

impl Default for SendTo {
    fn default() -> Self {
        Self::World
    }
}

impl Serialize for SendTo {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SendTo {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SendToVisitor;

        impl<'de> Visitor<'de> for SendToVisitor {
            type Value = SendTo;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a SendTo variant")
            }

            #[inline]
            #[rustfmt::skip]
            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(match s {
                    "0"  | "World" => SendTo::World,
                    "1"  | "Command" => SendTo::Command,
                    "2"  | "Output" => SendTo::Output,
                    "3"  | "Status" => SendTo::Status,
                    "4"  | "NotepadNew" => SendTo::NotepadNew,
                    "5"  | "NotepadAppend" => SendTo::NotepadAppend,
                    "6"  | "Log" => SendTo::Log,
                    "7"  | "NotepadReplace" => SendTo::NotepadReplace,
                    "8"  | "WorldDelay" => SendTo::WorldDelay,
                    "9"  | "Variable" => SendTo::Variable,
                    "10" | "Execute" => SendTo::Execute,
                    "11" | "Speedwalk" => SendTo::Speedwalk,
                    "12" | "Script" => SendTo::Script,
                    "13" | "WorldImmediate" => SendTo::WorldImmediate,
                    _ => return Err(E::unknown_variant(s, &["World","Command","Output","Status","NotepadNew","NotepadAppend","Log", "NotepadReplace", "WorldDelay", "Variable", "Execute", "Speedwalk", "Script", "WorldImmediate"])),
                })
            }
        }

        deserializer.deserialize_str(SendToVisitor)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SendRequest<'a> {
    pub send_to: SendTo,
    pub text: &'a str,
    pub plugin: PluginIndex,
}
