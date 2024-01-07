use qt_core::{KeyboardModifier, MouseButton, QBox, QCoreApplicationArgs, QFlags};
use qt_gui as q;

qt_binding!(
    GuiApplicationBinding,
    q::QGuiApplication,
    super::core_application::CoreApplicationBinding
);

pub struct QGuiApplication {
    pub(crate) inner: QBox<q::QGuiApplication>,
}

impl_deref_binding!(QGuiApplication, GuiApplicationBinding);

impl QGuiApplication {
    pub fn new() -> Self {
        let mut args = QCoreApplicationArgs::new();
        let (argc, argv) = args.get();
        let app = unsafe { q::QGuiApplication::new_2a(argc, argv) };
        Self { inner: app }
    }

    pub fn keyboard_modifiers() -> QFlags<KeyboardModifier> {
        unsafe { q::QGuiApplication::keyboard_modifiers() }
    }

    pub fn mouse_buttons() -> QFlags<MouseButton> {
        unsafe { q::QGuiApplication::mouse_buttons() }
    }
}
