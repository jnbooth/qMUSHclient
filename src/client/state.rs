use std::time::{Duration, Instant};

use enumeration::Enum;

use super::mxp;

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
        matches!(
            self,
            Self::MxpElement
                | Self::MxpComment
                | Self::MxpQuote
                | Self::MxpEntity
                | Self::MxpRoomName
                | Self::MxpRoomDescription
                | Self::MxpRoomExits
                | Self::MxpWelcome
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Mccp {
    V1,
    V2,
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

    pub disconnect_ok: bool,
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
