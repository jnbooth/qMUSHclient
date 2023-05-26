use itertools::Itertools;
use mlua::{FromLua, Lua, MultiValue, Result, String as LString};

use crate::api::Api;
use crate::client::color::Colors;

#[api_provider]
impl Api {
    #[api("AppendToNotepad")]
    pub fn append_to_notepad_api(&self, title: String, text: LString) {
        self.append_to_notepad(title, text);
    }

    #[api("Note")]
    pub fn note_api(&self, text: LString) {
        self.note(text);
    }

    #[api("ColourNote")]
    pub fn color_note_api(&self, lua: &Lua, vals: MultiValue) -> Result<()> {
        for (fg, bg, s) in vals.into_iter().tuples() {
            let s = String::from_lua(s, lua)?;
            let fg = Colors::from_lua(fg, lua)?;
            let bg = Colors::from_lua(bg, lua)?;
            self.color_note(s, fg, bg);
        }
        Ok(())
    }

    #[api("Send")]
    pub fn send_api(&self, text: LString) {
        let mut bytes = text.as_bytes_with_nul().to_vec();
        let nullpos = bytes.len() - 1;
        self.echo(&bytes[..nullpos]);
        bytes[nullpos] = b'\n';
        let _ = self.send(&bytes);
    }
}
