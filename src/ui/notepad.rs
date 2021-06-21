use crate::binding::{Printable, RWidget};
use crate::tr::TrContext;
use cpp_core::CppBox;
use hashbrown::HashMap;
use qt_core::{AlignmentFlag, QBox, QString};
use qt_widgets::QTextEdit;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TrContext)]
pub enum Pad {
    Normal,
    MxpDebug,
    Trigger(String),
    Script(String),
    PacketDebug,
    LineInfo,
    XmlComments,
    PluginInfo(String),
}

impl Pad {
    fn title(&self) -> CppBox<QString> {
        match self {
            Self::Normal => tr!("Notepad"),
            Self::MxpDebug => tr!("MXP Messages"),
            Self::Trigger(s) => QString::from_std_str(&s),
            Self::Script(s) => QString::from_std_str(&s),
            Self::PacketDebug => tr!("Packet debug"),
            Self::LineInfo => tr!("Line Information"),
            Self::XmlComments => tr!("XML import notes"),
            Self::PluginInfo(s) => tr!("{} description", s),
        }
    }
}

#[derive(Debug, RWidget, TrContext)]
struct PadWidget {
    widget: QBox<QTextEdit>,
    kind: Pad,
}

impl PadWidget {
    fn new(suffix: &CppBox<QString>, kind: Pad) -> Self {
        unsafe {
            let widget = QTextEdit::new();
            let title = kind.title();
            title.append_q_string(suffix);
            widget.set_window_title(&title);
            Self { widget, kind }
        }
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
            suffix: QString::from_std_str(&format!(" - {}", world)),
            pads: HashMap::new(),
        }
    }
    fn get_or_create(&mut self, kind: Pad) -> &PadWidget {
        if !self.pads.contains_key(&kind) {
            self.pads
                .insert(kind.clone(), PadWidget::new(&self.suffix, kind.clone()));
        }
        &self.pads[&kind]
    }

    pub fn append<S: Printable>(&mut self, kind: Pad, align: AlignmentFlag, text: S) {
        let pad = &self.get_or_create(kind).widget;
        unsafe {
            if align == AlignmentFlag::AlignLeft {
                pad.insert_plain_text(&text.to_print());
            } else {
                let cursor = pad.text_cursor();
                let oldfmt = cursor.block_format();
                let fmt = cursor.block_format();
                fmt.set_alignment(align.into());
                cursor.insert_block_1a(&fmt);
                cursor.insert_text_1a(&text.to_print());
                cursor.insert_block_1a(&oldfmt);
            }
            pad.show();
        }
    }

    pub fn clear(&mut self, pad: Pad) {
        if let Some(pad) = self.pads.get(&pad) {
            unsafe {
                pad.widget.clear();
            }
        }
    }
}
