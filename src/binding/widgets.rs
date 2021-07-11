use std::error::Error as StdError;
use std::os::raw::c_int;

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{q_event, Key, QFlags, QObject, QPtr, QString, SlotNoArgs};
use qt_gui::QKeyEvent;
use qt_widgets::q_message_box::Icon;
use qt_widgets::{QAbstractButton, QFileDialog, QLineEdit, QMessageBox, QWidget};

use super::Printable;
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

    fn connect_browse_button<T>(
        &self,
        browse: Browse,
        button: &QPtr<T>,
        field: &QPtr<QLineEdit>,
        dir: &str,
        ext: &str,
    ) where
        T: StaticUpcast<QObject> + StaticUpcast<QAbstractButton>,
    {
        unsafe {
            let caption = QString::new();
            let button: QPtr<QAbstractButton> = button.static_upcast();
            let field = field.to_owned();
            let widget = self.widget();
            let default_dir = QString::from_std_str(dir);
            let filter = QString::from_std_str(ext);
            let keypress = QKeyEvent::from_type_int_q_flags_keyboard_modifier(
                q_event::Type::KeyPress,
                Key::KeyReturn.to_int(),
                QFlags::from(0),
            );
            button
                .clicked()
                .connect(&SlotNoArgs::new(widget.clone(), move || {
                    let try_dir = field.text();
                    let dir = if try_dir.is_empty() {
                        &default_dir
                    } else {
                        &try_dir
                    };
                    let filename = match browse {
                        Browse::Open => QFileDialog::get_open_file_name_4a(
                            widget.clone(),
                            &caption,
                            dir,
                            &filter,
                        ),
                        Browse::Save => QFileDialog::get_save_file_name_4a(
                            widget.clone(),
                            &caption,
                            dir,
                            &filter,
                        ),
                        Browse::Directory => {
                            QFileDialog::get_existing_directory_3a(widget.clone(), &caption, dir)
                        }
                    };
                    if !filename.is_empty() {
                        field.set_text(&filename);
                        field.event(&keypress);
                    }
                }));
        }
    }
}

pub trait RDialog<Response: From<c_int>>: RWidget {
    fn exec(&self) -> Response;
}
