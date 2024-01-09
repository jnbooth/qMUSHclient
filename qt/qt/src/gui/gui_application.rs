use qt_core::{KeyboardModifier, MouseButton, QBox, QFlags};
use qt_gui as q;

qt_binding!(
    GuiApplicationBinding,
    q::QGuiApplication,
    crate::core::CoreApplicationBinding
);

pub struct QGuiApplication {
    pub(crate) inner: QBox<q::QGuiApplication>,
}

impl_deref_binding!(QGuiApplication, GuiApplicationBinding);

impl QGuiApplication {
    pub fn keyboard_modifiers() -> QFlags<KeyboardModifier> {
        unsafe { q::QGuiApplication::keyboard_modifiers() }
    }

    pub fn mouse_buttons() -> QFlags<MouseButton> {
        unsafe { q::QGuiApplication::mouse_buttons() }
    }
}
