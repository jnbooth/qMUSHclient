use std::borrow::Cow;
use std::hash::Hash;
use std::iter::FusedIterator;
use std::{iter, slice, str, vec};

use fancy_regex::{CaptureMatches, Captures, Replacer};

use super::file::PluginMetadata;
use super::pad::Pad;
use super::plugin::Plugin;
use crate::send::{Alias, Reaction, Sender, Timer, Trigger};

pub type PluginIndex = usize;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Indexed<T> {
    // Note: this is at the top for Ord-deriving purposes.
    val: T,
    plugin: PluginIndex,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Indexer<T>(Vec<Indexed<T>>);

impl<T> Default for Indexer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Indexer<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn replace<'a, I>(&mut self, index: PluginIndex, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a + Clone,
    {
        self.0.retain(|x| x.plugin != index);
        self.extend(index, iter);
    }

    pub fn extend<'a, I>(&mut self, index: PluginIndex, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a + Clone,
    {
        let new_iter = iter.into_iter().map(|content| Indexed {
            plugin: index,
            val: content.to_owned(),
        });
        self.0.extend(new_iter);
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.0.sort_unstable();
    }

    pub fn add(&mut self, plugin: PluginIndex, val: T)
    where
        T: Ord,
    {
        self.0.push(Indexed { val, plugin });
        self.sort();
    }

    pub fn iter(&self) -> impl Iterator<Item = (PluginIndex, &T)> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PluginIndex, &mut T)> {
        self.into_iter()
    }

    pub fn matches<'a, 'b: 'a>(&'a self, line: &'b str) -> impl Iterator<Item = SendMatch<'a, 'b, T>>
    where
        T: AsRef<Reaction> + Clone,
    {
        self.0.iter().enumerate().flat_map(move |(pos, indexed)| {
            let reaction = indexed.val.as_ref();
            if !reaction.enabled || !matches!(reaction.regex.is_match(line), Ok(true)) {
                return MatchIter::Repeat {
                    count: 0,
                    val: None,
                };
            }

            if reaction.script.is_empty() && !reaction.text.contains('$') {
                // We don't need captures, so we can simply replicate
                let count = if reaction.repeat {
                    reaction.regex.find_iter(line).count()
                } else {
                    1
                };
                let val = SendMatch {
                    plugin: indexed.plugin,
                    sender: &indexed.val,
                    pos,
                    text: Cow::Borrowed(&reaction.text),
                    wildcards: Vec::new(),
                };
                MatchIter::Repeat {
                    count,
                    val: Some(val),
                }
            } else {
                MatchIter::Captures {
                    pos,
                    iter: reaction.regex.captures_iter(line),
                    indexed,
                }
            }
        })
    }
}

#[allow(clippy::type_complexity)]
impl<T> IntoIterator for Indexer<T> {
    type Item = (PluginIndex, T);
    type IntoIter = iter::Map<vec::IntoIter<Indexed<T>>, fn(Indexed<T>) -> (PluginIndex, T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|i| (i.plugin, i.val))
    }
}

#[allow(clippy::type_complexity)]
impl<'a, T> IntoIterator for &'a Indexer<T> {
    type Item = (PluginIndex, &'a T);
    type IntoIter = iter::Map<slice::Iter<'a, Indexed<T>>, fn(&Indexed<T>) -> (PluginIndex, &T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().map(|i| (i.plugin, &i.val))
    }
}

#[allow(clippy::type_complexity)]
impl<'a, T> IntoIterator for &'a mut Indexer<T> {
    type Item = (PluginIndex, &'a mut T);
    type IntoIter =
        iter::Map<slice::IterMut<'a, Indexed<T>>, fn(&mut Indexed<T>) -> (PluginIndex, &mut T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut().map(|i| (i.plugin, &mut i.val))
    }
}

#[derive(Clone, Debug)]
pub struct SendMatch<'a, 'b, T> {
    pub plugin: PluginIndex,
    pub pos: usize,
    pub sender: &'a T,
    pub text: Cow<'a, str>,
    pub wildcards: Vec<&'b str>,
}

