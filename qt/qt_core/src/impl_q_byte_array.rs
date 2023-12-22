use std::os::raw::{c_char, c_int};

use cpp_core::CppBox;

use crate::QByteArray;

impl QByteArray {
    /// Creates a `QByteArray` containing bytes from `slice`.
    ///
    /// `QByteArray` makes a deep copy of the data.
    pub fn from_slice(slice: &[u8]) -> CppBox<QByteArray> {
        unsafe { QByteArray::from_char_int(slice.as_ptr() as *const c_char, slice.len() as c_int) }
    }

    /// Creates a byte vector from a `QByteArray`.
    pub fn to_vec(&self) -> Vec<u8> {
        unsafe { std::slice::from_raw_parts(self.const_data() as *const u8, self.size() as usize) }
            .to_vec()
    }
}
