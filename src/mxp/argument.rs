use std::collections::hash_map;
use std::iter::{self, Chain, Enumerate, Map};
use std::ops::{Deref, DerefMut};
use std::{slice, str};

use enumeration::{Enum, EnumSet};
use mlua::{Lua, Value};

use super::{validate, Error, ParseError, Words};
use crate::case_insensitive::ascii::{CaseFold, CaseFoldMap};
use crate::script::ScriptArg;

pub type Argument = String;
pub type Arg = str;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
impl<'a> ArgumentIndex<'a> {
    pub fn is_positional(self) -> bool {
        match self {
            Self::Positional(..) => true,
            Self::Named(..) => false,
        }
    }

    pub fn is_named(self) -> bool {
        match self {
            Self::Positional(..) => false,
            Self::Named(..) => true,
        }
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

#[derive(Clone, Debug)]
pub struct Scan<'a> {
    inner: iter::Map<slice::Iter<'a, Argument>, fn(&Argument) -> &Arg>,
    named: &'a CaseFoldMap<String, Argument>,
}

impl<'a> Scan<'a> {
    pub fn next_or(&mut self, names: &[&str]) -> Option<&'a Arg> {
        self.inner.next().or_else(|| {
            names
                .iter()
                .find_map(|&name| self.named.get(name))
                .map(String::as_str)
        })
    }
}

impl<'a> Deref for Scan<'a> {
    type Target = iter::Map<slice::Iter<'a, Argument>, fn(&Argument) -> &Arg>;

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
    named: CaseFoldMap<String, Argument>,
    keywords: EnumSet<Keyword>,
}

impl Arguments {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.positional.len() + self.named.len()
    }

    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.named.is_empty()
    }

    pub fn get<'a, Idx: Into<ArgumentIndex<'a>>>(&self, idx: Idx) -> Option<&Arg> {
        match idx.into() {
            ArgumentIndex::Positional(i) => self.positional.get(i),
            ArgumentIndex::Named(name) => self.named.get(name),
        }
        .map(String::as_str)
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

    pub fn push(&mut self, arg: Argument) {
        self.positional.push(arg);
    }

    pub fn set(&mut self, key: &str, arg: Argument) {
        self.named.insert(key.to_owned(), arg);
    }

    pub fn iter(&self) -> impl Iterator<Item = (ArgumentIndex, &Arg)> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ArgumentIndex, &mut Argument)> {
        self.into_iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &Arg> {
        self.positional
            .iter()
            .chain(self.named.values())
            .map(String::as_str)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Argument> {
        self.positional.iter_mut().chain(self.named.values_mut())
    }

    pub fn scan(&self) -> Scan {
        Scan {
            inner: self.positional.iter().map(String::as_str),
            named: &self.named,
        }
    }

    pub fn parse(tag: &str) -> Result<Self, ParseError> {
        Self::parse_words(Words::new(tag))
    }

    pub fn parse_words(iter: Words) -> Result<Self, ParseError> {
        let mut this = Self::new();
        this.append(iter)?;
        Ok(this)
    }

    pub fn append(&mut self, mut iter: Words) -> Result<(), ParseError> {
        while let Some(name) = iter.next() {
            if name == "/" {
                if iter.next().is_none() {
                    return Ok(());
                } else {
                    return Err(ParseError::new(name, Error::InvalidArgumentName));
                }
            }
            if iter.as_str().starts_with('=') {
                validate(name, Error::InvalidArgumentName)?;
                iter.next();
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

impl ScriptArg for &Arguments {
    fn to_arg(self, lua: &Lua) -> mlua::Result<Value> {
        let pos_iter = self
            .positional
            .iter()
            .enumerate()
            .map(|(i, v)| (Value::Integer(i as i64), v.as_str()));
        let named_iter: mlua::Result<Vec<_>> = self
            .named
            .iter()
            .map(|(k, v)| Ok((Value::String(lua.create_string(k.as_str())?), v.as_str())))
            .collect();
        lua.create_table_from(named_iter?.into_iter().chain(pos_iter))
            .map(Value::Table)
    }
}
impl ScriptArg for Arguments {
    fn to_arg(self, lua: &Lua) -> mlua::Result<Value> {
        (&self).to_arg(lua)
    }
}

// Just some nicknames for internal use
type Index<'a> = ArgumentIndex<'a>;

type IterItem<'a> = (Index<'a>, &'a Arg);
type IterItemMut<'a> = (Index<'a>, &'a mut Argument);

type PositionalEntry<'a> = (usize, &'a Argument);
type PositionalEntryMut<'a> = (usize, &'a mut Argument);

type NamedEntry<'a> = (&'a CaseFold<String>, &'a Argument);
type NamedEntryMut<'a> = (&'a CaseFold<String>, &'a mut Argument);

type Iter<'a, A, B, SliceIter, MapIter> = Chain<
    Map<Enumerate<SliceIter>, fn((usize, A)) -> (Index<'a>, B)>,
    Map<MapIter, fn((&'a CaseFold<String>, A)) -> (Index<'a>, B)>,
>;
type IntoIter<'a> = Iter<
    'a,
    &'a Argument,
    &'a Arg,
    slice::Iter<'a, Argument>,
    hash_map::Iter<'a, CaseFold<String>, Argument>,
>;
type IntoIterMut<'a> = Iter<
    'a,
    &'a mut Argument,
    &'a mut Argument,
    slice::IterMut<'a, Argument>,
    hash_map::IterMut<'a, CaseFold<String>, Argument>,
>;

impl<'a> IntoIterator for &'a Arguments {
    type IntoIter = IntoIter<'a>;
    type Item = IterItem<'a>;

    fn into_iter(self) -> IntoIter<'a> {
        let positional = self
            .positional
            .iter()
            .enumerate()
            .map((|(i, x)| (Index::Positional(i), x)) as fn(PositionalEntry) -> IterItem);

        let named = self
            .named
            .iter()
            .map((|(k, v)| (Index::Named(k.as_str()), v)) as fn(NamedEntry) -> IterItem);

        positional.chain(named)
    }
}

impl<'a> IntoIterator for &'a mut Arguments {
    type IntoIter = IntoIterMut<'a>;
    type Item = IterItemMut<'a>;

    fn into_iter(self) -> IntoIterMut<'a> {
        let positional = self
            .positional
            .iter_mut()
            .enumerate()
            .map((|(i, x)| (Index::Positional(i), x)) as fn(PositionalEntryMut) -> IterItemMut);

        let named = self
            .named
            .iter_mut()
            .map((|(k, v)| (Index::Named(k.as_str()), v)) as fn(NamedEntryMut) -> IterItemMut);

        positional.chain(named)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case_insensitive::ToCaseFold;

    #[test]
    fn arguments() {
        let args = Arguments::parse(r#"EL RName '<FONT COLOR=Red><B>' FLAG="RoomName""#).unwrap();
        let should_be = Arguments {
            positional: ["EL", "RName", "<FONT COLOR=Red><B>"]
                .iter()
                .map(ToString::to_string)
                .collect(),
            named: [("flag", "RoomName")]
                .iter()
                .map(|(k, v)| (k.to_string().to_case_fold(), v.to_string()))
                .collect(),
            keywords: Default::default(),
        };
        assert_eq!(args, should_be);
    }
}
