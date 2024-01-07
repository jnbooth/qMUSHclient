#![windows_subsystem = "windows"]

use qmushclient::{App, Widget};
use qt::{QLocale, QTranslator};
use qt_core::{ApplicationAttribute, QCoreApplication, QString};
use qt_widgets::QApplication;

fn load_translator() {
    unsafe {
        let locale = QLocale::new();
        let translator = QTranslator::new();
        let translate = translator.load(&locale, "app", "_", ":/translations", ".qm");
        if translate {
            QCoreApplication::install_translator(translator.get_translator());
        }
    }
}

fn main() {
    unsafe {
        QCoreApplication::set_attribute_2a(ApplicationAttribute::AAShareOpenGLContexts, true);
        QCoreApplication::set_organization_name(&QString::from_std_str("qMUSHclient"));
        QCoreApplication::set_organization_domain(&QString::from_std_str("qMUSHclient"));
    }
    QApplication::init(|_| {
        load_translator();

        let app = App::new();
        unsafe {
            app.widget().show();
            QApplication::exec()
        }
    })
}