impl<'a, 'b, T: AsRef<Reaction>> SendMatch<'a, 'b, T> {
    fn from_captures(pos: usize, captures: &Captures<'b>, indexed: &'a Indexed<T>) -> Self {
        let reaction = indexed.val.as_ref();
        let text = if reaction.text.is_empty() {
            Cow::Borrowed(reaction.text.as_str())
        } else {
            let mut s = String::new();
            reaction.text.as_str().replace_append(captures, &mut s);
            Cow::Owned(s)
        };
        if reaction.script.is_empty() {
            return Self {
                plugin: indexed.plugin,
                sender: &indexed.val,
                pos,
                text,
                wildcards: Vec::new(),
            };
        }
        let mut wildcards = Vec::with_capacity(captures.len() - 1);
        let mut iter = captures.iter();
        iter.next(); // skip the all-encompassing capture
        for capture in iter.flatten() {
            wildcards.push(capture.as_str());
        }
        Self {
            plugin: indexed.plugin,
            sender: &indexed.val,
            pos,
            text,
            wildcards,
        }
    }
}

enum MatchIter<'a, 'b, T> {
    Repeat {
        count: usize,
        val: Option<SendMatch<'a, 'b, T>>,
    },
    Captures {
        pos: usize,
        iter: CaptureMatches<'a, 'b>,
        indexed: &'a Indexed<T>,
    },
}

impl<'a, 'b, T: AsRef<Reaction> + Clone> Iterator for MatchIter<'a, 'b, T> {
    type Item = SendMatch<'a, 'b, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Repeat { count: 0, .. } => None,
            Self::Repeat { count, val } => {
                *count -= 1;
                if *count == 0 {
                    val.take()
                } else {
                    val.as_ref().map(ToOwned::to_owned)
                }
            }
            Self::Captures { iter, pos, indexed } => {
                Some(SendMatch::from_captures(*pos, &iter.next()?.ok()?, indexed))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Repeat { count, .. } => (*count, Some(*count)),
            Self::Captures { iter, .. } => iter.size_hint(),
        }
    }
}
impl<'a, 'b, T: AsRef<Reaction> + Clone> FusedIterator for MatchIter<'a, 'b, T> {}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Senders {
    aliases: Indexer<Alias>,
    timers: Indexer<Timer>,
    triggers: Indexer<Trigger>,
}

impl Senders {
    pub const fn new() -> Self {
        Self {
            aliases: Indexer::new(),
            timers: Indexer::new(),
            triggers: Indexer::new(),
        }
    }

    pub fn clear(&mut self) {
        self.triggers.clear();
        self.aliases.clear();
        self.timers.clear();
    }

    pub fn extend(&mut self, i: PluginIndex, plugin: &Plugin) {
        self.triggers.extend(i, &plugin.triggers);
        self.triggers.sort();
        self.aliases.extend(i, &plugin.aliases);
        self.aliases.sort();
        self.timers.extend(i, &plugin.timers);
        self.timers.sort();
    }

    pub fn replace_all<T: Sendable + Ord + Clone>(&mut self, i: PluginIndex, vals: &[T]) {
        let indexer = T::indexer_mut(self);
        indexer.replace(i, vals);
        indexer.sort();
    }

    pub fn matches<'a, 'b: 'a, T: Sendable + AsRef<Reaction> + Clone>(
        &'a mut self,
        line: &'b str,
    ) -> impl Iterator<Item = SendMatch<'a, 'b, T>> {
        T::indexer_mut(self).matches(line)
    }

    // Given a sorted and deduplicated list of vector positions, deletes the elements corresponding
    // to those positions.
    pub fn delete_all<T: Sendable>(&mut self, positions: &[usize]) {
        let indexer = &mut T::indexer_mut(self).0;
        for &pos in positions.iter().rev() {
            indexer.remove(pos);
        }
    }

    pub fn add<T: Sendable>(&mut self, plugin: PluginIndex, val: T) {
        T::indexer_mut(self).add(plugin, val);
    }

    pub fn replace<T: Sendable>(&mut self, plugin: PluginIndex, val: T) {
        let indexer = T::indexer_mut(self);
        let sender = val.as_ref();
        indexer.0.retain(|indexed| {
            indexed.plugin != plugin || {
                let other = indexed.val.as_ref();
                sender.label != other.label || sender.group != other.group
            }
        });
        indexer.add(plugin, val);
    }
}

