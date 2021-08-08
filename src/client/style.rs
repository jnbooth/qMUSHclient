use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

use crate::binding::color::Colored;
use crate::binding::text::CharFormat;
use crate::client::color::WorldColor;
use crate::enums::{Enum, EnumSet};
use crate::mxp::{SendTo, Span};
use crate::world::World;

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
    if let WorldColor::Ansi(code) = *color {
        if bold && code < 8 {
            return Some(WorldColor::Ansi(code + 8));
        } else if !bold && code >= 8 {
            return Some(WorldColor::Ansi(code - 8));
        }
    }
    None
}

#[derive(PartialEq)]
pub struct Style {
    format: CharFormat,
    foreground: WorldColor,
    background: WorldColor,
    spans: Vec<Span>,
    span_flags: EnumSet<TextStyle>,
    ansi_flags: EnumSet<TextStyle>,
    world: Rc<World>,
}

impl Debug for Style {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Style")
            .field("flags", &self.ansi_flags)
            .field("foreground", &self.foreground)
            .field("background", &self.background)
            .field("spans", &self.spans)
            .field("span_flags", &self.span_flags)
            .field("ansi_flags", &self.ansi_flags)
            .finish()
    }
}

impl Style {
    pub const fn new(format: CharFormat, world: Rc<World>) -> Self {
        Self {
            format,
            world,
            foreground: WorldColor::WHITE,
            background: WorldColor::BLACK,
            spans: Vec::new(),
            ansi_flags: enums![],
            span_flags: enums![],
        }
    }

    pub const fn format(&self) -> &CharFormat {
        &self.format
    }

    pub fn span(&self) -> &Span {
        const DEFAULT: &Span = &Span {
            flags: enums![],
            foreground: None,
            background: None,
            action: None,
            variable: None,
            list: None,
        };
        self.spans.last().unwrap_or(DEFAULT)
    }

    pub fn set_world(&mut self, world: Rc<World>) {
        self.world = world;
    }

