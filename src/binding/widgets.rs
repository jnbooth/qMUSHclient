use std::convert::TryFrom;
use std::error::Error as StdError;
use std::os::raw::{c_double, c_int};

use cpp_core::{CppBox, Ptr, StaticUpcast};
use qt_core::{q_event, Key, QFlags, QObject, QPtr, QString, SlotNoArgs, SlotOfBool, SlotOfInt};
use qt_gui::q_palette::ColorRole;
use qt_gui::{QFont, QKeyEvent, SlotOfQFont};
use qt_widgets::q_message_box::Icon;
use qt_widgets::*;

use super::Printable;
use crate::binding::color::HasPalette;
use crate::binding::{RColor, RFont};
use crate::enums::Enum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Browse {
    Open,
    Save,
    Directory,
}

pub trait RWidget {
    fn widget(&self) -> Ptr<QWidget>;

    fn alert<S: Printable, E: StdError + ?Sized>(&self, icon: Icon, text: S, err: &E) {
        unsafe {
            let alert = QMessageBox::from_q_widget(self.widget());
            alert.set_icon(icon);
            alert.set_text(&text.to_print());
            alert.set_informative_text(&err.to_string().to_print());
            alert.exec();
        }
    }

    fn connect_browse_button<T, F>(
        &self,
        browse: Browse,
        button: &QPtr<T>,
        field: &QPtr<QLineEdit>,
        suggest: F,
        ext: &str,
    ) where
        T: StaticUpcast<QObject> + StaticUpcast<QAbstractButton>,
        F: 'static + Fn() -> CppBox<QString>,
    {
        unsafe {
            let caption = QString::new();
            let button: QPtr<QAbstractButton> = button.static_upcast();
            let field = field.to_owned();
            let widget = self.widget();
            let filter = QString::from_std_str(ext);
            let keypress = QKeyEvent::from_type_int_q_flags_keyboard_modifier(
                q_event::Type::KeyPress,
                Key::KeyReturn.to_int(),
                QFlags::from(0),
            );
            button.clicked().connect(&SlotNoArgs::new(widget, move || {
                let mut suggested = field.text();
                if suggested.is_empty() {
                    suggested = suggest();
                }
                let filename = match browse {
                    Browse::Open => {
                        QFileDialog::get_open_file_name_4a(widget, &caption, &suggested, &filter)
                    }
                    Browse::Save => {
                        QFileDialog::get_save_file_name_4a(widget, &caption, &suggested, &filter)
                    }
                    Browse::Directory => {
                        QFileDialog::get_existing_directory_3a(widget, &caption, &suggested)
                    }
                };
                if !filename.is_empty() {
                    field.set_text(&filename);
                    field.event(&keypress);
                }
            }));
        }
    }

    fn connect_form<T, Q, F>(&self, field: &QPtr<Q>, initial: &T, set: F)
    where
        Q: RForm<T>,
        F: 'static + FnMut(T),
    {
        unsafe {
            RForm::connect(field, self.widget(), initial, set);
        }
    }
}

pub trait RDialog<Response: From<c_int>>: RWidget {
    fn exec(&self) -> Response;
}

pub trait RForm<T>: StaticUpcast<QObject> {
    /// # Safety
    ///
    /// `this` must be valid.
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &T, set: F)
    where
        F: 'static + FnMut(T);
}

impl RForm<String> for QLineEdit {
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &String, mut set: F)
    where
        F: 'static + FnMut(String),
    {
        unsafe {
            let this = this.clone();
            this.set_text(&QString::from_std_str(initial));
            this.editing_finished()
                .connect(&SlotNoArgs::new(parent, move || {
                    set(this.text().to_std_string());
                }));
        }
    }
}

impl RForm<String> for QPlainTextEdit {
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &String, mut set: F)
    where
        F: 'static + FnMut(String),
    {
        unsafe {
            let this = this.clone();
            this.set_plain_text(&QString::from_std_str(initial));
            this.text_changed()
                .connect(&SlotNoArgs::new(parent, move || {
                    set(this.to_plain_text().trimmed().to_std_string());
                }));
        }
    }
}

