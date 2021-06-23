use crate::caseinsensitive::ascii::CaseFoldMap;
use crate::tr::TrContext;
use std::ops::{Deref, DerefMut};
use std::str;

mod argument;
mod atom;
mod error;
mod words;

pub use argument::{Argument, ArgumentIndex, Arguments, Keyword};
pub use atom::{Action, Atom, Tag, TagFlag};
pub use error::{validate, Error, Info, ParseError, Warning};
pub use words::Words;

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
    /// The line is parsed as the name of a room.
    pub const ROOM_NAME: Self = Self(10);
    /// The line is parsed as the description of a room.
    pub const ROOM_DESCRIPTION: Self = Self(11);
    /// The line is parsed as an exit line for a room.
    pub const ROOM_EXITS: Self = Self(12);
    /// Text sent from the server at the beginning of a session.
    pub const WELCOME: Self = Self(19);
}

impl Mode {
    pub const fn is_permanent(self) -> bool {
        match self {
            Self::PERM_OPEN | Self::PERM_SECURE | Self::PERM_LOCKED => true,
            _ => false,
        }
    }
    pub const fn is_open(self) -> bool {
        match self {
            Self::OPEN | Self::PERM_OPEN => true,
            _ => false,
        }
    }
    pub const fn is_secure(self) -> bool {
        match self {
            Self::SECURE | Self::SECURE_ONCE | Self::PERM_SECURE => true,
            _ => false,
        }
    }
    pub const fn is_mxp(self) -> bool {
        match self {
            Self::OPEN | Self::PERM_OPEN | Self::SECURE | Self::SECURE_ONCE | Self::PERM_SECURE => {
                true
            }
            _ => false,
        }
    }

    pub fn name(self) -> String {
        match self {
            Self::OPEN => tr!("open"),
            Self::SECURE => tr!("secure"),
            Self::LOCKED => tr!("locked"),
            Self::RESET => tr!("reset"),
            Self::SECURE_ONCE => tr!("secure next tag only"),
            Self::PERM_OPEN => tr!("permanently open"),
            Self::PERM_SECURE => tr!("permanently secure"),
            Self::PERM_LOCKED => tr!("permanently locked"),
            Self(mode) => tr!("unknown mode {}", mode),
        }
        .to_std_string()
    }
}

/// List of arguments to an MXP tag.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementItem {
    pub atom: Atom,
    pub arguments: Arguments,
}

impl ElementItem {
    pub fn parse(tag: &str) -> Result<Self, ParseError> {
        let mut words = Words::new(tag);
        let atom_name = words
            .next()
            .ok_or_else(|| ParseError::new(tag, Error::NoDefinitionTag))?;
        let invalid_name = match atom_name {
            "/" => Some(Error::DefinitionCannotCloseElement),
            "!" => Some(Error::DefinitionCannotDefineElement),
            _ => None,
        };
        if let Some(invalid) = invalid_name {
            return Err(ParseError::new(words.next().unwrap_or(""), invalid));
        }
        let atom = Atom::get(&atom_name.to_lowercase())
            .ok_or_else(|| ParseError::new(atom_name, Error::NoInbuiltDefinitionTag))?;
        Ok(Self {
            atom,
            arguments: Arguments::parse_words(words)?,
        })
    }
}

/// User-defined MXP tags that we recognise, e.g. <boldcolor>.
/// For example: <!ELEMENT boldtext '<COLOR &col;><B>' ATT='col=red'>
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Element {
    /// Tag name
    pub name: String,
    /// What atomic elements it defines (arg 1)
    pub items: Vec<ElementItem>,
    /// List of attributes to this element (ATT="xx")
    pub attributes: Arguments,
    /// Line tag number (20 - 99) (TAG=n)
    pub tag: Option<u8>,
    /// Which variable to set (SET x)
    pub flag: Option<String>,
    /// Whether the element is open (OPEN)
    pub open: bool,
    /// Whether the element has no closing tag (EMPTY)
    pub command: bool,
}

impl Element {
    fn parse_items(argument: Option<&Argument>) -> Result<Vec<ElementItem>, ParseError> {
        let definitions = match argument {
            Some(definitions) => definitions,
            None => return Ok(Vec::new()),
        };

        let size_guess = definitions
            .as_bytes()
            .iter()
            .filter(|&&c| c == b'<')
            .count();
        let mut items = Vec::with_capacity(size_guess);
        let mut iter = definitions.char_indices();
        while let Some((start, startc)) = iter.next() {
            if startc != '<' {
                println!("It's {} from {} in {}", startc, start, definitions);
                return Err(ParseError::new(&definitions, Error::NoTagInDefinition));
            }
            loop {
                let (end, endc) = iter
                    .next()
                    .ok_or_else(|| ParseError::new(&definitions, Error::NoClosingDefinitionQuote))?;
                if endc == '>' {
                    let definition = &definitions[start + 1..end];
                    items.push(ElementItem::parse(definition)?);
                    break;
                }
                if (endc == '\'' || endc == '"') && iter.find(|&(_, c)| c == endc).is_none() {
                    return Err(ParseError::new(
                        &definitions,
                        Error::NoClosingDefinitionQuote,
                    ));
                }
            }
        }

        Ok(items)
    }