    fn set_ansi_bold(&mut self, bold: bool) {
        if self.world.show_bold {
            self.format.set_bold(bold);
            return;
        }
        let fg = if self.ansi_flags.contains(TextStyle::Inverse) {
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
        if !self.ansi_flags.contains(flag) {
            self.ansi_flags.insert(flag);
            match flag {
                TextStyle::Underline => self.format.set_underline(true),
                TextStyle::Italic => self.format.set_italic(true),
                TextStyle::Strikeout => self.format.set_strikeout(true),
                TextStyle::Inverse => invert(&self.format),
                TextStyle::Bold => self.set_ansi_bold(true),
            }
        }
    }

    pub fn remove_flag(&mut self, flag: TextStyle) {
        if self.ansi_flags.contains(flag) {
            self.ansi_flags.remove(flag);
            if !self.span_flags.contains(flag) {
                match flag {
                    TextStyle::Underline => self.format.set_underline(false),
                    TextStyle::Italic => self.format.set_italic(false),
                    TextStyle::Strikeout => self.format.set_strikeout(false),
                    TextStyle::Inverse => invert(&self.format),
                    TextStyle::Bold => self.set_ansi_bold(false),
                }
            }
        }
    }

    pub fn clear_flags(&mut self) {
        let clearing = self.ansi_flags & self.span_flags;
        if clearing.contains(TextStyle::Underline) {
            self.format.set_underline(false);
        }
        if clearing.contains(TextStyle::Italic) {
            self.format.set_italic(false);
        }
        if clearing.contains(TextStyle::Inverse) {
            invert(&self.format);
        }
        if clearing.contains(TextStyle::Bold) {
            self.format.set_bold(false);
            self.set_ansi_bold(false);
        }
        self.ansi_flags.clear();
    }

    pub fn reset(&mut self) {
        self.clear_flags();
        self.set_foreground_raw(WorldColor::WHITE);
        self.set_background_raw(WorldColor::BLACK);
    }

    fn set_foreground_raw(&mut self, color: WorldColor) {
        if self.foreground != color {
            if color != WorldColor::WHITE {
                self.format.set_foreground_color(self.world.color(&color));
            } else if let Some(newcolor) = self
                .spans
                .iter()
                .rev()
                .find_map(|span| span.foreground.as_ref())
            {
                self.format.set_foreground_color(newcolor);
            } else {
                self.format.clear_foreground();
            };
            self.foreground = color;
        }
    }

    fn set_background_raw(&mut self, color: WorldColor) {
        if self.background != color {
            if color != WorldColor::BLACK {
                self.format.set_background_color(self.world.color(&color));
            } else if let Some(newcolor) = self
                .spans
                .iter()
                .rev()
                .find_map(|span| span.background.as_ref())
            {
                self.format.set_background_color(newcolor);
            } else {
                self.format.clear_background();
            };
            self.background = color;
        }
    }

    pub fn set_foreground(&mut self, color: WorldColor) {
        if self.ansi_flags.contains(TextStyle::Inverse) {
            self.set_background_raw(color);
        } else {
            self.set_foreground_raw(color);
        }
    }

    pub fn set_background(&mut self, color: WorldColor) {
        if self.ansi_flags.contains(TextStyle::Inverse) {
            self.set_foreground_raw(color);
        } else {
            self.set_background_raw(color);
        }
    }

    pub fn foreground(&self) -> &WorldColor {
        if self.ansi_flags.contains(TextStyle::Inverse) {
            &self.foreground
        } else {
            &self.background
        }
    }

    pub fn _background(&self) -> &WorldColor {
        if self.ansi_flags.contains(TextStyle::Inverse) {
            &self.background
        } else {
            &self.foreground
        }
    }

    pub fn len(&self) -> usize {
        self.spans.len()
    }

    fn regen_flags(&mut self) {
        let new_flags = self.span().flags;
        for changed in !self.ansi_flags & (self.span_flags ^ new_flags) {
            let enable = new_flags.contains(changed);
            match changed {
                TextStyle::Underline => self.format.set_underline(enable),
                TextStyle::Italic => self.format.set_italic(enable),
                TextStyle::Strikeout => self.format.set_strikeout(enable),
                TextStyle::Inverse => invert(&self.format),
                TextStyle::Bold => self.set_ansi_bold(enable),
            }
        }
        if self.foreground == WorldColor::WHITE {
            match self
                .spans
                .iter()
                .rev()
                .find_map(|span| span.foreground.as_ref())
            {
                Some(fg) => self.format.set_foreground_color(fg),
                None => self.format.clear_foreground(),
            }
        }
        if self.background == WorldColor::BLACK {
            match self
                .spans
                .iter()
                .rev()
                .find_map(|span| span.background.as_ref())
            {
                Some(bg) => self.format.set_background_color(bg),
                None => self.format.clear_background(),
            }
        }
        self.span_flags = new_flags;
    }

    pub fn push(&mut self, span: Span) {
        if let Some(link) = &span.action {
            self.format.set_anchor(link.sendto == SendTo::Internet);
            self.format.set_anchor_href(&link.action);
            self.format.set_anchor_names(&link.prompts);
            match &link.hint {
                Some(hint) => self.format.set_tooltip(hint),
                None => self.format.clear_tooltip(),
            }
        }
        self.spans.push(span);
        self.regen_flags();
    }

    pub fn truncate(&mut self, i: usize) {
        if i >= self.len() {
            return;
        }
        let recalculate_style = self.spans[i..].iter().any(|span| span.action.is_some());
        self.spans.truncate(i);
        if !recalculate_style {
            return;
        }
        match self
            .spans
            .iter()
            .rev()
            .find_map(|span| span.action.as_ref())
        {
            None => {
                self.format.set_anchor(false);
                self.format.clear_anchor_href();
                self.format.clear_anchor_names();
                self.format.clear_tooltip();
            }
            Some(link) => {
                self.format.set_anchor(link.sendto == SendTo::Internet);
                self.format.set_anchor_href(&link.action);
                self.format.set_anchor_names(&link.prompts);
                match &link.hint {
                    Some(hint) => self.format.set_tooltip(hint),
                    None => self.format.clear_tooltip(),
                }
            }
        }
        self.regen_flags();
    }
}
