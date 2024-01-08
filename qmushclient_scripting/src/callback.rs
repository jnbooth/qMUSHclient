use std::str;

use enumeration::Enum;
use mlua::{Lua, Value};

use crate::convert::ScriptArg;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Callback {
    Install,
    Open,
    Close,
    ListChanged,

    Connect,
    Disconnect,

    SaveState,
    WorldSave,

    Enable,
    Disable,

    GetFocus,
    LoseFocus,

    Trace,
    PacketDebug,
    Broadcast,
    Screendraw,
    SelectionChanged,

    PlaySound,

    Send,
    Sent,
    PartialLine,
    LineReceived,
    PacketReceived,

    TelnetOption,
    TelnetRequest,
    TelnetSubnegotiation,
    IacGa,

    Command,
    CommandEntered,
    CommandChanged,
    TabComplete,

    WorldOutputResized,
    Tick,
    MouseMoved,

    MxpStart,
    MxpStop,
    MxpOpenTag,
    MxpCloseTag,
    MxpSetVariable,
    MxpSetEntity,
    MxpError,

    ChatAccept,
    ChatMessage,
    ChatMessageOut,
    ChatDisplay,
    ChatNewUser,
    ChatUserDisconnect,

    DrawOutputWindow,
}

impl ScriptArg for Callback {
    fn to_arg(self, lua: &Lua) -> mlua::Result<Value> {
        self.to_str().to_arg(lua)
    }
}
