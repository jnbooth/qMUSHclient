use std::os::raw::{c_char, c_int};

use cpp_core::CppBox;
use qt_core::QString;

pub trait Printable {
    fn to_print(self) -> CppBox<QString>;
}
impl Printable for CppBox<QString> {
    fn to_print(self) -> CppBox<QString> {
        self
    }
}
impl Printable for String {
    fn to_print(self) -> CppBox<QString> {
        QString::from_std_str(self)
    }
}
impl Printable for &String {
    fn to_print(self) -> CppBox<QString> {
        QString::from_std_str(self)
    }
}
impl Printable for &str {
    fn to_print(self) -> CppBox<QString> {
        QString::from_std_str(self)
    }
}
impl Printable for &[u8] {
    fn to_print(self) -> CppBox<QString> {
        unsafe { QString::from_utf8_char_int(self.as_ptr() as *const c_char, self.len() as c_int) }
    }
}
impl<const N: usize> Printable for &[u8; N] {
    fn to_print(self) -> CppBox<QString> {
        (self as &[u8]).to_print()
    }
}
impl<const N: usize> Printable for [u8; N] {
    fn to_print(self) -> CppBox<QString> {
        (&self as &[u8]).to_print()
    }
}

impl Printable for Vec<u8> {
    fn to_print(self) -> CppBox<QString> {
        self[..].to_print()
    }
}

impl Printable for &Vec<u8> {
    fn to_print(self) -> CppBox<QString> {
        self[..].to_print()
    }
}
