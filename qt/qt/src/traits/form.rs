use std::os::raw::{c_double, c_int};
use std::path::PathBuf;

use cpp_core::Ptr;
use enumeration::Enum;
use qt_core::{QPtr, QString, SlotNoArgs, SlotOfBool, SlotOfInt};
use qt_gui::q_palette::ColorRole;
use qt_gui::SlotOfQFont;
use qt_widgets as q;

use crate::gui::{QColor, QFont};
use crate::traits::HasPalette;

pub trait QForm<T> {
    /// # Safety
    ///
    /// `self` must be valid.
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &T, set: F)
    where
        F: 'static + Clone + FnMut(T);
}

impl QForm<Option<PathBuf>> for QPtr<q::QLineEdit> {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &Option<PathBuf>, mut set: F)
    where
        F: 'static + Clone + FnMut(Option<PathBuf>),
    {
        unsafe {
            self.set_text(&QString::from_std_str(
                initial.as_ref().and_then(|x| x.to_str()).unwrap_or(""),
            ));
            let this = self.clone();
            self.editing_finished()
                .connect(&SlotNoArgs::new(parent, move || {
                    let s = this.text();
                    set(if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s.to_std_string()))
                    })
                }));
        }
    }
}

impl QForm<String> for QPtr<q::QLineEdit> {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &String, mut set: F)
    where
        F: 'static + Clone + FnMut(String),
    {
        unsafe {
            self.set_text(&QString::from_std_str(initial));
            let this = self.clone();
            self.editing_finished()
                .connect(&SlotNoArgs::new(parent, move || {
                    set(this.text().to_std_string());
                }));
        }
    }
}

impl QForm<String> for QPtr<q::QPlainTextEdit> {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &String, mut set: F)
    where
        F: 'static + Clone + FnMut(String),
    {
        unsafe {
            self.set_plain_text(&QString::from_std_str(initial));
            let this = self.clone();
            self.text_changed()
                .connect(&SlotNoArgs::new(parent, move || {
                    set(this.to_plain_text().trimmed().to_std_string());
                }));
        }
    }
}

impl QForm<bool> for QPtr<q::QCheckBox> {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &bool, set: F)
    where
        F: 'static + Clone + FnMut(bool),
    {
        unsafe {
            self.set_checked(*initial);
            self.toggled().connect(&SlotOfBool::new(parent, set));
        }
    }
}

impl QForm<bool> for QPtr<q::QRadioButton> {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &bool, set: F)
    where
        F: 'static + Clone + FnMut(bool),
    {
        unsafe {
            self.set_checked(*initial);
            self.toggled().connect(&SlotOfBool::new(parent, set));
        }
    }
}

impl<E: Enum> QForm<E> for QPtr<q::QComboBox> {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &E, mut set: F)
    where
        F: 'static + Clone + FnMut(E),
    {
        unsafe {
            debug_assert!(self.count() == E::SIZE as c_int);
            self.set_current_index(initial.index() as c_int);
            self.current_index_changed()
                .connect(&SlotOfInt::new(parent, move |index| {
                    if let Some(val) = usize::try_from(index).ok().and_then(E::from_index) {
                        set(val);
                    }
                }));
        }
    }
}

impl<E: Enum + 'static, const N: usize> QForm<E> for [QPtr<q::QRadioButton>; N] {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &E, set: F)
    where
        F: 'static + Clone + FnMut(E),
    {
        unsafe {
            debug_assert!(N == E::SIZE);
            for (e, field) in E::enumerate(..).zip(self.iter()) {
                field.set_checked(e == *initial);
                let mut set = set.clone();
                field
                    .toggled()
                    .connect(&SlotOfBool::new(parent, move |checked| {
                        if checked {
                            set(e);
                        }
                    }));
            }
        }
    }
}

impl QForm<QFont> for QPtr<q::QFontComboBox> {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &QFont, mut set: F)
    where
        F: 'static + Clone + FnMut(QFont),
    {
        unsafe {
            self.set_current_font(initial);
            self.current_font_changed()
                .connect(&SlotOfQFont::new(parent, move |font| {
                    set(QFont::from(qt_gui::QFont::new_copy(font)));
                }));
        }
    }
}

impl QForm<QColor> for QPtr<q::QPushButton> {
    unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &QColor, mut set: F)
    where
        F: 'static + Clone + FnMut(QColor),
    {
        unsafe {
            self.set_maximum_width(self.height());
            self.set_palette_color(ColorRole::Button, initial);
            let this = self.clone();
            self.clicked().connect(&SlotNoArgs::new(parent, move || {
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
        impl QForm<$t> for QPtr<q::QSpinBox> {
            unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &$t, mut set: F)
            where
                F: 'static + Clone + FnMut($t),
            {
                unsafe {
                    self.set_value(*initial as c_int);
                    let this = self.clone();
                    self.editing_finished()
                        .connect(&SlotNoArgs::new(parent, move || {
                            if let Ok(val) = <$t>::try_from(this.value()) {
                                set(val);
                            }
                        }));
                }
            }
        }

        impl QForm<$t> for QPtr<q::QDoubleSpinBox> {
            unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &$t, mut set: F)
            where
                F: 'static + Clone + FnMut($t),
            {
                unsafe {
                    self.set_value(*initial as c_double / 1000.0);
                    let this = self.clone();
                    self.editing_finished()
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
        impl QForm<$t> for QPtr<q::QDoubleSpinBox> {
            unsafe fn connect<F>(&self, parent: Ptr<q::QWidget>, initial: &$t, mut set: F)
            where
                F: 'static + Clone + FnMut($t),
            {
                unsafe {
                    self.set_value(*initial as c_double);
                    let this = self.clone();
                    self.editing_finished()
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
