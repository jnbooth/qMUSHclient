use crate::binding::color::Colored;
use crate::binding::text::CharFormat;
use crate::client::color::WorldColor;
use crate::enums::{Enum, EnumSet};
use crate::world::World;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum TextStyle {
    Underline,
    Italic,
    Strikeout,
    Inverse,
    Bold,
}

fn invert(format: &CharFormat) {
    let mut colors = format.colors();
    colors.invert();
    format.set_colors(&colors);
}

fn set_bold(color: &WorldColor, bold: bool) -> Option<WorldColor> {
    if let &WorldColor::Ansi(code) = color {
        if bold && code < 8 {
            return Some(WorldColor::Ansi(code + 8));
        } else if !bold && code >= 8 {
            return Some(WorldColor::Ansi(code - 8));
        }
    }
    None
}

pub struct Style {
    format: CharFormat,
    world: Rc<World>,
    flags: EnumSet<TextStyle>,
    foreground: WorldColor,
    background: WorldColor,
}

impl Debug for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Style")
            .field("flags", &self.flags)
            .field("foreground", &self.foreground)
            .field("background", &self.background)
            .finish()
    }
}

impl Style {
    pub const fn new(format: CharFormat, world: Rc<World>) -> Self {
        Self {
            format,
            world,
            flags: enums![],
            foreground: WorldColor::WHITE,
            background: WorldColor::BLACK,
        }
    }
    pub const fn format(&self) -> &CharFormat {
        &self.format
    }
    pub fn set_world(&mut self, world: Rc<World>) {
        self.world = world;
    }

    fn set_ansi_bold(&mut self, bold: bool) {
        if self.world.show_bold {
            self.format.set_bold(bold);
            return;
        }
        let fg = if self.flags.contains(TextStyle::Inverse) {
            &self.background
        } else {
            &self.foreground
        };
        match set_bold(fg, bold) {
            Some(bfg) => self.set_foreground(bfg),
            None => self.format.set_bold(bold),
        }
    }

    pub fn add_flag(&mut self, flag: TextStyle) {
        if !self.flags.contains(flag) {
            self.flags.insert(flag);
            match flag {
                TextStyle::Underline => self.format.set_underline(true),
                TextStyle::Italic => self.format.set_italic(true),
                TextStyle::Strikeout => self.format.set_strikeout(true),
                TextStyle::Inverse => invert(&mut self.format),
                TextStyle::Bold => self.set_ansi_bold(true),
            }
        }
    }
    pub fn remove_flag(&mut self, flag: TextStyle) {
        if self.flags.contains(flag) {
            self.flags.remove(flag);
            match flag {
                TextStyle::Underline => self.format.set_underline(false),
                TextStyle::Italic => self.format.set_italic(false),
                TextStyle::Strikeout => self.format.set_strikeout(false),
                TextStyle::Inverse => invert(&mut self.format),
                TextStyle::Bold => self.set_ansi_bold(false),
            }
        }
    }
    pub fn clear_flags(&mut self) {
        if self.flags.contains(TextStyle::Underline) {
            self.format.set_underline(false);
        }
        if self.flags.contains(TextStyle::Italic) {
            self.format.set_italic(false);
        }
        if self.flags.contains(TextStyle::Inverse) {
            invert(&mut self.format);
        }
        if self.flags.contains(TextStyle::Bold) {
            self.format.set_bold(false);
            self.set_ansi_bold(false);
        }
        self.flags.clear();
    }
    pub fn reset(&mut self) {
        self.clear_flags();
        self.set_foreground_raw(WorldColor::WHITE);
        self.set_background_raw(WorldColor::BLACK);
    }
    fn set_foreground_raw(&mut self, color: WorldColor) {
        if self.foreground != color {
            self.format.set_foreground_color(self.world.color(&color));
            self.foreground = color;
        }
    }
    fn set_background_raw(&mut self, color: WorldColor) {
        if self.background != color {
            self.format.set_background_color(self.world.color(&color));
            self.background = color;
        }
    }
    pub fn set_foreground(&mut self, color: WorldColor) {
        if self.flags.contains(TextStyle::Inverse) {
            self.set_background_raw(color);
        } else {
            self.set_foreground_raw(color);
        }
    }
    pub fn set_background(&mut self, color: WorldColor) {
        if self.flags.contains(TextStyle::Inverse) {
            self.set_foreground_raw(color);
        } else {
            self.set_background_raw(color);
        }
    }
}
