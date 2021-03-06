use std::ffi::{CStr, CString};
use std::fmt::{self, Write};
#[cfg(test)]
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ptr;

use cpp_core::CppBox;
#[cfg(not(test))]
use qt_core::QCoreApplication;
use qt_core::QString;

#[cfg(test)]
/// A stub, because the real QCoreApplication is not initialized.
struct QCoreApplication;

#[cfg(test)]
impl QCoreApplication {
    pub unsafe fn translate_4a(
        _: *const c_char,
        key: *const c_char,
        _: *const c_char,
        _: c_int,
    ) -> CppBox<QString> {
        unsafe { QString::from_utf8_char(key) }
    }

    pub unsafe fn translate_2a(_: *const c_char, key: *const c_char) -> CppBox<QString> {
        unsafe { QString::from_utf8_char(key) }
    }
}

/// Any object that calls tr! should implement this trait.
/// It's probably best to let a procedural macro derive this automatically.
pub trait TrContext {
    const CLASS_NAME: &'static CStr;
}

/// Like [`std::format!`], but the string literal for formatting is translated through Qt's linguist
/// framework and the output is a `CppBox<QString>` instead of a `String`.
///
/// For example, `tr!("translate {}, {:#X}, {}", "something", 2, false)`
/// will ask the Qt engine to translate "translate %1, %2, %3". The result will be passed the
/// arguments "something", "0x2", and "false".
/// `tr!` uses [`std::format_args!`] under the hood, so formatting errors will be caught at compile
/// time.
#[macro_export]
macro_rules! tr {
    // simple translation of a string literal
    ($s:literal) => (
        $crate::tr::translate(Self::CLASS_NAME, $s)
    );
    // translation of a string formatted with arguments
    ($fmt:literal,$($arg:tt)*) => (
        $crate::tr::fmt(Self::CLASS_NAME, std::format_args!($fmt,$($arg)*))
    );
    // translation of a string literal with a numerus
    ($n:expr,$s:literal) => (
        $crate::tr::translate_amount(Self::CLASS_NAME, $s, $n as std::os::raw::c_int)
    );
    // translation of a string with a numerus, formatted with arguments
    ($n:expr,$s:literal,$($arg:tt)*) => (
        $crate::tr::fmt_amount(Self::CLASS_NAME, std::format_args!($s, $($arg)*), $n as std::os::raw::c_int)
    );
}

/// Separates [`fmt::Arguments`] into the key and its arguments.
struct ArgumentWalker {
    on_arg: bool,
    key: String,
    args: Vec<String>,
}

impl fmt::Write for ArgumentWalker {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.on_arg {
            self.args.push(s.to_owned());
            self.key.push('%');
            self.key.push_str(&self.args.len().to_string());
        } else {
            self.key.push_str(s);
        }
        self.on_arg = !self.on_arg;
        Ok(())
    }
}

const ERROR: &str = "unexpected error while formatting a string for translation";
/// If `n` is `Some`, formats a numerus translation. If `n` is `None`, formats without a numerus.
fn fmt_either(context: &CStr, args: fmt::Arguments, n: Option<c_int>) -> CppBox<QString> {
    let mut walker = ArgumentWalker {
        on_arg: false,
        key: String::new(),
        args: Vec::new(),
    };
    walker.write_fmt(args).expect(ERROR);
    let ckey = CString::new(walker.key).expect(ERROR);
    let mut qkey = unsafe {
        match n {
            None => QCoreApplication::translate_2a(context.as_ptr(), ckey.as_ptr()),
            Some(n) => {
                QCoreApplication::translate_4a(context.as_ptr(), ckey.as_ptr(), ptr::null(), n)
            }
        }
    };
    for arg in walker.args {
        let qarg = QString::from_std_str(&arg);
        qkey = unsafe { qkey.arg_q_string(&qarg) };
    }
    qkey
}
/// Translates arguments without a numerus.
pub fn fmt(context: &CStr, args: fmt::Arguments) -> CppBox<QString> {
    fmt_either(context, args, None)
}
/// Translates arguments with a provided numerus.
pub fn fmt_amount(context: &CStr, args: fmt::Arguments, n: c_int) -> CppBox<QString> {
    fmt_either(context, args, Some(n))
}
/// Translates a bare string literal without a numerus.
pub fn translate(context: &CStr, s: &str) -> CppBox<QString> {
    let cstr = CString::new(s).expect(ERROR);
    unsafe { QCoreApplication::translate_2a(context.as_ptr(), cstr.as_ptr()) }
}
/// Translates a bare string literal with a numerus.
pub fn translate_amount(context: &CStr, s: &str, n: c_int) -> CppBox<QString> {
    let cstr = CString::new(s).expect(ERROR);
    unsafe { QCoreApplication::translate_4a(context.as_ptr(), cstr.as_ptr(), ptr::null(), n) }
}
