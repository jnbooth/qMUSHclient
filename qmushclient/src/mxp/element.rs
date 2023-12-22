use std::ops::{Deref, DerefMut};

use case_insensitive::ascii::CaseFoldMap;
use enumeration::EnumSet;

use super::argument::{Arg, Arguments, Keyword};
use super::error::{validate, Error, ParseError};
use super::{Atom, Words};
use crate::mxp::TagFlag;

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
        let atom = Atom::get(atom_name)
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
    pub variable: Option<String>,
    /// Whether the element is open (OPEN)
    pub open: bool,
    /// Whether the element has no closing tag (EMPTY)
    pub command: bool,
}

impl Element {
    fn parse_items(argument: Option<&Arg>) -> Result<Vec<ElementItem>, ParseError> {
        let definitions = match argument {
            Some(definitions) => definitions,
            None => return Ok(Vec::new()),
        };

        let size_guess = definitions.bytes().filter(|&c| c == b'<').count();
        let mut items = Vec::with_capacity(size_guess);

        let mut iter = definitions.char_indices();
        while let Some((start, startc)) = iter.next() {
            if startc != '<' {
                return Err(ParseError::new(definitions, Error::NoTagInDefinition));
            }
            loop {
                let (end, endc) = iter
                    .next()
                    .ok_or_else(|| ParseError::new(definitions, Error::NoClosingDefinitionQuote))?;
                if endc == '>' {
                    let definition = &definitions[start + 1..end];
                    items.push(ElementItem::parse(definition)?);
                    break;
                }
                if (endc == '\'' || endc == '"') && !iter.any(|(_, c)| c == endc) {
                    return Err(ParseError::new(
                        definitions,
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

        let attributes = match scanner.next_or(&["att"]) {
            Some(atts) => Arguments::parse(atts)?,
            None => Arguments::default(),
        };

        let tag = match scanner.next_or(&["tag"]).and_then(|s| s.parse().ok()) {
            Some(i) if !(20..=99).contains(&i) => None,
            tag => tag,
        };

        let flag = scanner.next_or(&["flag"]).map(|flag| {
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
            variable: flag,
        })
    }
}

#[derive(Debug)]
pub enum ElementComponent<'a> {
    Atom(Atom),
    Custom(&'a Element),
}

impl<'a> ElementComponent<'a> {
    pub fn flags(&self) -> EnumSet<TagFlag> {
        match self {
            Self::Atom(atom) => atom.flags,
            Self::Custom(el) => {
                let mut flags = enums![];
                if el.open {
                    flags.insert(TagFlag::Open);
                }
                if el.command {
                    flags.insert(TagFlag::Command);
                }
                flags
            }
        }
    }

    pub fn variable(&self) -> Option<String> {
        match self {
            Self::Atom(..) => None,
            Self::Custom(el) => el.variable.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ElementMap(CaseFoldMap<String, Element>);

impl Deref for ElementMap {
    type Target = CaseFoldMap<String, Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElementMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ElementMap {
    pub fn get_component(&self, key: &str) -> Result<ElementComponent, ParseError> {
        validate(key, Error::InvalidElementName)?;
        if let Some(atom) = Atom::get(key) {
            Ok(ElementComponent::Atom(atom))
        } else if let Some(custom) = self.get(key) {
            Ok(ElementComponent::Custom(custom))
        } else {
            Err(ParseError::new(key, Error::UnknownElement))
        }
    }
}
