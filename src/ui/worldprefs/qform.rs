use std::convert::TryFrom;
use std::os::raw::{c_double, c_int};

use cpp_core::StaticUpcast;
use qt_core::{CheckState, QBox, QObject, QPtr, QString, SlotNoArgs};
use qt_gui::q_palette::ColorRole;
use qt_widgets::*;

use crate::binding::color::{HasPalette, RColor};
use crate::binding::RFont;
use crate::enums::Enum;

fn enum_from_index<E: Enum>(i: usize) -> Option<E> {
    E::enumerate().find(|e| e.index() == i)
}

pub trait QForm<T>: StaticUpcast<QObject> {
    unsafe fn get_rust(this: QPtr<Self>) -> T;
    unsafe fn connect_rust(this: QPtr<Self>, t: &T, receiver: QBox<SlotNoArgs>);
}

impl QForm<String> for QLineEdit {
    unsafe fn get_rust(this: QPtr<Self>) -> String {
        unsafe { this.text().trimmed() }.to_std_string()
    }

    unsafe fn connect_rust(this: QPtr<Self>, t: &String, receiver: QBox<SlotNoArgs>) {
        unsafe {
            this.set_text(&QString::from_std_str(t));
            this.editing_finished().connect(&receiver);
        }
    }
}

impl QForm<String> for QPlainTextEdit {
    unsafe fn get_rust(this: QPtr<Self>) -> String {
        unsafe { this.to_plain_text().trimmed() }.to_std_string()
    }

    unsafe fn connect_rust(this: QPtr<Self>, t: &String, receiver: QBox<SlotNoArgs>) {
        unsafe {
            this.set_plain_text(&QString::from_std_str(t));
            this.text_changed().connect(&receiver);
        }
    }
}

impl QForm<bool> for QCheckBox {
    unsafe fn get_rust(this: QPtr<Self>) -> bool {
        unsafe { this.check_state() == CheckState::Checked }
    }

    unsafe fn connect_rust(this: QPtr<Self>, t: &bool, receiver: QBox<SlotNoArgs>) {
        unsafe {
            this.set_check_state(if *t {
                CheckState::Checked
            } else {
                CheckState::Unchecked
            });
            this.state_changed().connect(&receiver);
        }
    }
}

impl QForm<bool> for QRadioButton {
    unsafe fn get_rust(this: QPtr<Self>) -> bool {
        unsafe { this.is_checked() }
    }

    unsafe fn connect_rust(this: QPtr<Self>, t: &bool, receiver: QBox<SlotNoArgs>) {
        unsafe {
            this.set_checked(*t);
            this.toggled().connect(&receiver);
        }
    }
}

impl<E: Enum> QForm<Option<E>> for QComboBox {
    unsafe fn get_rust(this: QPtr<Self>) -> Option<E> {
        usize::try_from(unsafe { this.current_index() } - 1)
            .ok()
            .and_then(enum_from_index)
    }

    unsafe fn connect_rust(this: QPtr<Self>, e: &Option<E>, receiver: QBox<SlotNoArgs>) {
        unsafe {
            this.set_current_index(match e {
                None => 0,
                Some(i) => i.index() as c_int + 1,
            });
            this.current_index_changed().connect(&receiver);
        }
    }
}

impl<E: Enum> QForm<E> for QComboBox {
    unsafe fn get_rust(this: QPtr<Self>) -> E {
        usize::try_from(unsafe { this.current_index() })
            .ok()
            .and_then(enum_from_index)
            .expect("Enum out of range")
    }

    unsafe fn connect_rust(this: QPtr<Self>, e: &E, receiver: QBox<SlotNoArgs>) {
        unsafe {
            this.set_current_index(e.index() as c_int);
            this.current_index_changed().connect(&receiver);
        }
    }
}

impl QForm<RFont> for QFontComboBox {
    unsafe fn get_rust(this: QPtr<Self>) -> RFont {
        RFont::from(unsafe { this.current_font() })
    }

    unsafe fn connect_rust(this: QPtr<Self>, t: &RFont, receiver: QBox<SlotNoArgs>) {
        unsafe {
            this.set_current_font(t);
            this.current_font_changed().connect(&receiver);
        }
    }
}

impl QForm<RColor> for QPushButton {
    unsafe fn get_rust(this: QPtr<Self>) -> RColor {
        this.palette_color(ColorRole::Button)
    }

    unsafe fn connect_rust(this: QPtr<Self>, t: &RColor, receiver: QBox<SlotNoArgs>) {
        unsafe {
            this.set_maximum_width(this.height());
            this.set_palette_color(ColorRole::Button, t);
            this.clicked()
                .connect(&SlotNoArgs::new(this.clone(), move || {
                    if let Some(color) = this.palette_color(ColorRole::Button).pick(this.clone()) {
                        this.set_palette_color(ColorRole::Button, &color);
                        receiver.slot();
                    }
                }));
        }
    }
}

macro_rules! impl_int {
    ($t: ty) => {
        impl QForm<$t> for QSpinBox {
            unsafe fn get_rust(this: QPtr<Self>) -> $t {
                <$t>::try_from(unsafe { this.value() }).unwrap_or(<$t>::MAX)
            }

            unsafe fn connect_rust(this: QPtr<Self>, t: &$t, receiver: QBox<SlotNoArgs>) {
                unsafe {
                    this.set_value(*t as c_int);
                    this.editing_finished().connect(&receiver);
                }
            }
        }
        impl QForm<$t> for QDoubleSpinBox {
            unsafe fn get_rust(this: QPtr<Self>) -> $t {
                (unsafe { this.value() } * 1000.0) as $t // milli-
            }

            unsafe fn connect_rust(this: QPtr<Self>, t: &$t, receiver: QBox<SlotNoArgs>) {
                unsafe {
                    this.set_value(*t as c_double / 1000.0);
                    this.editing_finished().connect(&receiver);
                }
            }
        }
    };
}
macro_rules! impl_float {
    ($t: ty) => {
        impl QForm<$t> for QDoubleSpinBox {
            unsafe fn get_rust(this: QPtr<Self>) -> $t {
                unsafe { this.value() as $t }
            }

            unsafe fn connect_rust(this: QPtr<Self>, t: &$t, receiver: QBox<SlotNoArgs>) {
                unsafe {
                    this.set_value(*t as c_double);
                    this.editing_finished().connect(&receiver);
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
/*
pub struct Force<E>(std::marker::PhantomData<E>);

impl<E: Enum> Force<E> {
    unsafe fn get_rust(this: &QComboBox) -> E {
        enum_from_index(this.current_index() as usize).expect("Enum out of bounds")
    }

    unsafe fn connect_rust(this: &QComboBox, e: &E, receiver: QBox<SlotNoArgs>) {
        this.set_current_index(e.index() as c_int);
        this.current_index_changed().connect(&receiver);
    }
}
*/
