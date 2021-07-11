use std::borrow::Cow;
use std::convert::TryInto;

use hashbrown::HashMap;
use mlua::{Error as E, FromLua, Lua, Value};
use once_cell::sync::Lazy;
use qt_core::GlobalColor;
use serde::{Deserialize, Serialize};

use crate::binding::color::RColorPair;
use crate::binding::RColor;
use crate::escape::ansi;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum WorldColor {
    Ansi(usize),
    CustomFg(usize),
    CustomBg(usize),
    Xterm(u8),
    Plain(RColor),
}

#[allow(unused)]
impl WorldColor {
    pub const BLACK: Self = Self::Ansi(0);
    pub const RED: Self = Self::Ansi(1);
    pub const GREEN: Self = Self::Ansi(2);
    pub const YELLOW: Self = Self::Ansi(3);
    pub const BLUE: Self = Self::Ansi(4);
    pub const PURPLE: Self = Self::Ansi(5);
    pub const CYAN: Self = Self::Ansi(6);
    pub const WHITE: Self = Self::Ansi(7);
    pub const BRIGHT_BLACK: Self = Self::Ansi(8);
    pub const BRIGHT_RED: Self = Self::Ansi(9);
    pub const BRIGHT_GREEN: Self = Self::Ansi(10);
    pub const BRIGHT_YELLOW: Self = Self::Ansi(11);
    pub const BRIGHT_BLUE: Self = Self::Ansi(12);
    pub const BRIGHT_PURPLE: Self = Self::Ansi(13);
    pub const BRIGHT_CYAN: Self = Self::Ansi(14);
    pub const BRIGHT_WHITE: Self = Self::Ansi(15);
    pub const fn fg_from_ansi(code: u8) -> Option<WorldColor> {
        if code == ansi::FG_DEFAULT {
            Some(Self::WHITE)
        } else if code >= ansi::FG_BLACK && code <= ansi::FG_WHITE {
            Some(Self::Ansi((code - ansi::FG_BLACK) as usize))
        } else {
            None
        }
    }
    pub const fn bg_from_ansi(code: u8) -> Option<WorldColor> {
        if code == ansi::BG_DEFAULT {
            Some(Self::BLACK)
        } else if code >= ansi::BG_BLACK && code <= ansi::BG_WHITE {
            Some(Self::Ansi((code - ansi::BG_BLACK) as usize))
        } else {
            None
        }
    }
}