pub trait Sendable: 'static + Sized + Ord + AsRef<Sender> + AsMut<Sender> {
    fn indexer(indices: &Senders) -> &Indexer<Self>;
    fn indexer_mut(indices: &mut Senders) -> &mut Indexer<Self>;
    fn pad(&self, metadata: &PluginMetadata) -> Pad;
}
impl Sendable for Alias {
    fn indexer(indices: &Senders) -> &Indexer<Self> {
        &indices.aliases
    }
    fn indexer_mut(indices: &mut Senders) -> &mut Indexer<Self> {
        &mut indices.aliases
    }

    fn pad(&self, metadata: &PluginMetadata) -> Pad {
        Pad::Alias {
            plugin: metadata.name.clone(),
            label: if self.label.is_empty() {
                self.pattern.clone()
            } else {
                self.label.clone()
            },
        }
    }
}
impl Sendable for Timer {
    fn indexer(indices: &Senders) -> &Indexer<Self> {
        &indices.timers
    }
    fn indexer_mut(indices: &mut Senders) -> &mut Indexer<Self> {
        &mut indices.timers
    }

    fn pad(&self, metadata: &PluginMetadata) -> Pad {
        Pad::Timer {
            plugin: metadata.name.clone(),
            event: self.event,
        }
    }
}
impl Sendable for Trigger {
    fn indexer(indices: &Senders) -> &Indexer<Self> {
        &indices.triggers
    }
    fn indexer_mut(indices: &mut Senders) -> &mut Indexer<Self> {
        &mut indices.triggers
    }