impl RForm<bool> for QCheckBox {
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &bool, set: F)
    where
        F: 'static + FnMut(bool),
    {
        unsafe {
            this.set_checked(*initial);
            this.toggled().connect(&SlotOfBool::new(parent, set));
        }
    }
}

impl RForm<bool> for QRadioButton {
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &bool, set: F)
    where
        F: 'static + FnMut(bool),
    {
        unsafe {
            this.set_checked(*initial);
            this.toggled().connect(&SlotOfBool::new(parent, set));
        }
    }
}

fn enum_from_index<E: Enum>(i: usize) -> Option<E> {
    E::enumerate().find(|e| e.index() == i)
}

impl<E: Enum> RForm<Option<E>> for QComboBox {
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &Option<E>, mut set: F)
    where
        F: 'static + FnMut(Option<E>),
    {
        unsafe {
            this.set_current_index(match initial {
                None => 0,
                Some(i) => i.index() as c_int + 1,
            });
            this.current_index_changed()
                .connect(&SlotOfInt::new(parent, move |index| {
                    set(usize::try_from(index - 1).ok().and_then(enum_from_index));
                }));
        }
    }
}

impl<E: Enum> RForm<E> for QComboBox {
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &E, mut set: F)
    where
        F: 'static + FnMut(E),
    {
        unsafe {
            this.set_current_index(initial.index() as c_int);
            this.current_index_changed()
                .connect(&SlotOfInt::new(parent, move |index| {
                    if let Some(val) = usize::try_from(index).ok().and_then(enum_from_index) {
                        set(val);
                    }
                }));
        }
    }
}

impl RForm<RFont> for QFontComboBox {
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &RFont, mut set: F)
    where
        F: 'static + FnMut(RFont),
    {
        unsafe {
            this.set_current_font(initial);
            this.current_font_changed()
                .connect(&SlotOfQFont::new(parent, move |font| {
                    set(RFont::from(QFont::new_copy(font)));
                }));
        }
    }
}

impl RForm<RColor> for QPushButton {
    unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &RColor, mut set: F)
    where
        F: 'static + FnMut(RColor),
    {
        unsafe {
            let this = this.clone();
            this.set_maximum_width(this.height());
            this.set_palette_color(ColorRole::Button, initial);
            this.clicked().connect(&SlotNoArgs::new(parent, move || {
                if let Some(color) = this.palette_color(ColorRole::Button).pick(this.clone()) {
                    this.set_palette_color(ColorRole::Button, &color);
                    set(color);
                }
            }));
        }
    }
}

macro_rules! impl_int {
    ($t:ty) => {
        impl RForm<$t> for QSpinBox {
            unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &$t, mut set: F)
            where
                F: 'static + FnMut($t),
            {
                unsafe {
                    let this = this.clone();
                    this.set_value(*initial as c_int);
                    this.editing_finished()
                        .connect(&SlotNoArgs::new(parent, move || {
                            if let Ok(val) = <$t>::try_from(this.value()) {
                                set(val);
                            }
                        }));
                }
            }
        }

        impl RForm<$t> for QDoubleSpinBox {
            unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &$t, mut set: F)
            where
                F: 'static + FnMut($t),
            {
                unsafe {
                    let this = this.clone();
                    this.set_value(*initial as c_double / 1000.0);
                    this.editing_finished()
                        .connect(&SlotNoArgs::new(parent, move || {
                            set((this.value() * 1000.0) as $t);
                        }));
                }
            }
        }
    };
}

macro_rules! impl_float {
    ($t:ty) => {
        impl RForm<$t> for QDoubleSpinBox {
            unsafe fn connect<F>(this: &QPtr<Self>, parent: Ptr<QWidget>, initial: &$t, mut set: F)
            where
                F: 'static + FnMut($t),
            {
                unsafe {
                    let this = this.clone();
                    this.set_value(*initial as c_double);
                    this.editing_finished()
                        .connect(&SlotNoArgs::new(parent, move || {
                            set(this.value() as $t);
                        }));
                }
            }
        }
    };
}

impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(usize);
impl_int!(u128);
impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(isize);
impl_int!(i128);
impl_float!(f32);
impl_float!(f64);