static NAMED_COLORS: Lazy<HashMap<String, RColor>> = Lazy::new(|| {
    [
        ("aliceblue", 0xF0F8FF),
        ("antiquewhite", 0xFAEBD7),
        ("aqua", 0x00FFFF),
        ("aquamarine", 0x7FFFD4),
        ("azure", 0xF0FFFF),
        ("beige", 0xF5F5DC),
        ("bisque", 0xFFE4C4),
        ("black", 0x000000),
        ("blanchedalmond", 0xFFEBCD),
        ("blue", 0x0000FF),
        ("blueviolet", 0x8A2BE2),
        ("brown", 0xA52A2A),
        ("burlywood", 0xDEB887),
        ("cadetblue", 0x5F9EA0),
        ("chartreuse", 0x7FFF00),
        ("chocolate", 0xD2691E),
        ("coral", 0xFF7F50),
        ("cornflowerblue", 0x6495ED),
        ("cornsilk", 0xFFF8DC),
        ("crimson", 0xDC143C),
        ("cyan", 0x00FFFF),
        ("darkblue", 0x00008B),
        ("darkcyan", 0x008B8B),
        ("darkgoldenrod", 0xB8860B),
        ("darkgray", 0xA9A9A9),
        ("darkgrey", 0xA9A9A9),
        ("darkgreen", 0x006400),
        ("darkkhaki", 0xBDB76B),
        ("darkmagenta", 0x8B008B),
        ("darkolivegreen", 0x556B2F),
        ("darkorange", 0xFF8C00),
        ("darkorchid", 0x9932CC),
        ("darkred", 0x8B0000),
        ("darksalmon", 0xE9967A),
        ("darkseagreen", 0x8FBC8F),
        ("darkslateblue", 0x483D8B),
        ("darkslategray", 0x2F4F4F),
        ("darkslategrey", 0x2F4F4F),
        ("darkturquoise", 0x00CED1),
        ("darkviolet", 0x9400D3),
        ("deeppink", 0xFF1493),
        ("deepskyblue", 0x00BFFF),
        ("dimgray", 0x696969),
        ("dimgrey", 0x696969),
        ("dodgerblue", 0x1E90FF),
        ("firebrick", 0xB22222),
        ("floralwhite", 0xFFFAF0),
        ("forestgreen", 0x228B22),
        ("fuchsia", 0xFF00FF),
        ("gainsboro", 0xDCDCDC),
        ("ghostwhite", 0xF8F8FF),
        ("gold", 0xFFD700),
        ("goldenrod", 0xDAA520),
        ("gray", 0x808080),
        ("grey", 0x808080),
        ("green", 0x008000),
        ("greenyellow", 0xADFF2F),
        ("honeydew", 0xF0FFF0),
        ("hotpink", 0xFF69B4),
        ("indianred", 0xCD5C5C),
        ("indigo", 0x4B0082),
        ("ivory", 0xFFFFF0),
        ("khaki", 0xF0E68C),
        ("lavender", 0xE6E6FA),
        ("lavenderblush", 0xFFF0F5),
        ("lawngreen", 0x7CFC00),
        ("lemonchiffon", 0xFFFACD),
        ("lightblue", 0xADD8E6),
        ("lightcoral", 0xF08080),
        ("lightcyan", 0xE0FFFF),
        ("lightgoldenrodyellow", 0xFAFAD2),
        ("lightgreen", 0x90EE90),
        ("lightgrey", 0xD3D3D3),
        ("lightgray", 0xD3D3D3),
        ("lightpink", 0xFFB6C1),
        ("lightsalmon", 0xFFA07A),
        ("lightseagreen", 0x20B2AA),
        ("lightskyblue", 0x87CEFA),
        ("lightslategray", 0x778899),
        ("lightslategrey", 0x778899),
        ("lightsteelblue", 0xB0C4DE),
        ("lightyellow", 0xFFFFE0),
        ("lime", 0x00FF00),
        ("limegreen", 0x32CD32),
        ("linen", 0xFAF0E6),
        ("magenta", 0xFF00FF),
        ("maroon", 0x800000),
        ("mediumaquamarine", 0x66CDAA),
        ("mediumblue", 0x0000CD),
        ("mediumorchid", 0xBA55D3),
        ("mediumpurple", 0x9370DB),
        ("mediumseagreen", 0x3CB371),
        ("mediumslateblue", 0x7B68EE),
        ("mediumspringgreen", 0x00FA9A),
        ("mediumturquoise", 0x48D1CC),
        ("mediumvioletred", 0xC71585),
        ("midnightblue", 0x191970),
        ("mintcream", 0xF5FFFA),
        ("mistyrose", 0xFFE4E1),
        ("moccasin", 0xFFE4B5),
        ("navajowhite", 0xFFDEAD),
        ("navy", 0x000080),
        ("oldlace", 0xFDF5E6),
        ("olive", 0x808000),
        ("olivedrab", 0x6B8E23),
        ("orange", 0xFFA500),
        ("orangered", 0xFF4500),
        ("orchid", 0xDA70D6),
        ("palegoldenrod", 0xEEE8AA),
        ("palegreen", 0x98FB98),
        ("paleturquoise", 0xAFEEEE),
        ("palevioletred", 0xDB7093),
        ("papayawhip", 0xFFEFD5),
        ("peachpuff", 0xFFDAB9),
        ("peru", 0xCD853F),
        ("pink", 0xFFC0CB),
        ("plum", 0xDDA0DD),
        ("powderblue", 0xB0E0E6),
        ("purple", 0x800080),
        ("rebeccapurple", 0x663399),
        ("red", 0xFF0000),
        ("rosybrown", 0xBC8F8F),
        ("royalblue", 0x4169E1),
        ("saddlebrown", 0x8B4513),
        ("salmon", 0xFA8072),
        ("sandybrown", 0xF4A460),
        ("seagreen", 0x2E8B57),
        ("seashell", 0xFFF5EE),
        ("sienna", 0xA0522D),
        ("silver", 0xC0C0C0),
        ("skyblue", 0x87CEEB),
        ("slateblue", 0x6A5ACD),
        ("slategray", 0x708090),
        ("slategrey", 0x708090),
        ("snow", 0xFFFAFA),
        ("springgreen", 0x00FF7F),
        ("steelblue", 0x4682B4),
        ("tan", 0xD2B48C),
        ("teal", 0x008080),
        ("thistle", 0xD8BFD8),
        ("tomato", 0xFF6347),
        ("turquoise", 0x40E0D0),
        ("violet", 0xEE82EE),
        ("wheat", 0xF5DEB3),
        ("white", 0xFFFFFF),
        ("whitesmoke", 0xF5F5F5),
        ("yellow", 0xFFFF00),
        ("yellowgreen", 0x9ACD32),
    ]
    .iter()
    .map(|&(s, code)| (s.to_owned(), RColor::from(code)))
    .collect()
});

