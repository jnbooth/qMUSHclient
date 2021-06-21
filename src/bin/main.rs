#![windows_subsystem = "windows"]

use qt_core::{
    q_init_resource, ApplicationAttribute, QCoreApplication, QLocale, QString, QTranslator,
};
use qt_widgets::QApplication;

use qmushclient::{App, RWidget};

fn load_translator() {
    unsafe {
        let locale = QLocale::new();
        let translator = QTranslator::new_0a();
        let translate = translator.load_q_locale4_q_string(
            &locale,
            &QString::from_std_str("app"),
            &QString::from_std_str("_"),
            &QString::from_std_str(":/translations"),
            &QString::from_std_str(".qm"),
        );
        if translate {
            QCoreApplication::install_translator(&translator);
        }
    }
}

fn main() {
    unsafe {
        QCoreApplication::set_attribute_2a(ApplicationAttribute::AAShareOpenGLContexts, true);
    }
    QApplication::init(|_| {
        q_init_resource!("resources");
        load_translator();

        let app = App::new("qMUSHclient", "qMUSHclient");
        unsafe {
            app.widget().show();
            QApplication::exec()
        }
    })
}
