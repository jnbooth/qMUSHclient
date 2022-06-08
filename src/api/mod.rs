use std::borrow::Borrow;
use std::cell::RefCell;
use std::io;
use std::os::raw::c_double;
use std::path::PathBuf;
use std::rc::Rc;

use hashbrown::HashMap;
use iter_chunks::IterChunks;
use mlua::{
    FromLua, Lua, MultiValue, Result, String as LString, ToLua, UserData, UserDataMethods, Value,
};
use qt_core::QPtr;
use qt_gui::q_text_cursor::MoveOperation;
use qt_network::QTcpSocket;
use qt_widgets::{QLineEdit, QTextBrowser};

use crate::binding::color::{Colored, RColor, RColorPair};
use crate::binding::text::{Cursor, ScrollBar};
use crate::binding::{Printable, RIODevice, RSettings};
use crate::client::color::Colors;
use crate::constants::Paths;
use crate::script::{PluginIndex, PluginMetadata, Reaction, Sender, Senders, Trigger};
use crate::tr::TrContext;
use crate::ui::{Notepad, Pad};
use crate::world::World;

mod getinfo;
mod state;
pub use state::ApiState;

#[derive(TrContext)]
pub struct Api {
    output: QPtr<QTextBrowser>,
    input: QPtr<QLineEdit>,
    cursor: Cursor,
    socket: RIODevice<QTcpSocket>,
    world: Rc<World>,
    state: Rc<ApiState>,
    pub notepad: Rc<RefCell<Notepad>>,
    pub senders: Rc<RefCell<Senders>>,
    custom_colors: HashMap<String, RColorPair>,
    variables: RefCell<HashMap<String, String>>,
    variables_key: String,
    index: PluginIndex,
    paths: &'static Paths,
}

impl Drop for Api {
    fn drop(&mut self) {
        self.save_variables();
    }
}

impl UserData for Api {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        self::provide_api(methods);
        getinfo::provide_api(methods);
    }
}

