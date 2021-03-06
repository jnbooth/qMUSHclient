use std::ops::{Deref, DerefMut};
use std::str;

use enumeration::{Enum, EnumSet};

use crate::binding::RColor;
use crate::caseinsensitive::ascii::CaseFoldMap;
use crate::client::style::TextStyle;
use crate::tr::TrContext;

mod argument;
mod atom;
mod element;
mod error;
mod words;

pub use argument::{Arg, Argument, ArgumentIndex, Arguments, Keyword};
pub use atom::{Action, Atom, Tag, TagFlag};
pub use element::{Element, ElementComponent, ElementItem, ElementMap};
pub use error::{is_valid, validate, Error, ParseError};
pub use words::Words;

pub const VERSION: &str = "0.5";

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, TrContext)]
pub struct Mode(pub u8);

impl Mode {
    /// Only MXP commands in the "open" category are allowed.
    pub const OPEN: Self = Self(0);
    /// All tags and commands in MXP are allowed within the line.
    pub const SECURE: Self = Self(1);
    /// No MXP or HTML commands are allowed in the line. The line is not parsed for any tags at all.
    pub const LOCKED: Self = Self(2);
    /// Close all open tags.
    pub const RESET: Self = Self(3);
    /// Next tag is secure only.
    pub const SECURE_ONCE: Self = Self(4);
    /// Open mode until mode change.
    pub const PERM_OPEN: Self = Self(5);
    /// Secure mode until mode change.
    pub const PERM_SECURE: Self = Self(6);
    /// Locked mode until mode change.
    pub const PERM_LOCKED: Self = Self(7);
}

impl Mode {
    pub const fn is_open(self) -> bool {
        matches!(self, Self::OPEN | Self::PERM_OPEN)
    }
    pub const fn is_secure(self) -> bool {
        matches!(self, Self::SECURE | Self::SECURE_ONCE | Self::PERM_SECURE)
    }
    pub const fn is_mxp(self) -> bool {
        matches!(
            self,
            Self::PERM_OPEN | Self::SECURE | Self::SECURE_ONCE | Self::PERM_SECURE
        )
    }
}

/// Not to be confused with [`trigger::SendTo`](crate::script::trigger::SendTo).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum SendTo {
    World,
    Input,
    Internet,
}

impl Default for SendTo {
    fn default() -> Self {
        Self::World
    }
}

impl SendTo {
    pub fn attach(self, s: &str) -> String {
        match self {
            Self::World => ["send:", s].concat(),
            Self::Input => ["echo:", s].concat(),
            _ if s.starts_with("echo:") || s.starts_with("send:") => ["http://", s].concat(),
            Self::Internet => s.to_owned(),
        }
    }

