use std::collections::HashMap;

use qmushclient_scripting::Pad;
#[cfg(feature = "show-special")]
use qt::core::AlignmentFlag;
use qt::gui::QTextCursor;
use qt::traits::Printable;
use qt::widgets::QTextEdit;
use tr::TrContext;

#[derive(Debug, TrContext)]
struct PadWidget {
    widget: QTextEdit,
    cursor: QTextCursor,
    kind: Pad,
}

impl PadWidget {
    fn new(kind: Pad) -> Self {
        let widget = QTextEdit::new(());
        widget.set_read_only(true);
        Self {
            cursor: widget.text_cursor(),
            widget,
            kind,
        }
    }

    fn set_name(&mut self, name: &str) {
        let title = self.kind.title(name);
        self.widget.set_window_title(title);
    }

    fn insert_header(&self) {
        let header = format!("<h2>{}</h2>", self.widget.window_title());
        self.cursor.insert_html(header);
        self.cursor.insert_block();
    }
}

#[derive(Debug, Default)]
pub struct Notepad {
    name: String,
    pads: HashMap<Pad, PadWidget>,
}

impl Notepad {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            pads: HashMap::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
        for pad in self.pads.values_mut() {
            pad.set_name(name);
        }
    }

    fn get_or_create(&mut self, kind: Pad) -> &PadWidget {
        if !self.pads.contains_key(&kind) {
            let mut pad = PadWidget::new(kind.clone());
            pad.set_name(&self.name);
            pad.insert_header();
            self.pads.insert(kind.clone(), pad);
        }
        &self.pads[&kind]
    }

    #[cfg(feature = "show-special")]
    pub fn append_aligned<S: Printable>(&mut self, kind: Pad, align: AlignmentFlag, text: S) {
        let pad = self.get_or_create(kind);
        if align == AlignmentFlag::AlignLeft {
            pad.cursor.insert_text(text);
        } else {
            pad.cursor.format.block.set_alignment(align);
            pad.cursor.insert_block();
            pad.cursor.insert_text(text);
            pad.cursor
                .format
                .block
                .set_alignment(AlignmentFlag::AlignLeft);
            pad.cursor.insert_block();
        }
        pad.widget.show();
    }

    pub fn append<S: Printable>(&mut self, kind: Pad, text: S) {
        let pad = self.get_or_create(kind);
        pad.cursor.insert_text(text);
        pad.widget.show();
    }

    pub fn replace<S: Printable>(&mut self, kind: Pad, text: S) {
        let pad = match self.pads.get(&kind) {
            Some(pad) => {
                pad.widget.clear();
                pad.insert_header();
                pad
            }
            None => self.get_or_create(kind),
        };
        pad.cursor.insert_text(text);
        pad.cursor.insert_block();
        pad.widget.show();
    }
}