    fn pad(&self, metadata: &PluginMetadata) -> Pad {
        Pad::Trigger {
            plugin: metadata.name.clone(),
            label: if self.label.is_empty() {
                self.pattern.clone()
            } else {
                self.label.clone()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod indexer {
        use super::*;
        #[test]
        fn test_clear() {
            let mut indexer = Indexer::new();
            indexer.extend(0, &[0, 1, 2, 3, 4]);
            indexer.clear();
            assert_eq!(indexer, Indexer::new());
        }

        #[test]
        fn test_replace() {
            let a: Vec<u8> = (0..5).collect();
            let b: Vec<u8> = (7..10).collect();
            let c: Vec<u8> = (9..13).collect();
            let mut indexer1 = Indexer::new();
            indexer1.replace(0, &a);
            indexer1.replace(1, &b);
            indexer1.replace(0, &c);
            let mut indexer2 = Indexer::new();
            indexer2.replace(1, &b);
            indexer2.replace(0, &c);
            assert_eq!(indexer1, indexer2);
        }

        #[test]
        fn test_sort() {
            let mut indexer = Indexer::new();
            indexer.extend(0, &[1, 5, 3, 2, 6]);
            indexer.extend(1, &[5, 3, 1, 2, 1]);
            indexer.sort();
            let unsorted: Vec<_> = indexer.iter().map(|(_, x)| *x).collect();
            let mut sorted = unsorted.clone();
            sorted.sort_unstable();
            assert_eq!(sorted, unsorted);
        }
    }

    mod send_match {
        use super::*;

        fn default_sendmatch() -> SendMatch<'static, 'static, Alias> {
            SendMatch {
                plugin: 0,
                pos: 0,
                sender: Box::leak(Box::default()),
                text: Cow::Borrowed(""),
                wildcards: Vec::new(),
            }
        }

        #[test]
        fn test_matchiter_repeat() {
            const COUNT: usize = 5;
            let iter = MatchIter::Repeat {
                count: COUNT,
                val: Some(default_sendmatch()),
            };
            assert_eq!(iter.size_hint().0, iter.count());
        }
    }

    mod senders {
        use super::*;

        fn basic_plugin() -> Plugin {
            Plugin {
                triggers: vec![Trigger::default()],
                aliases: vec![Alias::default()],
                timers: vec![Timer::default()],
                metadata: PluginMetadata::default(),
                callbacks: Default::default(),
                engine: mlua::Lua::new(),
            }
        }

        #[test]
        fn test_clear() {
            let mut senders = Senders::new();
            senders.extend(2, &basic_plugin());
            senders.clear();
            assert_eq!(
                (
                    senders.aliases.0.len(),
                    senders.timers.0.len(),
                    senders.triggers.0.len()
                ),
                (0, 0, 0)
            );
        }

        fn vals<T>(indexer: Indexer<T>) -> Vec<T> {
            indexer.into_iter().map(|(_, x)| x).collect()
        }

        #[test]
        fn test_extend() {
            let mut senders = Senders::new();
            let plugin = basic_plugin();
            senders.extend(2, &plugin);
            assert_eq!(
                (
                    vals(senders.aliases),
                    vals(senders.timers),
                    vals(senders.triggers)
                ),
                (plugin.aliases, plugin.timers, plugin.triggers)
            )
        }

        #[test]
        fn test_delete_all() {
            let mut senders = Senders::new();
            let mut aliases: Vec<_> = (0..11)
                .map(|i| {
                    let mut alias = Alias::default();
                    alias.send.one_shot = i % 3 == 0;
                    alias.label = i.to_string();
                    alias
                })
                .collect();
            senders.aliases.extend(0, &aliases);
            let mut delete_oneshots = Vec::new();
            for send in senders.matches::<Alias>("") {
                if send.sender.one_shot {
                    delete_oneshots.push(send.pos);
                }
            }
            senders.delete_all::<Alias>(&delete_oneshots);
            aliases.retain(|x| !x.one_shot);
            assert_eq!(vals(senders.aliases), aliases);
        }

        struct Opts<'a> {
            pattern: &'a str,
            is_regex: bool,
            repeat: bool,
            text: &'a str,
            line: &'a str,
            expect: &'a [&'a str],
        }

        fn test_match(opts: Opts) {
            let mut alias = Alias::default();
            alias.reaction.regex = Reaction::make_regex(opts.pattern, opts.is_regex).unwrap();
            alias.repeat = opts.repeat;
            alias.text = opts.text.to_owned();
            let mut senders = Senders::new();
            senders.aliases.add(1, alias);
            let outputs: Vec<_> = senders
                .matches::<Alias>(opts.line)
                .map(|send| send.text.into_owned())
                .collect();
            let expect: Vec<_> = opts.expect.iter().map(|s| s.to_owned()).collect();
            assert_eq!(outputs, expect);
        }

        #[test]
        fn matches_no_match() {
            test_match(Opts {
                pattern: "a*e",
                is_regex: true,
                repeat: false,
                text: "abcdeae",
                line: "b",
                expect: &[],
            });
        }

        #[test]
        fn matches_no_regex() {
            test_match(Opts {
                pattern: "a*e",
                is_regex: false,
                repeat: false,
                text: "0",
                line: "abcdeae",
                expect: &["0"],
            });
        }

        #[test]
        fn matches_regex() {
            test_match(Opts {
                pattern: "a.*e",
                is_regex: true,
                repeat: false,
                text: "0",
                line: "0abcdefgae",
                expect: &["0"],
            });
        }

        #[test]
        fn matches_captures() {
            test_match(Opts {
                pattern: "^(?>tiny |small |medium |large |huge |enormous )(.+) stencil(.+)$",
                is_regex: true,
                repeat: false,
                text: "a $2 $3 \\$1",
                line: "small green stencil!",
                expect: &["a !  \\green"],
            });
        }

        #[test]
        fn matches_repeat() {
            test_match(Opts {
                pattern: "a([^ ]*)e",
                is_regex: true,
                repeat: true,
                text: "f$2$1",
                line: "abcde ade ae",
                expect: &["fbcd", "fd", "f"],
            });
        }
    }
}
