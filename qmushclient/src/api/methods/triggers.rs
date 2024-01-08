use std::path::PathBuf;

use mlua::{FromLua, Lua, MultiValue, Result};
use qmushclient_scripting::{Reaction, Sender, Trigger};

use crate::api::Api;
use crate::client::color::Colors;

#[api_provider]
impl Api {
    /// Implements https://www.gammon.com.au/scripts/doc.php?function=AddTrigger
    #[api("AddTrigger")]
    pub fn add_trigger(&self, lua: &Lua, vals: MultiValue) -> Result<()> {
        let mut iter = vals.into_iter();
        let label = match iter.next() {
            Some(val) => String::from_lua(val, lua)?,
            None => String::new(),
        };
        let pattern = match iter.next() {
            Some(val) => String::from_lua(val, lua)?,
            None => String::new(),
        };
        let text = match iter.next() {
            Some(val) => String::from_lua(val, lua)?,
            None => String::new(),
        };
        let flags = match iter.next() {
            Some(val) => u16::from_lua(val, lua)?,
            None => 0,
        };
        let enabled = flags & 1 != 0;
        let omit_from_log = flags & 2 != 0;
        let omit_from_output = flags & 4 != 0;
        let keep_evaluating = flags & 8 != 0;
        let ignore_case = flags & 16 != 0;
        let is_regex = flags & 32 != 0;
        let expand_variables = flags & 512 != 0;
        let replace = flags & 1024 != 0;
        let temporary = flags & 16384 != 0;
        let one_shot = flags & 32768 != 0;
        let color = match iter.next() {
            Some(val) => Colors::from_lua(val, lua)?,
            None => None,
        };
        let _wildcard = iter.next();
        let sound = match iter.next() {
            Some(val) => String::from_lua(val, lua)?,
            None => String::new(),
        };
        let script = match iter.next() {
            Some(val) => String::from_lua(val, lua)?,
            None => String::new(),
        };

        let regex = Reaction::make_regex(&pattern, is_regex)
            .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

        let send = Sender {
            label,
            script,
            text,
            enabled,
            one_shot,
            temporary,
            omit_from_output,
            omit_from_log,
            ..Default::default()
        };

        let reaction = Reaction {
            pattern,
            send,
            ignore_case,
            keep_evaluating,
            is_regex,
            expand_variables,
            regex,
            ..Default::default()
        };

        let sound = if sound.is_empty() {
            None
        } else {
            Some(PathBuf::from(sound))
        };

        let trigger = match color {
            Some(color) => Trigger {
                reaction,
                sound,
                change_foreground: true,
                foreground: color.into_owned(),
                ..Default::default()
            },
            None => Trigger {
                reaction,
                sound,
                ..Default::default()
            },
        };

        if replace {
            self.senders.borrow_mut().replace(self.index, trigger);
        } else {
            self.senders.borrow_mut().add(self.index, trigger);
        }
        Ok(())
    }
}
