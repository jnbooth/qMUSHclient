use std::borrow::Borrow;
use std::cell::RefCell;
use std::io;
use std::os::raw::c_double;
use std::rc::Rc;

use hashbrown::HashMap;
use mlua::{UserData, UserDataMethods};
use qt_gui::q_text_cursor::MoveOperation;
use qt_network::QTcpSocket;

use crate::binding::color::{Colored, RColor, RColorPair};
use crate::binding::text::RTextCursor;
use crate::binding::widgets::{RLineEdit, RTextBrowser};
use crate::binding::{Printable, RIODevice, RSettings};
use crate::constants::Paths;
use crate::script::{PluginIndex, PluginMetadata, Senders};
use crate::tr::TrContext;
use crate::ui::{Notepad, Pad};
use crate::world::World;

mod methods;
mod state;
pub use state::ApiState;

#[derive(TrContext)]
pub struct Api {
    output: RTextBrowser,
    input: RLineEdit,
    cursor: RTextCursor,
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
        methods::provide_api(methods);
    }
}

impl Api {
    /// # Safety
    ///
    /// `output` and `input` must be valid and non-null.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        output: RTextBrowser,
        input: RLineEdit,
        socket: RIODevice<QTcpSocket>,
        world: Rc<World>,
        state: Rc<ApiState>,
        notepad: Rc<RefCell<Notepad>>,
        senders: Rc<RefCell<Senders>>,
        paths: &'static Paths,
    ) -> Self {
        let cursor = output.text_cursor();
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
            cursor: output.text_cursor(),
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
        if self.input.text().is_empty() {
            self.input.set_text(text);
        }
    }

    fn scroll_to_bottom(&self) {
        let scrollbar = self.output.vertical_scroll_bar();
        scrollbar.set_value(scrollbar.maximum());
    }

    pub fn append_to_notepad<S: Printable>(&self, title: String, text: S) {
        self.notepad.borrow_mut().append(Pad::Script(title), text);
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

    pub fn note<S: Printable>(&self, text: S) {
        self.cursor.move_position(MoveOperation::End, 1);
        self.cursor.insert_block();
        self.cursor.insert_text(text);
        self.scroll_to_bottom();
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

    pub fn send<S: AsRef<[u8]>>(&self, text: S) -> io::Result<()> {
        self.socket.write_all(text.as_ref())
    }

    pub fn _send_packet(&self, data: &[u8]) {
        if !self.socket.writable() {
            eprintln!("Tried to send over a closed connection: {:?}", data);
        } else if let Err(e) = self.send(data) {
            eprintln!("Error sending packet {:?}: {}", data, e);
        }
    }
}
