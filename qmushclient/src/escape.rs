#![allow(dead_code)]

pub mod telnet {
    pub const ESC: u8 = 0x1B;
    /// IAC WILL END-OF-RECORD.
    ///
    /// Specified in [RFC 885](https://datatracker.ietf.org/doc/html/rfc885):
    /// > The sender of this command requests permission to begin transmission of the Telnet
    /// > END-OF-RECORD (EOR) code when transmitting data characters, or the sender of this command
    /// > confirms it will now begin transmission of EORs with transmitted data characters.
    pub const WILL_EOR: u8 = 0x19;
    /// IAC END-OF-RECORD.
    ///
    /// Specified in [RFC 885](https://datatracker.ietf.org/doc/html/rfc885):
    /// > When the END-OF-RECORD option is in effect on the connection between a sender of data and
    /// > the receiver of the data, the sender transmits EORs.
    /// >
    /// > When the END-OF-RECORD option is not in effect, the IAC EOR command should be treated as a
    /// NOP if received, although IAC EOR should not normally be sent in this mode.
    /// >
    /// > As the EOR code indicates the end of an effective data unit, Telnet should attempt to send
    /// > the data up to and including the EOR code together to promote communication efficiency.
    /// see RFC 885
    pub const EOR: u8 = 0xEF;

    /// end of subnegotiation
    pub const SE: u8 = 0xF0;
    /// no operation
    pub const NOP: u8 = 0xF1;
    /// DataMark', see RFC 854
    pub const DM: u8 = 0xF2;
    /// Break
    pub const BRK: u8 = 0xF3;
    /// Interrupt Process
    pub const IP: u8 = 0xF4;
    /// Abort Output
    pub const AO: u8 = 0xF5;
    /// Are You There
    pub const AYT: u8 = 0xF6;
    /// Erase Character
    pub const EC: u8 = 0xF7;
    /// Erase Line
    pub const EL: u8 = 0xF8;
    /// Go Ahead
    pub const GA: u8 = 0xF9;
    /// subnegotiation
    pub const SB: u8 = 0xFA;
    pub const WILL: u8 = 0xFB;
    pub const WONT: u8 = 0xFC;
    pub const DO: u8 = 0xFD;
    pub const DONT: u8 = 0xFE;
    pub const IAC: u8 = 0xFF;

    // Capability escape sequences
    pub const ECHO: u8 = 0x01;
    /// Negotiate About Window Size
    pub const NAWS: u8 = 0x1F;
    /// Negotiate About Character Set
    pub const CHARSET: u8 = 0x2A;
    /// want to know terminal type
    pub const TERMINAL_TYPE: u8 = 0x18;
    /// telnet negotiation code for starting compression v1
    pub const COMPRESS: u8 = 0x55;
    /// telnet negotiation code for starting compression v2
    pub const COMPRESS2: u8 = 0x56;
    /// telnet negotiation code MUD-specific negotiations
    pub const MUD_SPECIFIC: u8 = 0x66;
    /// suppress go-ahead
    pub const SGA: u8 = 0x03;
    /// telnet negotiation code for MUD Sound Protocol (MSP)
    pub const MSP: u8 = 0x5A;
    /// telnet negotiation code for MUD Extension Protocol (MXP)
    pub const MXP: u8 = 0x5B;
    /// http://zmp.sourcemud.org/spec.shtml
    pub const ZMP: u8 = 0x5D;
    /// http://www.ironrealms.com/rapture/manual/files/FeatATCP-txt.html
    pub const ATCP: u8 = 0xC8;

    // Subnegotiation escape sequences
    pub const TTYPE_IS: u8 = 0x00;
    pub const TTYPE_SEND: u8 = 0x01;
    pub const ACCEPT: u8 = 0x02;
    pub const REJECT: u8 = 0x03;

    macro_rules! ttype {
        ($($b:literal),*) => (&[IAC, SB, TERMINAL_TYPE, TTYPE_IS, $($b,)* IAC, SE])
    }
    pub const TTYPE_ANSI: &[u8] = ttype!(b'A', b'N', b'S', b'I');
    pub const TTYPE_XTERM: &[u8] = ttype!(b'M', b'T', b'T', b'S', b' ', b'9');
    pub const TTYPE_UTF8: &[u8] = ttype!(b'M', b'T', b'T', b'S', b' ', b'1', b'3');

    macro_rules! charset {
        ($($b:literal),*) => (&[IAC, SB, CHARSET, ACCEPT, $($b,)* IAC, SE])
    }
    const CHARSET_UTF8: &[u8] = charset!(b'U', b'T', b'F', b'-', b'8');
    const CHARSET_US_ASCII: &[u8] = charset!(b'U', b'S', b'-', b'A', b'S', b'C', b'I', b'I');
    const REJECT_SUBNEGOTIATION: &[u8] = &[IAC, SB, REJECT, IAC, SE];

