macro_rules! impl_deref {
    ($t:ty, $target:ty, $field:ident) => {
        impl std::ops::Deref for $t {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl std::ops::DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

macro_rules! impl_asref {
    ($t:ty, $target:ty) => {
        impl AsRef<$target> for $t {
            fn as_ref(&self) -> &$target {
                self
            }
        }
        impl AsMut<$target> for $t {
            fn as_mut(&mut self) -> &mut $target {
                self
            }
        }
    };
}

mod alias;
pub use alias::{Alias, AliasXml};

mod event;
pub use event::Event;

mod reaction;
pub use reaction::Reaction;

mod send_to;
pub use send_to::SendTo;

mod sender;
pub use sender::Sender;

mod timer;
pub use timer::{Timer, TimerXml};

mod trigger;
pub use trigger::{Trigger, TriggerXml};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_regex() {
        let equivalents = &[
            ("", "^$"),
            ("abc", "^abc$"),
            ("*a", "^.*a$"),
            ("^?{a*bc**#d", r"^\^\?\{a.*bc.*.*\#d$"),
        ];
        let non_regex: Vec<_> = equivalents
            .iter()
            .map(|(pat, _)| Reaction::make_regex(pat, false).unwrap())
            .collect();
        let regex: Vec<_> = equivalents
            .iter()
            .map(|(_, pat)| Reaction::make_regex(pat, true).unwrap())
            .collect();
        assert_eq!(non_regex, regex);
    }
}
