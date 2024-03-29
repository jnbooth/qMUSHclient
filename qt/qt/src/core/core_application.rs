use qt_core as q;
use qt_core::{ApplicationAttribute, QBox, QString};

use crate::core::{ObjectBinding, QTranslator};

qt_binding!(CoreApplicationBinding, q::QCoreApplication, ObjectBinding);

#[repr(transparent)]
pub struct QCoreApplication {
    pub(crate) inner: QBox<q::QCoreApplication>,
}

impl_deref_binding!(QCoreApplication, CoreApplicationBinding);

impl QCoreApplication {
    pub fn install_translator(translator: &QTranslator) -> bool {
        unsafe { q::QCoreApplication::install_translator(&translator.inner) }
    }

    pub fn set_attribute(attribute: ApplicationAttribute, on: bool) {
        unsafe { q::QCoreApplication::set_attribute_2a(attribute, on) }
    }

    pub fn set_organization_name(name: &str) {
        unsafe { q::QCoreApplication::set_organization_name(&QString::from_std_str(name)) }
    }

    pub fn set_organization_domain(domain: &str) {
        unsafe { q::QCoreApplication::set_organization_domain(&QString::from_std_str(domain)) }
    }
}
