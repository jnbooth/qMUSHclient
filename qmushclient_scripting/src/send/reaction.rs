use serde::{Deserialize, Serialize};

use super::sender::Sender;
use crate::constants::DEFAULT_SEQUENCE;
use crate::regex::{Regex, RegexError};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Reaction {
    // Note: this is at the top for Ord-deriving purposes.
    pub sequence: i16,
    pub pattern: String,
    pub send: Sender,

    pub ignore_case: bool,
    pub keep_evaluating: bool,
    pub is_regex: bool,
    pub expand_variables: bool,
    pub repeat: bool,

    pub regex: Regex,
}

impl_deref!(Reaction, Sender, send);
impl_asref!(Reaction, Sender);

impl Default for Reaction {
    fn default() -> Self {
        Self {
            sequence: DEFAULT_SEQUENCE,
            pattern: String::new(),
            send: Sender::default(),
            ignore_case: false,
            keep_evaluating: false,
            is_regex: false,
            expand_variables: false,
            repeat: false,
            regex: Regex::new("^$").unwrap(),
        }
    }
}

impl Reaction {
    pub fn make_regex(pattern: &str, is_regex: bool) -> Result<Regex, RegexError> {
        #[rustfmt::skip]
        fn is_special(c: char) -> bool {
            matches!(c, '\\'|'.'|'+'|'*'|'?'|'('|')'|'|'|'['|']'|'{'|'}'|'^'|'$'|'#')
        }
        if is_regex {
            Regex::new(pattern)
        } else {
            let extra_len = pattern.bytes().filter(|&b| is_special(b as char)).count();
            let mut buf = String::with_capacity(pattern.len() + extra_len + 2);
            buf.push('^');
            for c in pattern.chars() {
                if c == '*' {
                    buf.push('.');
                } else if is_special(c) {
                    buf.push('\\');
                }
                buf.push(c);
            }
            buf.push('$');
            Regex::new(&buf)
        }
    }
}
