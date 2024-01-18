use std::iter::Iterator;
use std::{io, str};

use qmushclient_scripting::{Callback, PluginHandler};

use super::Client;
use crate::client::state::{Mccp, Phase};
use crate::client::Log;
use crate::escape::{ansi, telnet, utf8};
use crate::mxp;
use crate::world::{LogFormat, UseMxp};

impl<P: PluginHandler> Client<P> {
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

    #[cfg(feature = "show-special")]
    fn show_special(&mut self, old_phase: &mut Phase, c: u8) -> io::Result<()> {
        use qmushclient_scripting::Pad;
        use qt::core::AlignmentFlag;
        use qt::traits::Printable;

        use crate::client::color::WorldColor;

        if self.phase != *old_phase {
            self.flush()?;
            self.cursor.insert_text_colored(
                self.phase.to_str(),
                Some(self.world.color(&WorldColor::BRIGHT_BLACK)),
                None,
            );
            *old_phase = self.phase;
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

        Ok(())
    }

    pub(super) fn display_msg(&mut self, mut data: Vec<u8>) -> io::Result<()> {
        data = self
            .plugins
            .receive_from_all(Callback::PacketReceived, data);

        if data.is_empty() {
            return Ok(());
        }

        self.style.clear_flags();

        let mut iter = data.iter_mut();

        #[cfg(feature = "show-special")]
        let mut old_phase = self.phase;

        while let Some(&mut c) = iter.next() {
            #[cfg(feature = "show-special")]
            self.show_special(&mut old_phase, c)?;

            if self.phase == Phase::Utf8Character && !utf8::is_continuation(c) {
                self.bufoutput.append(&mut self.state.utf8_sequence);
                self.phase = Phase::Normal;
            }

            if self.phase.is_phase_reset(c) {
                if self.phase.is_mxp_mode_change() {
                    self.mxp_mode_change(None);
                }
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
                    self.state.utf8_sequence.push(c);
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
                    self.flush()?;
                    match c {
                        b'm' => {
                            self.interpret_code();
                            self.phase = Phase::Normal;
                        }
                        b';' | b':' => {
                            self.interpret_code();
                            self.state.ansi_code = 0;
                        }
                        b'z' => {
                            let mode = mxp::Mode(self.state.ansi_code);
                            if mode == mxp::Mode::RESET {
                                self.mxp_off(false);
                            } else {
                                self.mxp_mode_change(Some(mode));
                            }
                            self.phase = Phase::Normal;
                        }
                        b'0'..=b'9' => {
                            self.state.ansi_code =
                                ansi::append_digit_to_code(self.state.ansi_code, c);
                        }
                        _ => self.phase = Phase::Normal,
                    };
                }

                Phase::Iac => {
                    if c == telnet::IAC {
                        break;
                    }
                    self.state.subnegotiation_type = 0;
                    match c {
                        telnet::EOR | telnet::GA => {
                            self.phase = Phase::Normal;
                            self.state.last_line_with_iac_ga = self.api_state.linecount.get();
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

                Phase::Will => {
                    self.phase = Phase::Normal;
                    let verb = match c {
                        telnet::COMPRESS | telnet::COMPRESS2 if self.world.disable_compression => {
                            telnet::DONT
                        }
                        telnet::COMPRESS => {
                            if self.api_state.supports_mccp_2.get() {
                                telnet::DONT
                            } else {
                                telnet::DO
                            }
                        }
                        telnet::COMPRESS2 => {
                            self.api_state.supports_mccp_2.set(true);
                            telnet::DO
                        }
                        telnet::SGA | telnet::MUD_SPECIFIC => telnet::DO,
                        telnet::ECHO => {
                            if self.world.no_echo_off {
                                telnet::DONT
                            } else {
                                self.api_state.no_echo.set(true);
                                telnet::DO
                            }
                        }
                        telnet::MXP => match self.world.use_mxp {
                            UseMxp::Never => telnet::DONT,
                            UseMxp::Query => {
                                self.mxp_on(false, false);
                                telnet::DO
                            }
                            UseMxp::Always | UseMxp::Command => telnet::DO,
                        },
                        telnet::WILL_EOR => {
                            if self.world.convert_ga_to_newline {
                                telnet::DO
                            } else {
                                telnet::DONT
                            }
                        }
                        telnet::CHARSET => telnet::DO,
                        _ if self.telnet_callbacks(c, "WILL", "SENT_DO") => telnet::DO,
                        _ => telnet::DONT,
                    };
                    self.send_packet(&[telnet::IAC, verb, c])?;
                }

                Phase::Wont => {
                    self.phase = Phase::Normal;
                    if !self.world.no_echo_off {
                        self.api_state.no_echo.set(false);
                    }
                    self.send_packet(&[telnet::IAC, telnet::DONT, c])?;
                }

                Phase::Do => {
                    self.phase = Phase::Normal;

                    let verb = match c {
                        telnet::SGA | telnet::MUD_SPECIFIC | telnet::ECHO | telnet::CHARSET => {
                            telnet::WILL
                        }
                        telnet::TERMINAL_TYPE => {
                            self.state.ttype_sequence = 0;
                            telnet::WILL
                        }
                        telnet::NAWS => {
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
                            UseMxp::Always | UseMxp::Command => telnet::WILL,
                        },
                        _ if self.telnet_callbacks(c, "DO", "SENT_WILL") => telnet::WILL,
                        _ => telnet::WONT,
                    };
                    self.send_packet(&[telnet::IAC, verb, c])?;
                }

                Phase::Dont => {
                    self.phase = Phase::Normal;
                    let mxp = self.api_state.mxp_active.get();
                    self.send_packet(&[telnet::IAC, telnet::WONT, c])?;
                    match c {
                        telnet::MXP if mxp => self.mxp_off(true),
                        telnet::TERMINAL_TYPE => self.state.ttype_sequence = 0,
                        _ => (),
                    }
                }

                Phase::Sb => {
                    if c == telnet::COMPRESS {
                        self.phase = Phase::Compress;
                    } else {
                        self.state.subnegotiation_type = c;
                        self.state.subnegotiation_data.clear();
                        self.phase = Phase::Subnegotiation;
                    }
                }

                Phase::Subnegotiation => {
                    if c == telnet::IAC {
                        self.phase = Phase::SubnegotiationIac;
                    } else {
                        self.state.subnegotiation_data.push(c);
                    }
                }

                Phase::Compress => {
                    self.phase = if c == telnet::WILL {
                        Phase::CompressWill
                    } else {
                        Phase::Normal
                    };
                }

                Phase::CompressWill => {
                    if c == telnet::SE {
                        self.api_state.mccp_ver.set(Some(Mccp::V1));
                        iter.next();
                        self.start_decompressing(iter.into_slice().to_vec(), data);
                        return Ok(());
                    } else {
                        self.phase = Phase::Normal;
                    }
                }

                Phase::SubnegotiationIac => {
                    if c == telnet::IAC {
                        self.state.subnegotiation_data.push(c);
                        self.phase = Phase::Subnegotiation;
                    } else {
                        self.phase = Phase::Normal;
                        match self.state.subnegotiation_type {
                            telnet::COMPRESS2 => {
                                if !self.world.disable_compression {
                                    self.api_state.mccp_ver.set(Some(Mccp::V2));
                                    self.start_decompressing(iter.into_slice().to_vec(), data);
                                    return Ok(());
                                }
                            }
                            telnet::MXP => {
                                if self.world.use_mxp == UseMxp::Command {
                                    self.mxp_on(false, false);
                                }
                            }
                            telnet::TERMINAL_TYPE => {
                                if self.state.subnegotiation_data.first()
                                    == Some(&telnet::TTYPE_SEND)
                                {
                                    match self.state.ttype_sequence {
                                        0 => {
                                            self.state.ttype_sequence += 1;
                                            let ttype = &self.world.terminal_identification;
                                            self.send_packet(&telnet::wrap_ttype(ttype))
                                        }
                                        1 => {
                                            self.state.ttype_sequence += 1;
                                            self.send_packet(telnet::TTYPE_ANSI)
                                        }
                                        _ if self.world.utf_8 => {
                                            self.send_packet(telnet::TTYPE_UTF8)
                                        }
                                        _ => self.send_packet(telnet::TTYPE_XTERM),
                                    }?;
                                }
                            }
                            telnet::CHARSET => {
                                let data = &self.state.subnegotiation_data;
                                if data.len() >= 3 && data[0] == 1 {
                                    let charset = telnet::find_charset(data, self.world.utf_8);
                                    self.send_packet(charset)?;
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

                Phase::MxpComment => match c {
                    b'>' if self.state.mxp_string.ends_with(b"--") => {
                        self.phase = Phase::Normal;
                    }
                    _ => self.state.mxp_string.push(c),
                },

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
                    b'<' if self.api_state.mxp_active.get() && self.state.mxp_mode.is_mxp() => {
                        self.state.mxp_string.clear();
                        self.phase = Phase::MxpElement;
                    }
                    b'&' if self.api_state.mxp_active.get() && self.state.mxp_mode.is_mxp() => {
                        self.state.mxp_string.clear();
                        self.phase = Phase::MxpEntity;
                    }
                    _ if utf8::is_higher_order(c) => {
                        self.state.utf8_sequence.push(c);
                        self.phase = Phase::Utf8Character;
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
