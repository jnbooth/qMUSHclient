#![windows_subsystem = "windows"]

use qmushclient::{App, Widget};
use qt::core::{ApplicationAttribute, QCoreApplication, QLocale, QTranslator};
use qt::widgets::QApplication;

fn load_translator() {
    let locale = QLocale::new();
    let translator = QTranslator::new();
    let translate = translator.load(&locale, "app", "-", ":/translations", ".qm");
    if translate {
        QCoreApplication::install_translator(&translator);
    }
}

fn main() {
    QCoreApplication::set_attribute(ApplicationAttribute::AAShareOpenGLContexts, true);
    QCoreApplication::set_organization_name("qMUSHclient");
    QCoreApplication::set_organization_domain("qMUSHclient");

    QApplication::init(|_| {
        load_translator();

        let app = App::new();
        unsafe {
            app.widget().show();
        }
        QApplication::exec()
    })
}