    pub fn detach(s: &str) -> (Self, &str) {
        if let Some(world) = s.strip_prefix("send:") {
            (Self::World, world)
        } else if let Some(input) = s.strip_prefix("echo:") {
            (Self::Input, input)
        } else {
            (Self::Internet, s)
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Link {
    pub action: String,
    /// Flyover hint.
    pub hint: Option<String>,
    /// Right-click prompts for actions.
    pub prompts: Vec<String>,
    /// Where to send the result of clicking on the link.
    pub sendto: SendTo,
}

impl Link {
    pub fn new(action: &str, hint: Option<&str>, sendto: SendTo) -> Self {
        let mut actions = action.split('|');
        let action = sendto.attach(actions.next().unwrap());
        match hint {
            None => Self {
                action,
                hint: None,
                prompts: actions.map(ToOwned::to_owned).collect(),
                sendto,
            },
            Some(hint) => {
                let mut hints = hint.split('|').map(ToOwned::to_owned);
                let first_hint = hints.next().unwrap();
                let mut prompts: Vec<_> = hints.collect();
                if prompts.is_empty() {
                    prompts = actions.map(ToOwned::to_owned).collect();
                }
                Self {
                    action,
                    hint: Some(first_hint),
                    prompts,
                    sendto,
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InList {
    Ordered(u32),
    Unordered,
}

// eg. <send "command1|command2|command3" hint="click to see menu|Item 1|Item
// 2|Item 2">this is a menu link</SEND>
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub flags: EnumSet<TextStyle>,
    pub foreground: Option<RColor>,
    pub background: Option<RColor>,
    pub action: Option<Link>,
    /// Which variable to set (FLAG in MXP).
    pub variable: Option<String>,
    pub list: Option<InList>,
}

impl Span {
    pub fn child(&self) -> Self {
        Self {
            flags: self.flags,
            foreground: None,
            background: None,
            action: None,
            variable: None,
            list: self.list,
        }
    }
}

fn decode_amps<'a, F>(mut s: &str, mut f: F) -> Result<String, ParseError>
where
    F: FnMut(&str) -> Result<Option<&'a str>, ParseError>,
{
    let mut res = String::with_capacity(s.len());
    while let Some(start) = s.find('&') {
        if start > 0 {
            res.push_str(&s[..start]);
        }
        s = &s[start..];
        let end = s
            .find(';')
            .ok_or_else(|| ParseError::new(s, Error::NoClosingSemicolon))?;
        res.push_str(f(&s[1..end])?.unwrap_or(&s[0..=end]));
        s = &s[end + 1..];
    }
    if !s.is_empty() {
        res.push_str(s);
    }
    Ok(res)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EntityMap(CaseFoldMap<String, String>);

impl Deref for EntityMap {
    type Target = CaseFoldMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const CHARS: &str = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2a\x2b\x2c\x2d\x2e\x2f\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3a\x3b\x3c\x3d\x3e\x3f\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4a\x4b\x4c\x4d\x4e\x4f\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5a\x5b\x5c\x5d\x5e\x5f\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6a\x6b\x6c\x6d\x6e\x6f\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7a\x7b\x7c\x7d\x7e\x7f";

impl EntityMap {
    pub fn get(&self, key: &str) -> Result<Option<&str>, ParseError> {
        if key.starts_with('#') {
            let id = match key.strip_prefix('x') {
                Some(hex) => u8::from_str_radix(hex, 16),
                None => u8::from_str_radix(key, 10),
            }
            .map_err(|_| ParseError::new(key, Error::InvalidEntityNumber))?;
            if id < 32 && id != b'\t' && id != b'\n' && id != b'\r' {
                Err(ParseError::new(key, Error::DisallowedEntityNumber))
            } else {
                let id = id as usize;
                match CHARS.get(id..=id) {
                    None => Err(ParseError::new(key, Error::DisallowedEntityNumber)),
                    some => Ok(some),
                }
            }
        } else {
            Ok(Self::global(key).or_else(|| self.0.get(key).map(String::as_str)))
        }
    }

    pub fn decode(&self, s: &str) -> Result<String, ParseError> {
        decode_amps(s, |entity| self.get(entity))
    }

    pub fn decode_el(&self, el: &Element, s: &str, args: &Arguments) -> Result<String, ParseError> {
        decode_amps(s, |entity| {
            if entity == "text" {
                return Ok(None);
            }
            match el.attributes.iter().find(|&(i, attr)| match i {
                ArgumentIndex::Positional(..) => attr.eq_ignore_ascii_case(entity),
                ArgumentIndex::Named(name) => name.eq_ignore_ascii_case(entity),
            }) {
                None => self.get(entity),
                Some((i, attr)) => Ok(match args.get(i) {
                    Some(arg) => Some(arg),
                    None if i.is_named() => Some(attr), // default replacement
                    None => Some(""),                   // TODO is this right?
                }),
            }
        })
    }

    pub const fn global(key: &str) -> Option<&'static str> {
        match key.as_bytes() {
            b"lt" => Some("<"),
            b"gt" => Some(">"),
            b"amp" => Some("&"),
            b"quot" => Some("\""),
            b"apos" => Some("'"),
            b"nbsp" => Some(" "),
            b"iexcl" => Some("??"),
            b"cent" => Some("??"),
            b"pound" => Some("??"),
            b"curren" => Some("??"),
            b"yen" => Some("??"),
            b"brvbar" => Some("??"),
            b"sect" => Some("??"),
            b"uml" => Some("??"),
            b"copy" => Some("??"),
            b"ordf" => Some("??"),
            b"laquo" => Some("??"),
            b"not" => Some("??"),
            #[allow(clippy::invisible_characters)]
            b"shy" => Some("??"),
            b"reg" => Some("??"),
            b"macr" => Some("??"),
            b"deg" => Some("??"),
            b"plusmn" => Some("??"),
            b"sup2" => Some("??"),
            b"sup3" => Some("??"),
            b"acute" => Some("??"),
            b"micro" => Some("??"),
            b"para" => Some("??"),
            b"middot" => Some("??"),
            b"cedil" => Some("??"),
            b"sup1" => Some("??"),
            b"ordm" => Some("??"),
            b"raquo" => Some("??"),
            b"frac14" => Some("??"),
            b"frac12" => Some("??"),
            b"frac34" => Some("??"),
            b"iquest" => Some("??"),
            b"Agrave" => Some("??"),
            b"Aacute" => Some("??"),
            b"Acirc" => Some("??"),
            b"Atilde" => Some("??"),
            b"Auml" => Some("??"),
            b"Aring" => Some("??"),
            b"AElig" => Some("??"),
            b"Ccedil" => Some("??"),
            b"Egrave" => Some("??"),
            b"Eacute" => Some("??"),
            b"Ecirc" => Some("??"),
            b"Euml" => Some("??"),
            b"Igrave" => Some("??"),
            b"Iacute" => Some("??"),
            b"Icirc" => Some("??"),
            b"Iuml" => Some("??"),
            b"ETH" => Some("??"),
            b"Ntilde" => Some("??"),
            b"Ograve" => Some("??"),
            b"Oacute" => Some("??"),
            b"Ocirc" => Some("??"),
            b"Otilde" => Some("??"),
            b"Ouml" => Some("??"),
            b"times" => Some("??"),
            b"Oslash" => Some("??"),
            b"Ugrave" => Some("??"),
            b"Uacute" => Some("??"),
            b"Ucirc" => Some("??"),
            b"Uuml" => Some("??"),
            b"Yacute" => Some("??"),
            b"THORN" => Some("??"),
            b"szlig" => Some("??"),
            b"agrave" => Some("??"),
            b"aacute" => Some("??"),
            b"acirc" => Some("??"),
            b"atilde" => Some("??"),
            b"auml" => Some("??"),
            b"aring" => Some("??"),
            b"aelig" => Some("??"),
            b"ccedil" => Some("??"),
            b"egrave" => Some("??"),
            b"eacute" => Some("??"),
            b"ecirc" => Some("??"),
            b"euml" => Some("??"),
            b"igrave" => Some("??"),
            b"iacute" => Some("??"),
            b"icirc" => Some("??"),
            b"iuml" => Some("??"),
            b"eth" => Some("??"),
            b"ntilde" => Some("??"),
            b"ograve" => Some("??"),
            b"oacute" => Some("??"),
            b"ocirc" => Some("??"),
            b"otilde" => Some("??"),
            b"ouml" => Some("??"),
            b"divide" => Some("??"),
            b"oslash" => Some("??"),
            b"ugrave" => Some("??"),
            b"uacute" => Some("??"),
            b"ucirc" => Some("??"),
            b"uuml" => Some("??"),
            b"yacute" => Some("??"),
            b"thorn" => Some("??"),
            b"yuml" => Some("??"),
            _ => None,
        }
    }
}
