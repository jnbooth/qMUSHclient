use std::str;

use uuid::Uuid;

use crate::enums::Enum;

pub type PluginId = Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Callback {
    // install / remove
    Install,
    Close,
    ListChanged,

    // connect / disconnect
    Connect,
    Disconnect,

    // saving
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
