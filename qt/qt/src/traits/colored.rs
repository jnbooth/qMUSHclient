use qt_gui as q;
use qt_gui::q_palette::ColorRole;

use crate::gui::{QColor, QColorPair};

pub trait HasPalette {
    fn palette_color(&self, role: ColorRole) -> QColor;
    fn set_palette_color(&self, role: ColorRole, color: &QColor);
}

impl HasPalette for qt_widgets::QWidget {
    fn palette_color(&self, role: ColorRole) -> QColor {
        QColor::of(unsafe { self.palette().color_1a(role) })
    }

    fn set_palette_color(&self, role: ColorRole, color: &QColor) {
        unsafe {
            let palette = self.palette();
            palette.set_brush_2a(role, &color.inner);
            self.set_palette(palette);
            self.repaint();
        }
    }
}

pub trait Colored {
    fn foreground_color(&self) -> QColor;
    fn set_foreground_color(&self, color: &QColor);
    fn background_color(&self) -> QColor;
    fn set_background_color(&self, color: &QColor);

    fn colors(&self) -> QColorPair {
        QColorPair {
            foreground: self.foreground_color(),
            background: self.background_color(),
        }
    }
    fn set_colors(&self, colors: &QColorPair) {
        self.set_foreground_color(&colors.foreground);
        self.set_background_color(&colors.background);
    }
}

impl Colored for q::QTextFormat {
    fn foreground_color(&self) -> QColor {
        QColor::from(unsafe { self.foreground() })
    }
    fn background_color(&self) -> QColor {
        QColor::from(unsafe { self.background() })
    }
    fn set_foreground_color(&self, color: &QColor) {
        unsafe {
            self.set_foreground(&color.inner);
        }
    }
    fn set_background_color(&self, color: &QColor) {
        unsafe {
            if color.is_transparent() {
                self.clear_background();
            } else {
                self.set_background(&color.inner);
            }
        }
    }
}

impl Colored for qt_widgets::QTextEdit {
    fn foreground_color(&self) -> QColor {
        unsafe { QColor::from(self.text_color()) }
    }
    fn background_color(&self) -> QColor {
        self.palette_color(ColorRole::Base)
    }
    fn set_foreground_color(&self, color: &QColor) {
        unsafe {
            self.set_text_color(color.color());
        }
    }
    fn set_background_color(&self, color: &QColor) {
        self.set_palette_color(ColorRole::Base, color);
    }
}

impl Colored for qt_widgets::QWidget {
    fn foreground_color(&self) -> QColor {
        self.palette_color(unsafe { self.foreground_role() })
    }
    fn background_color(&self) -> QColor {
        self.palette_color(unsafe { self.background_role() })
    }
    fn set_foreground_color(&self, color: &QColor) {
        self.set_palette_color(unsafe { self.foreground_role() }, color);
    }
    fn set_background_color(&self, color: &QColor) {
        self.set_palette_color(unsafe { self.background_role() }, color);
    }
    fn set_colors(&self, colors: &QColorPair) {
        unsafe {
            let palette = self.palette();
            palette.set_brush_2a(self.foreground_role(), &colors.foreground.inner);
            palette.set_brush_2a(self.background_role(), &colors.background.inner);
            self.set_palette(palette);
        }
    }
}
