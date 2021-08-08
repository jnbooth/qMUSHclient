use std::convert::TryInto;
use std::iter::Iterator;
use std::{io, str};

#[cfg(feature = "show-special")]
use qt_core::AlignmentFlag;

use super::Client;
#[cfg(feature = "show-special")]
use crate::binding::Printable;
#[cfg(feature = "show-special")]
use crate::client::color::WorldColor;
use crate::client::state::{Mccp, Phase};
use crate::client::Log;
use crate::escape::telnet;
use crate::mxp;
use crate::script::Callback;
#[cfg(feature = "show-special")]
use crate::ui::Pad;
use crate::world::{LogFormat, UseMxp};

#[inline]
fn left<T>(xs: &[T], amt: usize) -> &[T] {
    if xs.len() > amt {
        &xs[..amt]
    } else {
        xs
    }
}

impl Client {
    fn telnet_callbacks(&mut self, c: u8, verb: &str, confirm: &str) -> bool {
        let stop_on_true = enums![true];
        if self
            .plugins
            .send_to_all_until(Callback::TelnetRequest, (c, verb), stop_on_true)
        {
            self.plugins
                .send_to_all_until(Callback::TelnetRequest, (c, confirm), stop_on_true);
            true
        } else {
            false
        }
    }

    pub(super) fn display_msg(&mut self, mut data: Vec<u8>) -> io::Result<()> {
        data = self
            .plugins
            .receive_from_all(Callback::PacketReceived, data);
        if data.is_empty() {
            return Ok(()); // plugin discarded it
        }
        self.style.clear_flags(); // MUD input cancels style flags

        let mut iter = data.iter_mut();

        #[cfg(feature = "show-special")]
        let mut old_phase = self.phase;

        while let Some(&mut c) = iter.next() {
            #[cfg(feature = "show-special")]
            {
                if self.phase != old_phase {
                    self.flush()?;
                    self.cursor.insert_text_colored(
                        self.phase.to_str(),
                        Some(self.world.color(&WorldColor::BRIGHT_BLACK)),
                        None,
                    );
                    old_phase = self.phase;
                }
                if self.phase != Phase::Normal {
                    let data = if let Some(escaped) = telnet::escape_char(c) {
                        escaped.to_print()
                    } else if c.is_ascii() {
                        [c].to_print()
                    } else {
                        format!("{:#X}", c).to_print()
                    };
                    self.append_to_notepad(Pad::PacketDebug, AlignmentFlag::AlignLeft, data)
                }
            }

            // bail out of UTF-8 collection if a non-high order bit is found in the incoming stream
            if self.phase == Phase::Utf8Character && (c & 0x80) == 0 {
                self.output_bad_utf8();
            }
            // note that CR, LF, ESC and IAC can appear inside telnet negotiation now (version 4.48)
            if !(self.phase == Phase::Iac && c == telnet::IAC)
                && self.phase != Phase::Sb
                && self.phase != Phase::Subnegotiation
                && self.phase != Phase::SubnegotiationIac
                // the following characters will terminate any collection/negotiation phases
                //  newline, carriage-return, escape, IAC
                && matches!(c, b'\r' | b'\n' | b'\x1b' | b'\xff')
            {
                if self.phase == Phase::MxpRoomName
                    || self.phase == Phase::MxpRoomDescription
                    || self.phase == Phase::MxpRoomExits
                    || self.phase == Phase::MxpWelcome
                {
                    self.mxp_mode_change(None);
                }
                // cannot be in middle of escape sequence
                self.phase = Phase::Normal;
            }
            match self.phase {
                Phase::Esc => {
                    if c == b'[' {
                        self.phase = Phase::DoingCode;
                        self.state.ansi_code = 0;
                    } else {
                        self.phase = Phase::Normal;
                    }
                }
                Phase::Utf8Character => {
                    // append to our UTF8 sequence
                    self.state.utf8_sequence.push(c);

                    if let Ok(utf8array) = self.state.utf8_sequence.as_slice().try_into() {
                        match char::from_u32(u32::from_be_bytes(utf8array)) {
                            None => self.output_bad_utf8(),
                            Some(..) => {
                                self.phase = Phase::Normal;
                                self.bufoutput.append(&mut self.state.utf8_sequence);
                            }
                        }
                    }
                }
                Phase::DoingCode
                | Phase::Foreground256Start
                | Phase::Foreground256Finish
                | Phase::Background256Start
                | Phase::Background256Finish
                | Phase::Foreground24bFinish
                | Phase::Foreground24brFinish
                | Phase::Foreground24bgFinish
                | Phase::Foreground24bbFinish
                | Phase::Background24bFinish
                | Phase::Background24brFinish
                | Phase::Background24bgFinish
                | Phase::Background24bbFinish => {
                    self.flush()?; // style is changing, so be sure to print whatever we've got
                    if (b'0'..=b'9').contains(&c) {
                        self.state.ansi_code = self.state.ansi_code * 10 + (c - b'0');
                    } else if c == b'm' {
                        self.interpret_code();
                        self.phase = Phase::Normal;
                    } else if c == b';' || c == b':' {
                        // separator, eg. ESC[ 38:5:<n>
                        self.interpret_code();
                        self.state.ansi_code = 0;
                    } else if c == b'z' {
                        // MXP line security mode
                        let mode = mxp::Mode(self.state.ansi_code);
                        if mode == mxp::Mode::RESET {
                            self.mxp_off(false);
                        } else {
                            self.mxp_mode_change(Some(mode));
                        }
                        self.phase = Phase::Normal;
                    } else {
                        self.phase = Phase::Normal;
                    }
                }
                Phase::Iac => {
                    if c == telnet::IAC {
                        break;
                    }
                    self.state.subnegotiation_type = 0; // no subnegotiation type yet
                    match c {
                        telnet::EOR | telnet::GA => {
                            self.phase = Phase::Normal;
                            self.state.last_line_with_iac_ga = self.state.linecount;
                            self.plugins.send_to_all(Callback::IacGa, ());
                            if self.world.convert_ga_to_newline {
                                self.insert_line();
                                break;
                            } else {
                                continue;
                            }
                        }
                        telnet::SB => self.phase = Phase::Sb,
                        telnet::WILL => self.phase = Phase::Will,
                        telnet::WONT => self.phase = Phase::Wont,
                        telnet::DO => self.phase = Phase::Do,
                        telnet::DONT => self.phase = Phase::Dont,
                        _ => self.phase = Phase::Normal,
                    }
                    continue;
                }
                // WILL - we have IAC WILL x - reply DO or DONT
                // (generally based on client option settings)
                // for unknown types we query plugins: function OnPluginTelnetRequest (num, type)
                //    eg. num = 200, type = WILL
                // They reply true or false to handle or not handle that telnet type
                Phase::Will => {
                    // telnet negotiation : in response to WILL, we say DONT
                    // (except for compression, MXP, TERMINAL_TYPE and SGA), we *will* handle that)
                    self.phase = Phase::Normal; // back to normal text after this character
                    let verb = match c {
                        telnet::COMPRESS | telnet::COMPRESS2 => {
                            if self.world.disable_compression
                                || (c == telnet::COMPRESS && self.state.supports_mccp_2)
                            {
                                telnet::DONT
                            } else {
                                if c == telnet::COMPRESS2 {
                                    self.state.supports_mccp_2 = true;
                                }
                                telnet::DO
                            }
                        }
                        telnet::SGA => telnet::DO, // Suppress GoAhead
                        telnet::MUD_SPECIFIC => telnet::DO,
                        telnet::ECHO => {
                            if self.world.no_echo_off {
                                telnet::DONT
                            } else {
                                self.state.no_echo = true;
                                telnet::DO
                            }
                        }
                        telnet::MXP => match self.world.use_mxp {
                            UseMxp::Never => telnet::DONT,
                            UseMxp::Query => {
                                self.mxp_on(false, false);
                                telnet::DO
                            }
                            _ => telnet::DO,
                        },
                        telnet::WILL_EOR => {
                            if self.world.convert_ga_to_newline {
                                telnet::DO
                            } else {
                                telnet::DONT
                            }
                        }
                        telnet::CHARSET => telnet::DO,
                        _ => {
                            if self.telnet_callbacks(c, "WILL", "SENT_DO") {
                                telnet::DO
                            } else {
                                telnet::DONT
                            }
                        }
                    };
                    self.send_packet(&[telnet::IAC, verb, c])?;
                }
                // Received: IAC WONT x
                Phase::Wont => {
                    // telnet negotiation : in response to WONT, we say DONT
                    self.phase = Phase::Normal;
                    if !self.world.no_echo_off {
                        self.state.no_echo = false;
                    }
                    self.send_packet(&[telnet::IAC, telnet::DONT, c])?;
                }
                // Received: IAC DO x
                // for unknown types we query plugins: function OnPluginTelnetRequest (num, type)
                //    eg. num = 200, type = DO
                // They reply true or false to handle or not handle that telnet type
                Phase::Do => {
                    // telnet negotiation : in response to DO, we say WILL for:
                    //  <102> (Aardwolf), SGA, echo, NAWS, CHARSET, MXP and Terminal type
                    // for others we query plugins to see if they want to handle it or not
                    self.phase = Phase::Normal;

                    let verb = match c {
                        // things we will do
                        telnet::SGA | telnet::MUD_SPECIFIC | telnet::ECHO | telnet::CHARSET => {
                            telnet::WILL
                        }
                        // for MTTS start back at sequence 0
                        telnet::TERMINAL_TYPE => {
                            self.state.ttype_sequence = 0;
                            telnet::WILL
                        }
                        telnet::NAWS => {
                            // option off - must be server initiated
                            if self.world.naws {
                                self.state.naws_wanted = true;
                                self.send_window_sizes(self.world.wrap_column)?;
                                telnet::WILL
                            } else {
                                telnet::WONT
                            }
                        }
                        telnet::MXP => match self.world.use_mxp {
                            UseMxp::Never => telnet::WONT,
                            UseMxp::Query => {
                                self.mxp_on(false, false);
                                telnet::WILL
                            }
                            _ => telnet::WILL,
                        },
                        _ => {
                            if self.telnet_callbacks(c, "DO", "SENT_WILL") {
                                telnet::WILL
                            } else {
                                telnet::WONT
                            }
                        }
                    };
                    self.send_packet(&[telnet::IAC, verb, c])?;
                }
                // Received: IAC DONT x
                Phase::Dont => {
                    // telnet negotiation : in response to DONT, we say WONT
                    self.phase = Phase::Normal;
                    let mxp = self.state.mxp_active;
                    self.send_packet(&[telnet::IAC, telnet::WONT, c])?;
                    match c {
                        telnet::MXP if mxp => self.mxp_off(true),
                        // for MTTS start back at sequence 0
                        telnet::TERMINAL_TYPE => self.state.ttype_sequence = 0,
                        _ => (),
                    }
                }
                // SUBNEGOTIATION - we have IAC SB c
                // remember c (the type) and start collecting the data, as in:
                // IAC SB c <data> IAC SE
                Phase::Sb => {
                    // note IAC SB COMPRESS is a special case because they forgot to specify
                    // the IAC SE, and thus we can't use normal negotiation
                    if c == telnet::COMPRESS {
                        self.phase = Phase::Compress;
                    } else {
                        self.state.subnegotiation_type = c;
                        self.state.subnegotiation_data.clear();
                        self.phase = Phase::Subnegotiation;
                    }
                }
                // SUBNEGOTIATION - we have IAC SB c (data)
                // if we get an IAC remember it, because it may or may not be followed by IAC or SE
                Phase::Subnegotiation => {
                    if c == telnet::IAC {
                        self.phase = Phase::SubnegotiationIac;
                    } else {
                        self.state.subnegotiation_data.push(c);
                    }
                }
                // COMPRESSION - we have IAC SB COMPRESS x
                Phase::Compress => {
                    self.phase = if c == telnet::WILL {
                        Phase::CompressWill // should get
                    } else {
                        Phase::Normal // error
                    };
                }
                // COMPRESSION - we have IAC SB COMPRESS IAC/WILL x   (MCCP v1)
                Phase::CompressWill => {
                    if c == telnet::SE {
                        // end of subnegotiation
                        self.state.mccp_ver = Some(Mccp::V1);
                        // special case, can't keep treating the  data as if it was not compressed
                        // skip SE (normaly done at end of loop)
                        iter.next();
                        // initialise compression library if not already done and copy
                        // compressed data to compression buffer
                        self.start_decompressing(iter.into_slice().to_vec(), data);
                        // done with this loop, now it needs to be decompressed
                        return Ok(());
                    } else {
                        self.phase = Phase::Normal; // error
                    }
                }

                // SUBNEGOTIATION - we have IAC SB x (data) IAC c
                // if the c after IAC is IAC then that becomes a single IAC (which we store now)
                // otherwise it should be SE, and we assume it is
                // otherwise we have an invalid sequence
                Phase::SubnegotiationIac => {
                    if c == telnet::IAC {
                        // have IAC SB x <data> IAC IAC
                        // store the single IAC
                        self.state.subnegotiation_data.push(c);
                        self.phase = Phase::Subnegotiation;
                    } else {
                        // see: http://www.gammon.com.au/forum/?id=10043
                        // we have to assume that anything other than IAC is a SE, because
                        // the spec is silent on what to do otherwise
                        // end of subnegotiation
                        // negotiation is over, next byte is plaintext
                        self.phase = Phase::Normal;
                        // subnegotiation is complete ...
                        // we have IAC SB <m_subnegotiation_type> <m_IAC_subnegotiation_data> IAC SE
                        match self.state.subnegotiation_type {
                            // turn MCCP v2 on
                            telnet::COMPRESS2 => {
                                if !self.world.disable_compression {
                                    self.state.mccp_ver = Some(Mccp::V2);
                                    // special case, can't keep treating the  data as if it was not compressed
                                    // skip SE (normaly done at end of loop)
                                    //iter.next();
                                    // initialise compression library if not already done and copy
                                    // compressed data to compression buffer
                                    self.start_decompressing(iter.into_slice().to_vec(), data);
                                    // done with this loop, now it needs to be decompressed
                                    return Ok(());
                                }
                            }
                            // turn MXP on, if required on subnegotiation
                            telnet::MXP => {
                                // if wanted now
                                if self.world.use_mxp == UseMxp::Command {
                                    self.mxp_on(false, false);
                                }
                            }
                            // terminal type request
                            telnet::TERMINAL_TYPE => {
                                if self.state.subnegotiation_data.get(0) == Some(&telnet::TTYPE_SEND)
                                {
                                    // we reply: IAC SB TERMINAL-TYPE IS ... IAC SE
                                    // see: RFC 930 and RFC 1060
                                    // also see: http://tintin.sourceforge.net/mtts/
                                    let p1 = [
                                        telnet::IAC,
                                        telnet::SB,
                                        telnet::TERMINAL_TYPE,
                                        telnet::TTYPE_IS,
                                    ];
                                    /*
                                    On the first TTYPE SEND request the client should return its name, preferably without a version number and in all caps.

                                    On the second TTYPE SEND request the client should return a terminal type, preferably in all caps.
                                      Console clients should report the name of the terminal emulator,
                                      other clients should report one of the four most generic terminal types.

                                        "DUMB"              Terminal has no ANSI color or VT100 support.
                                        "ANSI"              Terminal supports all ANSI color codes. Supporting blink and underline is optional.
                                        "VT100"             Terminal supports most VT100 codes, including ANSI color codes.
                                        "XTERM"             Terminal supports all VT100 and ANSI color codes, xterm 256 colors, mouse tracking, and the OSC color palette.

                                    If 256 color detection for non MTTS compliant servers is a must it's an option
                                      to report "ANSI-256COLOR", "VT100-256COLOR", or "XTERM-256COLOR".
                                      The terminal is expected to support VT100, mouse tracking, and the OSC color palette if "XTERM-256COLOR" is reported.

                                    On the third TTYPE SEND request the client should return MTTS followed by a bitvector. The bit values and their names are defined below.

                                            1 "ANSI"              Client supports all ANSI color codes. Supporting blink and underline is optional.
                                            2 "VT100"             Client supports most VT100 codes.
                                            4 "UTF-8"             Client is using UTF-8 character encoding.
                                            8 "256 COLORS"        Client supports all xterm 256 color codes.
                                           16 "MOUSE TRACKING"    Client supports xterm mouse tracking.
                                           32 "OSC COLOR PALETTE" Client supports the OSC color palette.
                                           64 "SCREEN READER"     Client is using a screen reader.
                                          128 "PROXY"             Client is a proxy allowing different users to connect from the same IP address.

                                    */
                                    let text = match self.state.ttype_sequence {
                                        0 => {
                                            self.state.ttype_sequence += 1;
                                            left(self.world.terminal_identification.as_bytes(), 20)
                                        }
                                        1 => {
                                            self.state.ttype_sequence += 1;
                                            b"ANSI"
                                        }
                                        _ if self.world.utf_8 => b"MTTS 13",
                                        _ => b"MTTS 9",
                                    };
                                    let p2 = [telnet::IAC, telnet::SE];
                                    let packet = [&p1, text, &p2].concat();
                                    self.send_packet(&packet)?;
                                }
                            }
                            // IAC SB CHARSET REQUEST DELIMITER <name> DELIMITER
                            /*

                            For backwards compatibility:

                            Server sends:  IAC DO CHARSET
                            Client sends:  IAC WILL CHARSET

                              or:

                            See: https://tools.ietf.org/html/rfc2066

                            Server sends:  IAC WILL CHARSET
                            Client sends:  IAC DO CHARSET

                            Server sends:  IAC SB CHARSET REQUEST DELIM NAME IAC SE
                            Client sends:  IAC SB CHARSET ACCEPTED NAME IAC SE
                            or
                            Client sends:  IAC SB CHARSET REJECTED IAC SE

                            where:

                              CHARSET: 0x2A
                              REQUEST: 0x01
                              ACCEPTED:0x02
                              REJECTED:0x03
                              DELIM:   some character that does not appear in the charset name, other than IAC, eg. comma, space
                              NAME:    the character string "UTF-8" (or some other name like "S-JIS")

                            */
                            telnet::CHARSET => {
                                // must have at least REQUEST DELIM NAME [ DELIM NAME2 ...]
                                let data = self.state.subnegotiation_data.clone();
                                if data.len() >= 3 && data[0] == 1 {
                                    let delim = data[1];
                                    let charset: &[u8] = if self.world.utf_8 {
                                        // hack! ugh.
                                        b"UTF-8"
                                    } else {
                                        b"US-ASCII"
                                    };
                                    let mut found = false;
                                    for fragment in data[2..].split(|&c| c == delim) {
                                        if fragment == charset {
                                            found = true;
                                            let p1 = [
                                                telnet::IAC,
                                                telnet::SB,
                                                telnet::CHARSET,
                                                telnet::ACCEPT,
                                            ];
                                            let p2 = [telnet::IAC, telnet::SE];
                                            let packet = [&p1, left(fragment, 20), &p2].concat();
                                            self.send_packet(&packet)?;
                                        }
                                    }
                                    if !found {
                                        let packet = [
                                            telnet::IAC,
                                            telnet::SB,
                                            telnet::REJECT,
                                            telnet::IAC,
                                            telnet::SE,
                                        ];
                                        self.send_packet(&packet)?;
                                    }
                                }
                            }
                            telnet::MUD_SPECIFIC => {
                                let data = String::from_utf8_lossy(&self.state.subnegotiation_data);
                                self.plugins.send_to_all(Callback::TelnetOption, data);
                            }
                            _ => {
                                let sbtype = self.state.subnegotiation_type;
                                let data = String::from_utf8_lossy(&self.state.subnegotiation_data);
                                self.plugins
                                    .send_to_all(Callback::TelnetSubnegotiation, (sbtype, data));
                            }
                        }
                    }
                }
                Phase::MxpElement => match c {
                    b'>' => {
                        if let Err(e) = self.mxp_collected_element() {
                            self.handle_mxp_io_error(e)?;
                        }
                        self.phase = Phase::Normal;
                    }
                    b'<' => {
                        self.state.mxp_string.push(c);
                        self.handle_mxp_error(mxp::ParseError::new(
                            &String::from_utf8_lossy(&self.state.mxp_string),
                            mxp::Error::UnterminatedElement,
                        ));
                        self.state.mxp_string.clear();
                    }
                    b'\'' | b'"' => {
                        self.state.mxp_string.push(c);
                        self.state.mxp_quote_terminator = Some(c);
                        self.phase = Phase::MxpQuote;
                    }
                    b'-' => {
                        self.state.mxp_string.push(c);
                        if self.state.mxp_string.starts_with(b"!--") {
                            self.phase = Phase::MxpComment;
                        }
                    }
                    _ => self.state.mxp_string.push(c),
                },
                Phase::MxpComment => {
                    match c {
                        b'>' if self.state.mxp_string.ends_with(b"--") => {
                            // discard comment
                            self.phase = Phase::Normal;
                        }
                        _ => self.state.mxp_string.push(c),
                    }
                }
                Phase::MxpQuote => {
                    if self.state.mxp_quote_terminator == Some(c) {
                        self.phase = Phase::MxpElement;
                        self.state.mxp_quote_terminator = None;
                    }
                    self.state.mxp_string.push(c);
                }
                Phase::MxpEntity => match c {
                    b';' => {
                        self.phase = Phase::Normal;
                        if let Err(e) = self.mxp_collected_entity() {
                            self.handle_mxp_io_error(e)?;
                        }
                    }
                    b'&' => {
                        self.state.mxp_string.push(c);
                        self.handle_mxp_error(mxp::ParseError::new(
                            &String::from_utf8_lossy(&self.state.mxp_string),
                            mxp::Error::UnterminatedEntity,
                        ));
                        self.state.mxp_string.clear();
                    }
                    b'<' => {
                        self.state.mxp_string.push(c);
                        self.handle_mxp_error(mxp::ParseError::new(
                            &String::from_utf8_lossy(&self.state.mxp_string),
                            mxp::Error::UnterminatedEntity,
                        ));
                        self.state.mxp_string.clear();
                        self.phase = Phase::MxpElement;
                    }
                    _ => self.state.mxp_string.push(c),
                },
                Phase::MxpRoomName
                | Phase::MxpRoomDescription
                | Phase::MxpRoomExits
                | Phase::MxpWelcome => {
                    // nope
                }
                Phase::Normal => match c {
                    telnet::ESC => self.phase = Phase::Esc,
                    telnet::IAC => {
                        if self.phase == Phase::Iac {
                            self.bufoutput.push(c);
                            self.phase = Phase::Normal;
                        } else {
                            self.phase = Phase::Iac;
                        }
                    }
                    b'\r' => (),
                    b'\n' => self.insert_line(),
                    b'<' if self.state.mxp_active && self.state.mxp_mode.is_mxp() => {
                        self.state.mxp_string.clear();
                        self.phase = Phase::MxpElement;
                    }
                    b'&' if self.state.mxp_active && self.state.mxp_mode.is_mxp() => {
                        self.state.mxp_string.clear();
                        self.phase = Phase::MxpEntity;
                    }
                    _ => self.bufoutput.push(c),
                },
            }
        }
        self.flush()?;
        if self.world.log_format == LogFormat::Raw {
            self.write_to_log(Log::Output, &data);
        }
        self.scroll_to_bottom();
        Ok(())
    }
}
