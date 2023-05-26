use qt_core::{KeyboardModifier, MouseButton, QFlags};
use qt_gui::QGuiApplication;

pub fn keyboard_modifiers() -> QFlags<KeyboardModifier> {
    unsafe { QGuiApplication::keyboard_modifiers() }
}

pub fn mouse_buttons() -> QFlags<MouseButton> {
    unsafe { QGuiApplication::mouse_buttons() }
}
