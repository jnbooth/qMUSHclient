use crate::enums::{Enum, EnumSet};
use hashbrown::hash_map::{self, HashMap};
use std::iter::{Chain, Enumerate, Map};
use std::ops::{Deref, DerefMut};
use std::{slice, str};

use super::{validate, Error, ParseError, Words};

pub type Argument = String;
/*
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Arguments to an MXP tag.
pub struct Argument {
    /// Value of argument, e.g. red
    pub value: String,
    /// Whether the argument is used
    pub used: bool,
}

impl Argument {
    pub const fn new(value: String) -> Self {
        Self { value, used: false }
    }
}
*/

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArgumentIndex<'a> {
    Positional(usize),
    Named(&'a str),
}
impl<'a> From<usize> for ArgumentIndex<'a> {
    fn from(value: usize) -> Self {
        Self::Positional(value)
    }
}
impl<'a> From<&'a str> for ArgumentIndex<'a> {
    fn from(value: &'a str) -> Self {
        Self::Named(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Keyword {
    Delete,
    Open,
    Empty,
    Prompt,
    Off,
    DefaultOpen,
    DefaultSecure,
    DefaultLocked,
    UseNewlines,
    IgnoreNewlines,
    IsMap,
}

struct SizeGuess {
    positional: usize,
    named: usize,
}

impl SizeGuess {
    fn guess(tag: &[u8]) -> Self {
        let mut positional = 0;
        let mut named = 0;
        let mut in_named = false;
        let mut in_whitespace = false;
        for &c in tag {
            if c == b'=' {
                in_named = true;
            } else if !c.is_ascii_whitespace() {
                in_whitespace = false;
            } else if !in_whitespace {
                in_whitespace = true;
                if in_named {
                    named += 1;
                } else {
                    positional += 1;
                }
                in_named = false;
            }
        }
        Self { positional, named }
    }
}

#[derive(Clone, Debug)]
pub struct Scan<'a> {
    inner: slice::Iter<'a, Argument>,
    named: &'a HashMap<String, Argument>,
}

impl<'a> Scan<'a> {
    pub fn next_or(&mut self, name: &str) -> Option<&Argument> {
        self.inner.next().or_else(|| self.named.get(name))
    }
}

impl<'a> Deref for Scan<'a> {
    type Target = slice::Iter<'a, Argument>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for Scan<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Arguments {
    positional: Vec<Argument>,
    named: HashMap<String, Argument>,
    keywords: EnumSet<Keyword>,
}

impl Arguments {
    pub fn len(&self) -> usize {
        self.positional.len() + self.named.len()
    }

    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.named.is_empty()
    }

    pub fn get<'a, Idx: Into<ArgumentIndex<'a>>>(&self, idx: Idx) -> Option<&Argument> {
        match idx.into() {
            ArgumentIndex::Positional(i) => self.positional.get(i),
            ArgumentIndex::Named(name) => self.named.get(name),
        }
    }

    pub fn get_mut<'a, Idx: Into<ArgumentIndex<'a>>>(&mut self, idx: Idx) -> Option<&mut Argument> {
        match idx.into() {
            ArgumentIndex::Positional(i) => self.positional.get_mut(i),
            ArgumentIndex::Named(name) => self.named.get_mut(name),
        }
    }

    pub fn has_keyword(&self, k: Keyword) -> bool {
        self.keywords.contains(k)
    }

    pub fn iter(&self) -> impl Iterator<Item = (ArgumentIndex<'_>, &Argument)> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ArgumentIndex<'_>, &mut Argument)> {
        self.into_iter()
    }

    pub fn scan(&self) -> Scan {
        Scan {
            inner: self.positional.iter(),
            named: &self.named,
        }
    }

    pub fn parse(tag: &str) -> Result<Self, ParseError> {
        Self::parse_words(Words::new(tag))
    }

    pub fn parse_words(iter: Words) -> Result<Self, ParseError> {
        let guess = SizeGuess::guess(iter.as_str().as_bytes());
        let mut this = Self {
            positional: Vec::with_capacity(guess.positional),
            named: HashMap::with_capacity(guess.named),
            keywords: EnumSet::new(),
        };
        this.build(iter)?;
        Ok(this)
    }

    pub fn append(&mut self, words: Words) -> Result<(), ParseError> {
        let guess = SizeGuess::guess(words.as_str().as_bytes());
        self.positional.reserve(guess.positional);
        self.named.reserve(guess.named);
        self.build(words)
    }

    fn build(&mut self, mut iter: Words) -> Result<(), ParseError> {
        while let Some(name) = iter.next() {
            if name == "/" {
                if iter.next().is_none() {
                    // NB - not implemented yet - we have detected an empty tag.
                    // e.g. <sound blah blah />
                    return Ok(());
                } else {
                    return Err(ParseError::new(name, Error::InvalidArgumentName));
                }
            }
            if iter.as_str().starts_with('=') {
                validate(name, Error::InvalidArgumentName)?;
                iter.next(); // skip equals sign
                let val = iter
                    .next()
                    .ok_or_else(|| ParseError::new(name, Error::NoArgument))?;
                self.named.insert(name.to_lowercase(), val.to_owned());
            } else if let Some(keyword) = Keyword::parse_ci(name) {
                self.keywords.insert(keyword);
            } else {
                self.positional.push(name.to_owned());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arguments() {
        let args = Arguments::parse(r#"EL RName '<FONT COLOR=Red><B>' FLAG="RoomName""#).unwrap();
        let should_be = Arguments {
            positional: ["EL", "RName", "<FONT COLOR=Red><B>"].iter().map(ToString::to_string).collect(),
            named: [("flag", "RoomName")].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            keywords: Default::default(),
        };
        assert_eq!(args, should_be);
    }
}


// Just some nicknames for internal use
type Idx<'a> = ArgumentIndex<'a>;
type Arg = Argument;
type Iter2<'a, A, SliceIter, MapIter> = Chain<
    Map<Enumerate<SliceIter>, fn((usize, A)) -> (Idx<'a>, A)>,
    Map<MapIter, fn((&'a String, A)) -> (Idx<'a>, A)>,
>;

impl<'a> IntoIterator for &'a Arguments {
    type Item = (Idx<'a>, &'a Arg);

    type IntoIter = Iter2<'a, &'a Arg, slice::Iter<'a, Arg>, hash_map::Iter<'a, String, Arg>>;

    fn into_iter(self) -> Self::IntoIter {
        fn index_positional<'a>((i, x): (usize, &'a Arg)) -> (Idx<'a>, &'a Arg) {
            (Idx::Positional(i), x)
        }
        fn index_named<'a>((k, v): (&'a String, &'a Arg)) -> (Idx<'a>, &'a Arg) {
            (Idx::Named(k.as_str()), v)
        }
        let positional = self
            .positional
            .iter()
            .enumerate()
            .map(index_positional as fn((usize, &'a Arg)) -> (Idx<'a>, &'a Arg));
        let named = self
            .named
            .iter()
            .map(index_named as fn((&'a String, &'a Arg)) -> (Idx<'a>, &'a Arg));
        positional.chain(named)
    }
}

impl<'a> IntoIterator for &'a mut Arguments {
    type Item = (Idx<'a>, &'a mut Arg);

    type IntoIter =
        Iter2<'a, &'a mut Arg, slice::IterMut<'a, Arg>, hash_map::IterMut<'a, String, Arg>>;

    fn into_iter(self) -> Self::IntoIter {
        fn index_positional<'a>((i, x): (usize, &'a mut Arg)) -> (Idx<'a>, &'a mut Arg) {
            (Idx::Positional(i), x)
        }
        fn index_named<'a>((k, v): (&'a String, &'a mut Arg)) -> (Idx<'a>, &'a mut Arg) {
            (Idx::Named(k.as_str()), v)
        }
        let positional = self
            .positional
            .iter_mut()
            .enumerate()
            .map(index_positional as fn((usize, &'a mut Arg)) -> (Idx<'a>, &'a mut Arg));
        let named = self
            .named
            .iter_mut()
            .map(index_named as fn((&'a String, &'a mut Arg)) -> (Idx<'a>, &'a mut Arg));
        positional.chain(named)
    }
}
