use std::collections::HashMap;

use cpp_core::CppBox;
#[cfg(feature = "show-special")]
use qt_core::AlignmentFlag;
use qt_core::{QBox, QString};
use qt_widgets::QTextEdit;

use crate::binding::text::RTextCursor;
use crate::binding::{Printable, Widget};
use crate::script::Event;
use crate::tr::TrContext;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TrContext)]
pub enum Pad {
    Script(String),
    Alias {
        plugin: String,
        label: String,
    },
    Timer {
        plugin: String,
        event: Event,
    },
    Trigger {
        plugin: String,
        label: String,
    },
    #[cfg(feature = "show-special")]
    PacketDebug,
}

impl Pad {
    fn title(&self) -> CppBox<QString> {
        match self {
            Self::Script(s) => QString::from_std_str(s),
            Self::Alias { plugin, label } => tr!("Alias: {} ({})", label, plugin),
            Self::Timer { plugin, event } => tr!("Timer: {} ({})", event, plugin),
            Self::Trigger { plugin, label } => tr!("Trigger: {} ({})", label, plugin),
            //Self::Trigger(s) => QString::from_std_str(&s),
            #[cfg(feature = "show-special")]
            Self::PacketDebug => tr!("Packet debug"),
        }
    }
}

#[derive(Debug, Widget, TrContext)]
struct PadWidget {
    widget: QBox<QTextEdit>,
    cursor: RTextCursor,
    kind: Pad,
}

impl PadWidget {
    fn new(kind: Pad) -> Self {
        unsafe {
            let widget = QTextEdit::new();
            widget.set_read_only(true);
            Self {
                cursor: RTextCursor::get(&widget),
                widget,
                kind,
            }
        }
    }

    fn retitle(&mut self, suffix: &CppBox<QString>) {
        unsafe {
            let title = self.kind.title();
            title.append_q_string(suffix);
            self.widget.set_window_title(&title);
        }
    }

    fn insert_header(&self) {
        let header = unsafe {
            QString::from_std_str("<h2>%1</h2>").arg_q_string(&self.widget.window_title())
        };
        self.cursor.insert_html(header);
        self.cursor.insert_block();
    }
}

#[derive(Debug)]
pub struct Notepad {
    suffix: CppBox<QString>,
    pads: HashMap<Pad, PadWidget>,
}

impl Notepad {
    pub fn new(world: &str) -> Self {
        Self {
            suffix: QString::from_std_str(format!(" - {}", world)),
            pads: HashMap::new(),
        }
    }

    pub fn retitle(&mut self, title: &str) {
        self.suffix = QString::from_std_str(title);
        for pad in self.pads.values_mut() {
            pad.retitle(&self.suffix);
        }
    }

    fn get_or_create(&mut self, kind: Pad) -> &PadWidget {
        if !self.pads.contains_key(&kind) {
            let mut pad = PadWidget::new(kind.clone());
            pad.retitle(&self.suffix);
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
        unsafe {
            pad.widget.show();
        }
    }

    pub fn append<S: Printable>(&mut self, kind: Pad, text: S) {
        let pad = self.get_or_create(kind);
        pad.cursor.insert_text(text);
        unsafe {
            pad.widget.show();
        }
    }

    pub fn replace<S: Printable>(&mut self, kind: Pad, text: S) {
        let pad = match self.pads.get(&kind) {
            Some(pad) => unsafe {
                pad.widget.clear();
                pad.insert_header();
                pad
            },
            None => self.get_or_create(kind),
        };
        pad.cursor.insert_text(text);
        pad.cursor.insert_block();
        unsafe {
            pad.widget.show();
        }
    }
}