    pub const fn escape_char(s: u8) -> Option<&'static [u8]> {
        Some(match s {
            self::ESC => b"[ESC]",
            self::WILL_EOR => b"[WILL_EOR]",
            self::EOR => b"[EOR]",
            self::SE => b"[SE]",
            self::NOP => b"[NOP]",
            self::DM => b"[DM]",
            self::BRK => b"[BRK]",
            self::IP => b"[IP]",
            self::AO => b"[AO]",
            self::AYT => b"[AYT]",
            self::EC => b"[EC]",
            self::EL => b"[EL]",
            self::GA => b"[GA]",
            self::SB => b"[SB]",
            self::WILL => b"[WILL]",
            self::WONT => b"[WONT]",
            self::DO => b"[DO]",
            self::DONT => b"[DONT]",
            self::IAC => b"[IAC]",

            self::ECHO => b"[ECHO]/[TTYPE_SEND]",
            self::NAWS => b"[NAWS]",
            self::CHARSET => b"[CHARSET]",
            self::TERMINAL_TYPE => b"[TT]",
            self::COMPRESS => b"[COMPRESS]",
            self::COMPRESS2 => b"[COMPRESS2]",
            self::MUD_SPECIFIC => b"[MUDSPECIFIC]",
            self::SGA => b"[SGA]/[REJECT]",
            self::MSP => b"[MSP]",
            self::MXP => b"[MXP]",
            self::ZMP => b"[ZMP]",
            self::ATCP => b"[ATCP]",
            self::TTYPE_IS => b"[TTYPE_IS]",
            self::ACCEPT => b"[ACCEPT]",
            _ => return None,
        })
    }

    pub fn escape(s: &[u8]) -> Vec<u8> {
        let mut escaped = Vec::with_capacity(s.len());
        for &c in s {
            match escape_char(c) {
                None => escaped.push(c),
                Some(esc) => escaped.extend_from_slice(esc),
            }
        }
        escaped
    }

    pub fn wrap_ttype<T: AsRef<[u8]>>(ttype: T) -> Vec<u8> {
        let bytes = ttype.as_ref();
        let trimmed = if bytes.len() > 20 {
            &bytes[..20]
        } else {
            bytes
        };
        [&[IAC, SB, TERMINAL_TYPE, TTYPE_IS], trimmed, &[IAC, SE]].concat()
    }

    pub fn find_charset(data: &[u8], utf8: bool) -> &'static [u8] {
        let delim = data[1];
        let mut fragments = data[2..].split(|&c| c == delim);
        if !utf8 {
            return if fragments.any(|x| x == b"US-ASCII") {
                CHARSET_US_ASCII
            } else {
                REJECT_SUBNEGOTIATION
            };
        };
        let mut supports_ascii = false;
        for fragment in fragments {
            if fragment == b"UTF-8" {
                return CHARSET_UTF8;
            }
            if fragment == b"US-ASCII" {
                supports_ascii = true;
            }
        }
        if supports_ascii {
            CHARSET_US_ASCII
        } else {
            REJECT_SUBNEGOTIATION
        }
    }
}

pub mod ansi {
    pub const RESET: u8 = 0;
    pub const BOLD: u8 = 1;
    pub const BLINK: u8 = 3;
    pub const UNDERLINE: u8 = 4;
    pub const SLOW_BLINK: u8 = 5;
    pub const FAST_BLINK: u8 = 6;
    pub const INVERSE: u8 = 7;
    pub const STRIKEOUT: u8 = 9;

    pub const CANCEL_BOLD: u8 = 22;
    pub const CANCEL_BLINK: u8 = 23;
    pub const CANCEL_UNDERLINE: u8 = 24;
    pub const CANCEL_SLOW_BLINK: u8 = 25;
    pub const CANCEL_FAST_BLINK: u8 = 26;
    pub const CANCEL_INVERSE: u8 = 27;
    pub const CANCEL_STRIKEOUT: u8 = 29;

    pub const FG_BLACK: u8 = 30;
    pub const FG_RED: u8 = 31;
    pub const FG_GREEN: u8 = 32;
    pub const FG_YELLOW: u8 = 33;
    pub const FG_BLUE: u8 = 34;
    pub const FG_MAGENTA: u8 = 35;
    pub const FG_CYAN: u8 = 36;
    pub const FG_WHITE: u8 = 37;
    pub const FG_256_COLOR: u8 = 38;
    pub const FG_DEFAULT: u8 = 39;

    pub const BG_BLACK: u8 = 40;
    pub const BG_RED: u8 = 41;
    pub const BG_GREEN: u8 = 42;
    pub const BG_YELLOW: u8 = 43;
    pub const BG_BLUE: u8 = 44;
    pub const BG_MAGENTA: u8 = 45;
    pub const BG_CYAN: u8 = 46;
    pub const BG_WHITE: u8 = 47;
    pub const BG_256_COLOR: u8 = 48;
    pub const BG_DEFAULT: u8 = 49;

    pub const fn append_digit_to_code(code: u8, digit: u8) -> u8 {
        code * 10 + (digit - b'0')
    }
}

pub mod utf8 {
    pub const fn is_higher_order(c: u8) -> bool {
        (c & 0x80) != 0
    }

    pub const fn is_continuation(c: u8) -> bool {
        (c & 0xC0) != 0x80
    }
}
