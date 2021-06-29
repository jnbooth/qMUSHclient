use crate::binding::color::{Colored, RColor, RColorPair};
use crate::binding::text::Cursor;
use crate::binding::{Printable, RIODevice, RWidget};
use crate::client::color::Colors;
use crate::tr::TrContext;
use crate::ui::{Notepad, Pad};
use crate::world::World;
use hashbrown::HashMap;
use iter_chunks::IterChunks;
use mlua::{FromLua, MultiValue, UserData, UserDataMethods};
use qt_core::{AlignmentFlag, QPtr};
use qt_network::QTcpSocket;
use qt_widgets::QTextBrowser;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

#[derive(RWidget, TrContext)]
pub struct Api {
    widget: QPtr<QTextBrowser>,
    cursor: Cursor,
    socket: RIODevice<QTcpSocket>,
    world: Rc<World>,
    notepad: Rc<RefCell<Notepad>>,
    custom_colors: HashMap<String, RColorPair>,
}

impl Clone for Api {
    fn clone(&self) -> Self {
        let widget = self.widget.clone();
        unsafe {
            Self {
                cursor: Cursor::get(&widget),
                widget,
                socket: self.socket.clone(),
                world: self.world.clone(),
                notepad: self.notepad.clone(),
                custom_colors: self.custom_colors.clone(),
            }
        }
    }
}

impl UserData for Api {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("Note", |_, this, s: String| {
            this.note(s);
            Ok(())
        });
        methods.add_method("ColourNote", |lua, this, vals: MultiValue| {
            for [fg, bg, s] in vals.into_iter().chunks() {
                let s = String::from_lua(s, lua)?;
                let fg = Colors::from_lua(fg, lua)?;
                let bg = Colors::from_lua(bg, lua)?;
                this.color_note(s, fg, bg)
            }
            Ok(())
        });
    }
}

impl Api {
    /// # Safety
    ///
    /// `socket` must be valid.
    pub unsafe fn new(
        widget: QPtr<QTextBrowser>,
        socket: RIODevice<QTcpSocket>,
        world: Rc<World>,
        notepad: Rc<RefCell<Notepad>>,
    ) -> Self {
        let cursor = unsafe { Cursor::get(&widget) };
        cursor
            .format
            .text
            .set_foreground_color(&world.note_text_color);
        Self {
            cursor,
            widget,
            socket,
            notepad,
            custom_colors: world.custom_color_map(),
            world,
        }
    }

    pub fn set_world(&mut self, world: Rc<World>) {
        self.custom_colors = world.custom_color_map();
        self.world = world;
    }

    pub fn append_to_notepad<S: Printable>(&self, title: String, s: S) {
        self.notepad
            .borrow_mut()
            .append(Pad::Script(title), AlignmentFlag::AlignLeft, s);
    }

    pub fn note<S: Printable>(&self, s: S) {
        self.cursor.insert_block();
        self.cursor.insert_text(s);
    }

    pub fn color_note<S, B>(&self, s: S, fg: Option<B>, bg: Option<B>)
    where
        S: Printable,
        B: Borrow<RColor>,
    {
        self.cursor.insert_block();
        self.cursor.insert_text_colored(s, fg, bg);
    }

    fn send<S: AsRef<[u8]>>(&self, buf: S) -> io::Result<()> {
        let buf = buf.as_ref();
        self.socket.io().write_all(buf)?;
        Ok(())
    }

    pub fn send_packet(&self, data: &[u8]) {
        if !self.socket.writable() {
            eprintln!("Tried to send over a closed connection: {:?}", data);
        } else if let Err(e) = self.send(data) {
            eprintln!("Error sending packet {:?}: {}", data, e);
        }
    }
}
