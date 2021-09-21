use std::str;

use mlua::{Lua, Value};

use crate::enums::Enum;
use crate::script::ScriptArg;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Callback {
    // install / remove
    Install,
    Open,
    Close,
    ListChanged,

    // world state
    Connect,
    Disconnect,
    
    SaveState,
    WorldSave,

    // enable / disable
    Enable,
    Disable,

    // the focus
    GetFocus,
    LoseFocus,

    // capture stuff
    Trace,
    PacketDebug,
    Broadcast,
    Screendraw,
    SelectionChanged,

    // sounds
    PlaySound,

    // stuff received/sent
    Send,
    Sent,
    PartialLine,
    LineReceived,
    PacketReceived,

    // telnet negotiation
    TelnetOption,
    TelnetRequest,
    TelnetSubnegotiation,
    IacGa,

    // commands
    Command,
    CommandEntered,
    CommandChanged,
    TabComplete,

    // resizing, ticking, moving, rhythm
    WorldOutputResized,
    Tick,
    MouseMoved,

    // MXP stuff
    MxpStart,
    MxpStop,
    MxpOpenTag,
    MxpCloseTag,
    MxpSetVariable,
    MxpSetEntity,
    MxpError,

    // chat stuff
    ChatAccept,
    ChatMessage,
    ChatMessageOut,
    ChatDisplay,
    ChatNewUser,
    ChatUserDisconnect,

    // drawing
    DrawOutputWindow,
}

impl ScriptArg for Callback {
    fn to_arg(self, lua: &Lua) -> mlua::Result<Value> {
        self.to_str().to_arg(lua)
    }
}