#[api_provider]
impl Api {
    /// # Safety
    ///
    /// `output` and `input` must be valid and non-null.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn new(
        output: QPtr<QTextBrowser>,
        input: QPtr<QLineEdit>,
        socket: RIODevice<QTcpSocket>,
        world: Rc<World>,
        state: Rc<ApiState>,
        notepad: Rc<RefCell<Notepad>>,
        senders: Rc<RefCell<Senders>>,
        paths: &'static Paths,
    ) -> Self {
        // SAFETY: `output` is valid.
        let cursor = Cursor::get(&output);
        cursor
            .format
            .text
            .set_foreground_color(&world.note_text_color);
        Self {
            cursor,
            output,
            input,
            socket,
            notepad,
            custom_colors: world.custom_color_map(),
            world,
            state,
            senders,
            variables: RefCell::new(HashMap::new()),
            variables_key: String::new(),
            index: 0,
            paths,
        }
    }

    pub fn clone_with(&self, index: PluginIndex, metadata: &PluginMetadata) -> Self {
        let variables_key = format!("vars-{:?}-{:?}", self.world.name, metadata.id);
        let output = self.output.clone();
        Self {
            cursor: Cursor::get(&output),
            output,
            input: self.input.clone(),
            socket: self.socket.clone(),
            world: self.world.clone(),
            state: self.state.clone(),
            notepad: self.notepad.clone(),
            senders: self.senders.clone(),
            custom_colors: self.custom_colors.clone(),
            variables: RefCell::new(
                RSettings::default()
                    .get(&variables_key)
                    .unwrap_or_else(|_| HashMap::new()),
            ),
            variables_key,
            index,
            paths: self.paths,
        }
    }

    pub fn set_index(&mut self, index: PluginIndex) {
        self.index = index;
    }

    #[api("GetVariable")]
    pub fn get_variable<'lua>(&self, lua: &'lua Lua, key: String) -> Result<Value<'lua>> {
        match self.variables.borrow().get(&key) {
            Some(val) => val.as_str().to_lua(lua),
            None => Ok(Value::Nil),
        }
    }

    #[api("SetVariable")]
    pub fn set_variable(&self, key: String, val: String) {
        self.variables.borrow_mut().insert(key, val);
    }

    pub fn save_variables(&mut self) {
        if !self.variables_key.is_empty() {
            RSettings::default().set(&self.variables_key, &*self.variables.borrow());
        }
    }

    pub fn set_world(&mut self, world: Rc<World>) {
        self.custom_colors = world.custom_color_map();
        self.world = world;
    }

    pub fn set_spacing(&mut self, spacing: c_double) {
        self.cursor.format.block.set_line_height(spacing);
    }

    /// If the [`QLineEdit`] input field if is empty, inserts the given text into it.
    pub fn command<S: Printable>(&self, text: S) {
        unsafe {
            if self.input.text().is_empty() {
                self.input.set_text(&text.to_print());
            }
        }
    }

    fn scroll_to_bottom(&self) {
        let scrollbar = ScrollBar::get_vertical(&self.output);
        scrollbar.set_value(scrollbar.maximum());
    }

    pub fn append_to_notepad<S: Printable>(&self, title: String, text: S) {
        self.notepad.borrow_mut().append(Pad::Script(title), text);
    }

    #[api("AppendToNotepad")]
    pub fn append_to_notepad_api(&self, title: String, text: LString) {
        self.append_to_notepad(title, text);
    }

    pub fn echo<S: Printable>(&self, text: S) {
        let world = &*self.world;
        if world.display_my_input {
            self.cursor.move_position(MoveOperation::End, 1);
            if !world.keep_commands_on_same_line && !self.cursor.at_block_start() {
                self.cursor.insert_block();
            }
            let echo_colors = &world.echo_colors;
            self.cursor.insert_text_colored(
                text,
                Some(&echo_colors.foreground),
                Some(&echo_colors.background),
            );
            self.scroll_to_bottom();
        }
    }

    //#[api("Note")]
    pub fn note<S: Printable>(&self, text: S) {
        self.cursor.move_position(MoveOperation::End, 1);
        self.cursor.insert_block();
        self.cursor.insert_text(text);
        self.scroll_to_bottom();
    }

    #[api("Note")]
    pub fn note_api(&self, text: LString) {
        self.note(text);
    }

    pub fn color_note<S, B>(&self, text: S, fg: Option<B>, bg: Option<B>)
    where
        S: Printable,
        B: Borrow<RColor>,
    {
        self.cursor.move_position(MoveOperation::End, 1);
        self.cursor.insert_block();
        self.cursor.insert_text_colored(text, fg, bg);
        self.scroll_to_bottom();
    }

    #[api("ColourNote")]
    pub fn color_note_api(&self, lua: &Lua, vals: MultiValue) -> Result<()> {
        for [fg, bg, s] in vals.into_iter().chunks() {
            let s = String::from_lua(s, lua)?;
            let fg = Colors::from_lua(fg, lua)?;
            let bg = Colors::from_lua(bg, lua)?;
            self.color_note(s, fg, bg);
        }
        Ok(())
    }

    pub fn send<S: AsRef<[u8]>>(&self, text: S) -> io::Result<()> {
        self.socket.write_all(text.as_ref())
    }

    #[api("Send")]
    pub fn send_api(&self, text: LString) {
        let mut bytes = text.as_bytes_with_nul().to_vec();
        let nullpos = bytes.len() - 1;
        self.echo(&bytes[..nullpos]);
        bytes[nullpos] = b'\n';
        let _ = self.send(&bytes);
    }

    pub fn _send_packet(&self, data: &[u8]) {
        if !self.socket.writable() {
            eprintln!("Tried to send over a closed connection: {:?}", data);
        } else if let Err(e) = self.send(data) {
            eprintln!("Error sending packet {:?}: {}", data, e);
        }
    }

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
