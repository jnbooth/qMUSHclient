qt_binding!(ObjectBinding, qt_core::QObject);

impl ObjectBinding {
    pub fn object_name(&self) -> String {
        unsafe { self.inner.object_name().to_std_string() }
    }
}
