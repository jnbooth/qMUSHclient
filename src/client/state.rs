use super::mxp;
use crate::enums::Enum;
use crate::escape::telnet;
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum MessageFlag {
    Comment,
    UserInput,
    LogLine,
    Bookmark,
    Rule,
    Note,
    Fake,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Phase {
    /// Normal text
    Normal,
    /// Received an escape
    Esc,
    /// Processing an ANSI escape sequence
    DoingCode,
    /// Received TELNET IAC (interpret as command)
    Iac,
    /// Received TELNET WILL
    Will,
    /// Received TELNET WONT
    Wont,
    /// Received TELNET DO
    Do,
    /// Received TELNET DONT
    Dont,
    /// Received TELNET IAC SB
    Sb,
    /// Received TELNET IAC SB c (collecting data, awaiting IAC SE)
    Subnegotiation,
    /// Received TELNET IAC SB c <data> IAC (awaiting IAC or SE)
    SubnegotiationIac,
    /// Received TELNET IAC COMPRESS
    Compress,
    /// Received TELNET IAC COMPRESS WILL
    CompressWill,

    // see: https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
    /// Received ESC[38;
    Foreground256Start,
    /// Received ESC[38;5;
    Foreground256Finish,
    /// Received ESC[48;
    Background256Start,
    /// Received ESC[48;5;
    Background256Finish,

    // see: https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit
    /// Received ESC[38;2;
    Foreground24bFinish,
    /// Received ESC[38;2;r;
    Foreground24brFinish,
    /// Received ESC[38;2;r;g;
    Foreground24bgFinish,
    /// Received ESC[38;2;r;g;b;
    Foreground24bbFinish,
    /// Received ESC[48;2;
    Background24bFinish,
    /// Received ESC[48;2;r;
    Background24brFinish,
    /// Received ESC[48;2;r;g;
    Background24bgFinish,
    /// Received ESC[48;2;r;g;b
    Background24bbFinish,

    /// Received 110 xxxxx, 1110 xxxx, or 11110 xxx
    Utf8Character,

    // MXP modes
    /// Collecting element, eg. < xxxxx >. Starts on <, stops on >
    MxpElement,
    /// Collecting comment, eg. <!-- xxxxx -->. Starts on <!--, stops on -->
    MxpComment,
    /// Collecting quote inside element, eg. <color='red'>
    MxpQuote,
    /// Collecting entity, eg. &gt; . Starts on &, stops on ;
    MxpEntity,

    // mxp special collection modes following a special escape sequence
    /// Parsing the name of a room.
    MxpRoomName,
    /// Parsing the description of a room.
    MxpRoomDescription,
    /// Parsing an exit line for a room.
    MxpRoomExits,
    /// Text sent from the server at the beginning of a session.
    MxpWelcome,
}

impl Phase {
    pub const fn is_mxp(self) -> bool {
        match self {
            Self::MxpElement
            | Self::MxpComment
            | Self::MxpQuote
            | Self::MxpEntity
            | Self::MxpRoomName
            | Self::MxpRoomDescription
            | Self::MxpRoomExits
            | Self::MxpWelcome => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum ConnectPhase {
    /// 0: not connected and not attempting connection
    NotConnected,
    /// 1: finding address of MUD
    MudNameLookup,
    /// 2: finding address of proxy server
    ProxyNameLookup,
    /// 3: connecting to MUD (no proxy server)
    ConnectingToMud,
    /// 4: connecting to proxy server
    ConnectingToProxy,
    /// 5: sent SOCKS authentication method, awaiting confirmation
    AwaitingProxyResponse1,
    /// 6: sent SOCKS username/password, awaiting confirmation
    AwaitingProxyResponse2,
    /// 7: sent SOCKS connect details, awaiting confirmation
    AwaitingProxyResponse3,
    /// 8: connected, we can play now
    ConnectedToMud,
    /// 9: in process of disconnecting, don't attempt to reconnect
    Disconnecting,
}

impl Default for ConnectPhase {
    fn default() -> Self {
        Self::NotConnected
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Source {
    /// no particular reason, could be plugin saving
    Unknown,
    /// user typed something in the command area and pressed <Enter>
    Typing,
    /// user typed a macro  (eg. F2)
    Macro,
    /// user used the numeric keypad
    Keypad,
    /// user used an accelerator key
    Accelerator,
    /// item chosen from pop-up menu
    Menu,
    /// trigger fired
    Trigger,
    /// timer fired
    Timer,
    /// input arrived (eg. packet received)
    Server,
    /// some sort of world action (eg. world open, connect, got focus)
    World,
    /// executing Lua sandbox
    Sandbox,
    /// miniwindow hotspot callback
    Hotspot,
}

impl Default for Source {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Iac {
    Do,
    Dont,
    Will,
    Wont,
}
impl Iac {
    pub const fn opposite(self) -> Self {
        match self {
            Self::Do => Self::Dont,
            Self::Dont => Self::Do,
            Self::Will => Self::Wont,
            Self::Wont => Self::Will,
        }
    }
    pub const fn code(self) -> u8 {
        match self {
            Self::Do => telnet::DO,
            Self::Dont => telnet::DONT,
            Self::Will => telnet::WILL,
            Self::Wont => telnet::WONT,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Mccp {
    V1,
    V2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Capabilities {
    sent: [[bool; 256]; Iac::SIZE],
    got: [[bool; 256]; Iac::SIZE],
    total_got: [u64; Iac::SIZE],
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl Capabilities {
    pub const fn new() -> Self {
        Self {
            sent: [[false; 256]; Iac::SIZE],
            got: [[false; 256]; Iac::SIZE],
            total_got: [0; Iac::SIZE],
        }
    }
    pub fn send(&mut self, iac: Iac, c: u8) -> [u8; 3] {
        let i = c as usize;
        self.sent[iac.index()][i] = true;
        self.sent[iac.opposite().index()][i] = false;
        [telnet::IAC, iac.code(), c]
    }
    pub fn sent(&self, iac: Iac, c: u8) -> bool {
        self.sent[iac.index()][c as usize]
    }
    pub fn get(&mut self, iac: Iac, c: u8) {
        self.got[iac.index()][c as usize] = true;
        self.total_got[iac.index()] += 1;
    }
    pub fn got(&self, iac: Iac, c: u8) -> bool {
        self.got[iac.index()][c as usize]
    }
    pub fn total_got(&self, iac: Iac) -> u64 {
        self.total_got[iac.index()]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum ListMode {
    Ordered,
    Unordered,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Latest {
    pub connected: Instant,
    pub input: Instant,
}

impl Latest {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            connected: now,
            input: now,
        }
    }
}

impl Default for Latest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClientState {
    pub linecount: u64,
    pub current_action_source: Source,

    pub disconnect_ok: bool,
    pub connect_phase: ConnectPhase,
    pub total_connect_duration: Duration,

    pub mxp_active: bool,
    pub pueblo_active: bool,
    pub mxp_script: bool,
    pub pre_mode: bool,
    pub in_paragraph: bool,
    pub suppress_newline: bool,
    pub mxp_mode_default: mxp::Mode,
    pub mxp_mode: mxp::Mode,
    pub mxp_mode_previous: mxp::Mode,
    pub mxp_quote_terminator: Option<u8>,
    pub mxp_string: Vec<u8>,
    pub mxp_active_tags: Vec<mxp::Tag>,
    pub mxp_elements: mxp::ElementMap,
    pub mxp_entities: mxp::EntityMap,
    pub last_outstanding_tag_count: u64,
    pub list_mode: Option<ListMode>,
    pub list_index: u16,

    pub iac: Capabilities,
    pub last_line_with_iac_ga: u64,
    pub subnegotiation_type: u8,
    pub subnegotiation_data: Vec<u8>,
    pub ttype_sequence: u8,
    pub naws_wanted: bool,
    pub mccp_ver: Option<Mccp>,
    pub ansi_code: u8,
    pub ansi_red: u8,
    pub ansi_green: u8,
    pub ansi_blue: u8,

    pub supports_mccp_2: bool,
    pub no_echo: bool,

    pub utf8_sequence: Vec<u8>,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            utf8_sequence: Vec::with_capacity(4),
            ..Default::default()
        }
    }
}
