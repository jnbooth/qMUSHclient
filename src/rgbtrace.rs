#![allow(dead_code)]

pub const BLACK: u8 = 30;
pub const RED: u8 = 31;
pub const GREEN: u8 = 32;
pub const YELLOW: u8 = 33;
pub const BLUE: u8 = 34;
pub const MAGENTA: u8 = 35;
pub const CYAN: u8 = 36;
pub const WHITE: u8 = 37;

#[allow(unused_macros)]
macro_rules! rgbtrace {
    ($col:ident, $s:literal, $($arg:tt)*) =>
        (println!(concat!("\x1B[{}m", $s, "\x1B[0m"), crate::rgbtrace::$col, $($arg)*))
}
