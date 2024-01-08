macro_rules! impl_widget {
    ($t:ty) => {
        impl qt::traits::Widget for $t {
            fn widget(&self) -> cpp_core::Ptr<qt_widgets::QWidget> {
                unsafe { cpp_core::CastFrom::cast_from(&self.ui.widget) }
            }
        }

        impl cpp_core::StaticUpcast<qt_core::QObject> for $t {
            unsafe fn static_upcast(ptr: cpp_core::Ptr<Self>) -> cpp_core::Ptr<qt_core::QObject> {
                unsafe { cpp_core::CastFrom::cast_from(&ptr.ui.widget) }
            }
        }
    };
}

mod uic;

mod app;
pub use app::App;

mod notepad;
pub use notepad::{Notepad, Pad};

mod worldprefs;
pub use worldprefs::WorldPrefs;

mod worldtab;
pub use worldtab::WorldTab;