static XTERM_COLORS: Lazy<[RColor; 256]> = Lazy::new(|| {
    let mut vec = Vec::with_capacity(256);
    vec.extend_from_slice(&[
        RColor::rgb(0, 0, 0),       // black
        RColor::rgb(128, 0, 0),     // maroon
        RColor::rgb(0, 128, 0),     // green
        RColor::rgb(128, 128, 0),   // olive
        RColor::rgb(0, 0, 128),     // navy
        RColor::rgb(128, 0, 128),   // purple
        RColor::rgb(0, 128, 128),   // teal
        RColor::rgb(192, 192, 192), // silver
        RColor::rgb(128, 128, 128), // gray
        RColor::rgb(255, 0, 0),     // red
        RColor::rgb(0, 255, 0),     // lime
        RColor::rgb(255, 255, 0),   // yellow
        RColor::rgb(0, 0, 255),     // blue
        RColor::rgb(255, 0, 255),   // magenta
        RColor::rgb(0, 255, 255),   // cyan
        RColor::rgb(255, 255, 255), // white
    ]);
    const COLOR_SCALE: [u8; 6] = [
        0,
        95,
        95 + 40,
        95 + 40 + 40,
        95 + 40 + 40 + 40,
        95 + 40 + 40 + 40 + 40,
    ];
    for &red in &COLOR_SCALE {
        for &green in &COLOR_SCALE {
            for &blue in &COLOR_SCALE {
                vec.push(RColor::rgb(red, green, blue));
            }
        }
    }
    for gray in (8..248).step_by(10) {
        vec.push(RColor::rgb(gray, gray, gray));
    }
    vec.try_into().unwrap()
});

pub struct Colors;

impl Colors {
    pub fn named(name: &str) -> Option<Cow<'static, RColor>> {
        match NAMED_COLORS.get(name) {
            Some(named) => Some(Cow::Borrowed(named)),
            None => RColor::named(name).map(Cow::Owned),
        }
    }
    pub fn xterm(code: u8) -> &'static RColor {
        &XTERM_COLORS[code as usize]
    }
    pub fn from_lua<'lua>(
        x: Value<'lua>,
        lua: &'lua Lua,
    ) -> Result<Option<Cow<'static, RColor>>, E> {
        fn color_err(ty: &'static str) -> E {
            E::FromLuaConversionError {
                from: ty,
                to: "Color",
                message: Some("expected hex code or color name".to_owned()),
            }
        }
        let ty = x.type_name();
        let name = String::from_lua(x, lua).map_err(|_| color_err(ty))?;
        if name.is_empty() {
            Ok(None)
        } else if let Some(color) = Colors::named(&name) {
            Ok(Some(color))
        } else {
            Err(color_err(ty))
        }
    }
    pub fn ansi16() -> [RColor; 16] {
        let colors: &[RColor; 16] = XTERM_COLORS[..16].try_into().unwrap();
        colors.to_owned()
    }

    pub fn default_custom() -> [RColorPair; 16] {
        [
            RColorPair::new(0xFF8080, GlobalColor::Transparent),
            RColorPair::new(0xFFFF80, GlobalColor::Transparent),
            RColorPair::new(0x80FF80, GlobalColor::Transparent),
            RColorPair::new(0x80FFFF, GlobalColor::Transparent),
            RColorPair::new(0x0080FF, GlobalColor::Transparent),
            RColorPair::new(0xFF80C0, GlobalColor::Transparent),
            RColorPair::new(0xFF0000, GlobalColor::Transparent),
            RColorPair::new(0x0080C0, GlobalColor::Transparent),
            RColorPair::new(0x804040, GlobalColor::Transparent),
            RColorPair::new(0xFF8040, GlobalColor::Transparent),
            RColorPair::new(0x008080, GlobalColor::Transparent),
            RColorPair::new(0x004080, GlobalColor::Transparent),
            RColorPair::new(0xFF0080, GlobalColor::Transparent),
            RColorPair::new(0x008000, GlobalColor::Transparent),
            RColorPair::new(0x0000FF, GlobalColor::Transparent),
            RColorPair::new(0x686868, GlobalColor::Transparent),
        ]
    }
}