    pub fn parse(name: String, arguments: Arguments) -> Result<Self, ParseError> {
        let mut scanner = arguments.scan();
        let items = Self::parse_items(scanner.next())?;

        let attributes = match scanner.next_or("att") {
            Some(atts) => Arguments::parse(atts)?,
            None => Arguments::default(),
        };

        let tag = match scanner.next_or("tag").and_then(|s| s.parse().ok()) {
            Some(i) if i < 20 || i > 99 => None,
            tag => tag,
        };

        let flag = scanner.next_or("flag").map(|flag| {
            flag.strip_prefix("set ")
                .unwrap_or(flag)
                .trim()
                .replace(' ', "_")
        });

        Ok(Self {
            name,
            open: arguments.has_keyword(Keyword::Open),
            command: arguments.has_keyword(Keyword::Empty),
            items,
            attributes,
            tag,
            flag,
        })
    }
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

const CHARS: &'static str = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2a\x2b\x2c\x2d\x2e\x2f\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3a\x3b\x3c\x3d\x3e\x3f\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4a\x4b\x4c\x4d\x4e\x4f\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5a\x5b\x5c\x5d\x5e\x5f\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6a\x6b\x6c\x6d\x6e\x6f\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7a\x7b\x7c\x7d\x7e\x7f";

impl EntityMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Result<Option<&str>, ParseError> {
        if key.starts_with('#') {
            let id = if key.starts_with('x') {
                u8::from_str_radix(&key[1..], 16)
            } else {
                u8::from_str_radix(key, 10)
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
        let mut res = String::with_capacity(s.len());
        let mut iter = s.char_indices();
        while let Some((start, startc)) = iter.next() {
            if startc == '&' {
                let end = iter
                    .find(|&(_, x)| x == ';')
                    .ok_or_else(|| ParseError::new(s, Error::NoClosingSemicolon))?
                    .0;
                let key = &s[start..end];
                res.push_str(self.get(key)?.unwrap_or(key));
            } else {
                res.push(startc);
            }
        }
        Ok(res)
    }

    pub const fn global(key: &str) -> Option<&'static str> {
        match key.as_bytes() {
            b"lt" => Some("<"),
            b"gt" => Some(">"),
            b"amp" => Some("&"),
            b"quot" => Some("\""),
            b"apos" => Some("'"),
            b"nbsp" => Some(" "),
            b"iexcl" => Some("¡"),
            b"cent" => Some("¢"),
            b"pound" => Some("£"),
            b"curren" => Some("¤"),
            b"yen" => Some("¥"),
            b"brvbar" => Some("¦"),
            b"sect" => Some("§"),
            b"uml" => Some("¨"),
            b"copy" => Some("©"),
            b"ordf" => Some("ª"),
            b"laquo" => Some("«"),
            b"not" => Some("¬"),
            b"shy" => Some("­"),
            b"reg" => Some("®"),
            b"macr" => Some("¯"),
            b"deg" => Some("°"),
            b"plusmn" => Some("±"),
            b"sup2" => Some("²"),
            b"sup3" => Some("³"),
            b"acute" => Some("´"),
            b"micro" => Some("µ"),
            b"para" => Some("¶"),
            b"middot" => Some("·"),
            b"cedil" => Some("¸"),
            b"sup1" => Some("¹"),
            b"ordm" => Some("º"),
            b"raquo" => Some("»"),
            b"frac14" => Some("¼"),
            b"frac12" => Some("½"),
            b"frac34" => Some("¾"),
            b"iquest" => Some("¿"),
            b"Agrave" => Some("À"),
            b"Aacute" => Some("Á"),
            b"Acirc" => Some("Â"),
            b"Atilde" => Some("Ã"),
            b"Auml" => Some("Ä"),
            b"Aring" => Some("Å"),
            b"AElig" => Some("Æ"),
            b"Ccedil" => Some("Ç"),
            b"Egrave" => Some("È"),
            b"Eacute" => Some("É"),
            b"Ecirc" => Some("Ê"),
            b"Euml" => Some("Ë"),
            b"Igrave" => Some("Ì"),
            b"Iacute" => Some("Í"),
            b"Icirc" => Some("Î"),
            b"Iuml" => Some("Ï"),
            b"ETH" => Some("Ð"),
            b"Ntilde" => Some("Ñ"),
            b"Ograve" => Some("Ò"),
            b"Oacute" => Some("Ó"),
            b"Ocirc" => Some("Ô"),
            b"Otilde" => Some("Õ"),
            b"Ouml" => Some("Ö"),
            b"times" => Some("×"),
            b"Oslash" => Some("Ø"),
            b"Ugrave" => Some("Ù"),
            b"Uacute" => Some("Ú"),
            b"Ucirc" => Some("Û"),
            b"Uuml" => Some("Ü"),
            b"Yacute" => Some("Ý"),
            b"THORN" => Some("Þ"),
            b"szlig" => Some("ß"),
            b"agrave" => Some("à"),
            b"aacute" => Some("á"),
            b"acirc" => Some("â"),
            b"atilde" => Some("ã"),
            b"auml" => Some("ä"),
            b"aring" => Some("å"),
            b"aelig" => Some("æ"),
            b"ccedil" => Some("ç"),
            b"egrave" => Some("è"),
            b"eacute" => Some("é"),
            b"ecirc" => Some("ê"),
            b"euml" => Some("ë"),
            b"igrave" => Some("ì"),
            b"iacute" => Some("í"),
            b"icirc" => Some("î"),
            b"iuml" => Some("ï"),
            b"eth" => Some("ð"),
            b"ntilde" => Some("ñ"),
            b"ograve" => Some("ò"),
            b"oacute" => Some("ó"),
            b"ocirc" => Some("ô"),
            b"otilde" => Some("õ"),
            b"ouml" => Some("ö"),
            b"divide" => Some("÷"),
            b"oslash" => Some("ø"),
            b"ugrave" => Some("ù"),
            b"uacute" => Some("ú"),
            b"ucirc" => Some("û"),
            b"uuml" => Some("ü"),
            b"yacute" => Some("ý"),
            b"thorn" => Some("þ"),
            b"yuml" => Some("ÿ"),
            _ => None,
        }
    }
}
