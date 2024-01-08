use cpp_core::CppBox;
use q::QString;
use qt_core as q;
use qt_core::QBox;

#[repr(transparent)]
pub struct QLocale {
    pub(crate) inner: CppBox<q::QLocale>,
}

impl_eq_cpp!(QLocale);

impl From<CppBox<q::QLocale>> for QLocale {
    fn from(value: CppBox<q::QLocale>) -> Self {
        Self { inner: value }
    }
}

impl QLocale {
    pub fn new() -> Self {
        Self {
            inner: unsafe { q::QLocale::new() },
        }
    }
}

#[repr(transparent)]
pub struct QTranslator {
    pub(crate) inner: QBox<q::QTranslator>,
}

impl From<QBox<q::QTranslator>> for QTranslator {
    fn from(value: QBox<q::QTranslator>) -> Self {
        Self { inner: value }
    }
}

impl QTranslator {
    pub fn new() -> Self {
        Self {
            inner: unsafe { q::QTranslator::new_0a() },
        }
    }

    pub fn load(
        &self,
        locale: &QLocale,
        filename: &str,
        prefix: &str,
        directory: &str,
        suffix: &str,
    ) -> bool {
        unsafe {
            self.inner.load_q_locale4_q_string(
                &locale.inner,
                &QString::from_std_str(filename),
                &QString::from_std_str(prefix),
                &QString::from_std_str(directory),
                &QString::from_std_str(suffix),
            )
        }
    }
}
