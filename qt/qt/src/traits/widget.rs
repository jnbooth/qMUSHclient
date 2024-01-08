use std::rc;

use cpp_core::{Ptr, StaticUpcast};
use enumeration::Enum;
use qt_core::{q_event, Key, QFlags, QObject, QPtr, QString, SlotNoArgs};
use qt_gui::QKeyEvent;
use qt_widgets as q;
use qt_widgets::q_message_box::Icon;

use super::{Printable, QForm};
use crate::widgets::QMessageBox;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Browse {
    Open,
    Save,
    Directory,
}

pub trait Widget {
    fn widget(&self) -> Ptr<q::QWidget>;

    fn alert<S: Printable, E: std::error::Error + ?Sized>(&self, icon: Icon, text: S, err: &E) {
        let alert = QMessageBox::new(self.widget());
        alert.set_icon(icon);
        alert.set_text(text.to_print());
        alert.set_informative_text(err.to_string().to_print());
        alert.exec();
    }

    /// # Safety
    ///
    /// `button` and `field` must be valid.
    unsafe fn connect_browse_button<S, T, F>(
        &self,
        browse: Browse,
        button: &QPtr<T>,
        field: &QPtr<q::QLineEdit>,
        suggest: F,
        ext: &str,
    ) where
        S: Printable,
        T: StaticUpcast<QObject> + StaticUpcast<q::QAbstractButton>,
        F: 'static + Fn() -> S,
    {
        unsafe {
            let caption = QString::new();
            let button: QPtr<q::QAbstractButton> = button.static_upcast();
            let field = field.to_owned();
            let widget = self.widget();
            let filter = QString::from_std_str(ext);
            let keypress = QKeyEvent::from_type_int_q_flags_keyboard_modifier(
                q_event::Type::KeyPress,
                Key::KeyReturn.to_int(),
                QFlags::from(0),
            );
            button.clicked().connect(&SlotNoArgs::new(widget, move || {
                let default_suggestion = field.text();
                let suggested = if default_suggestion.is_empty() {
                    suggest().to_print()
                } else {
                    default_suggestion.to_print()
                };
                let filename = match browse {
                    Browse::Open => {
                        q::QFileDialog::get_open_file_name_4a(widget, &caption, &suggested, &filter)
                    }
                    Browse::Save => {
                        q::QFileDialog::get_save_file_name_4a(widget, &caption, &suggested, &filter)
                    }
                    Browse::Directory => {
                        q::QFileDialog::get_existing_directory_3a(widget, &caption, &suggested)
                    }
                };
                if !filename.is_empty() {
                    field.set_text(&filename);
                    field.event(&keypress);
                }
            }));
        }
    }

    /// # Safety
    ///
    /// `field` must be valid.
    unsafe fn connect_form<T, Q, W, F>(&self, field: &Q, initial: &T, weak: rc::Weak<W>, mut set: F)
    where
        Q: QForm<T>,
        W: 'static,
        F: 'static + Clone + FnMut(&W, T),
    {
        unsafe {
            QForm::connect(field, self.widget(), initial, move |val| {
                if let Some(strong) = weak.upgrade() {
                    set(&strong, val)
                }
            });
        }
    }
}
