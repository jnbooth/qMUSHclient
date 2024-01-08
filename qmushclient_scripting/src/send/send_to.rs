use enumeration::Enum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum, Deserialize, Serialize)]
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
    Speedwalk,
    Execute,
    Variable,
    Script,
    ScriptAfterOmit,
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
            Self::NotepadAppend
                | Self::NotepadReplace
                | Self::Output
                | Self::Variable
                | Self::NotepadNew
                | Self::Log
        )
    }
}

pub mod sendto_serde {
    use serde::de::{Error as _, Unexpected};
    use serde::{Deserializer, Serializer};

    use super::*;

    pub fn serialize<S: Serializer>(value: &SendTo, serializer: S) -> Result<S::Ok, S::Error> {
        match *value {
            SendTo::World => 0u8,
            SendTo::Command => 1,
            SendTo::Output => 2,
            SendTo::Status => 3,
            SendTo::NotepadNew => 4,
            SendTo::NotepadAppend => 5,
            SendTo::Log => 6,
            SendTo::NotepadReplace => 7,
            SendTo::WorldDelay => 8,
            SendTo::Variable => 9,
            SendTo::Execute => 10,
            SendTo::Speedwalk => 11,
            SendTo::Script => 12,
            SendTo::WorldImmediate => 13,
            SendTo::ScriptAfterOmit => 14,
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
            4 => Ok(SendTo::NotepadNew),
            5 => Ok(SendTo::NotepadAppend),
            6 => Ok(SendTo::Log),
            7 => Ok(SendTo::NotepadReplace),
            8 => Ok(SendTo::WorldDelay),
            9 => Ok(SendTo::Variable),
            10 => Ok(SendTo::Execute),
            11 => Ok(SendTo::Speedwalk),
            12 => Ok(SendTo::Script),
            13 => Ok(SendTo::WorldImmediate),
            14 => Ok(SendTo::ScriptAfterOmit),
            _ => Err(D::Error::invalid_value(
                Unexpected::Unsigned(pos as u64),
                &"integer between 0 and 14",
            )),
        }
    }
}
