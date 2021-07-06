#![windows_subsystem = "windows"]

use qmushclient::{App, RWidget};
use qt_core::{ApplicationAttribute, QCoreApplication, QLocale, QString, QTranslator};
use qt_widgets::QApplication;

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
        QCoreApplication::set_organization_name(&QString::from_std_str("qMUSHclient"));
        QCoreApplication::set_organization_domain(&QString::from_std_str("qMUSHclient"));
    }
    QApplication::init(|_| {
        extern "C" {
            fn ritual_init_resource_resources();
        }
        unsafe {
            ritual_init_resource_resources();
        }
        load_translator();

        let app = App::new();
        unsafe {
            app.widget().show();
            QApplication::exec()
        }
    })
}
