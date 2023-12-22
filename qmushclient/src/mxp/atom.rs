use std::fmt::Write;
use std::os::raw::c_int;
use std::str;

use case_insensitive::ascii::{CaseFold, CaseFoldMap};
use case_insensitive::ToCaseFold;
use enumeration::{self, Enum, EnumSet};
use once_cell::sync::Lazy;

use super::Arguments;

/// Outstanding (unclosed) tags.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag {
    /// Name of tag we opened
    pub name: String,
    /// Was it secure mode at the time?
    pub secure: bool,
    /// Protected from reset?
    pub no_reset: bool,
    /// Index in a style's span list.
    pub span_index: usize,
    /// Position in the text document.
    pub text_index: c_int,
    /// Special replacement sequence for clickable links that use the text they contain.
    pub anchor_template: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum TagFlag {
    /// Tag is an open one (otherwise secure)
    Open,
    /// Tag is a command (doesn't have closing tag)
    Command,
    /// Tag is Pueblo-only
    Pueblo,
    /// Not closed by reset (eg. body)
    NoReset,
    /// Not really implemented (for <supports> tag)
    NotImp,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Action {
    /// eg. <send href="go west"> west
    Send,
    /// bold
    Bold,
    /// underline
    Underline,
    /// italic
    Italic,
    /// eg. <color fore=red back=blue>
    Color,
    /// version request
    Version,
    /// Font appearance
    Font,
    /// play sound
    Sound,
    /// send username
    User,
    /// send password
    Password,
    /// causes a new connect to open
    Relocate,
    /// frame
    Frame,
    /// destination frame
    Dest,
    /// show image
    Image,
    /// sound/image filter
    Filter,
    /// Hyperlink (secure)
    Hyperlink,
    /// Hard Line break (secure)
    Br,
    /// Level 1 heading (secure)
    H1,
    /// Level 2 heading (secure)
    H2,
    /// Level 3 heading (secure)
    H3,
    /// Level 4 heading (secure)
    H4,
    /// Level 5 heading (secure)
    H5,
    /// Level 6 heading (secure)
    H6,
    /// Horizontal rule (secure)
    Hr,
    /// non-breaking newline
    NoBr,
    /// Paragraph break (secure)
    P,
    /// Strikethrough
    Strike,
    /// Client script (secure)
    Script,
    /// Small text
    Small,
    /// Non-proportional font
    Tt,
    /// Unordered list
    Ul,
    /// Ordered list
    Ol,
    /// List item
    Li,
    /// Sample text
    Samp,
    /// Centre text
    Center,
    /// Highlight text
    High,
    /// Set variable
    Var,
    /// AFK - away from keyboard time
    Afk,

    // recent
    /// gauge
    Gauge,
    /// status
    Stat,
    /// expire
    Expire,

    /// close all open tags
    Reset,
    /// MXP command (eg. MXP OFF)
    Mxp,
    /// what commands we support
    Support,

    /// client options set
    SetOption,
    /// server sets option
    RecommendOption,

    // Pueblo
    /// Preformatted text
    Pre,
    Body,
    Head,
    Html,
    Title,
    Img,
    XchPage,
    XchPane,
}

/// Atomic MXP tags that we recognise, e.g. <b>.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    /// Tag name, e.g. bold
    pub name: &'static str,
    /// Secure, Command, etc.
    pub flags: EnumSet<TagFlag>,
    /// Its action.
    pub action: Action,
    /// Supported arguments, e.g. href, hint
    pub args: &'static [&'static CaseFold<str>],
}

impl Atom {
    pub fn get(name: &str) -> Option<Self> {
        ALL_ATOMS.get(name).map(Clone::clone)
    }

    pub fn supported(args: Arguments) -> String {
        const ERR: &str = "unexpected format error in Atom::supported";
        let mut supported = String::from("\x1B[1z<SUPPORTS ");
        if args.is_empty() {
            for atom in ALL_ATOMS.values() {
                write!(supported, "+{} ", atom.name).expect(ERR);
                for atom_arg in atom.args {
                    write!(supported, "+{}.{} ", atom.name, atom_arg).expect(ERR);
                }
            }
        } else {
            for arg in args.values() {
                let mut questions = arg.split('.');
                let tag = questions.next().unwrap();
                match Atom::get(tag) {
                    None => write!(supported, "-{} ", tag).expect(ERR),
                    Some(atom) if atom.flags.contains(TagFlag::NotImp) => {
                        write!(supported, "-{} ", tag).expect(ERR)
                    }
                    Some(atom) => match questions.next() {
                        None => write!(supported, "+{} ", tag).expect(ERR),
                        Some("*") => {
                            for atom_arg in atom.args {
                                write!(supported, "+{}.{} ", atom.name, atom_arg).expect(ERR);
                            }
                        }
                        Some(subtag) => {
                            let can = if atom.args.contains(&subtag.to_case_fold()) {
                                '+'
                            } else {
                                '-'
                            };
                            write!(supported, "{}{}", can, subtag).expect(ERR);
                        }
                    },
                }
            }
        }
        supported.push_str(">\n");
        supported
    }
}

static ALL_ATOMS: Lazy<CaseFoldMap<String, Atom>> = Lazy::new(|| {
    let mut all = CaseFoldMap::new();
    let mut add = |name: &'static str, flags, action, args| {
        all.insert(
            name.to_owned(),
            Atom {
                name,
                flags,
                action,
                args,
            },
        )
    };

    use Action::*;
    use TagFlag::*;
    add("bold", enums![Open], Bold, &[]);
    add("b", enums![Open], Bold, &[]);
    add("high", enums![Open], High, &[]);
    add("h", enums![Open], High, &[]);
    add("underline", enums![Open], Underline, &[]);
    add("u", enums![Open], Underline, &[]);
    add("italic", enums![Open], Italic, &[]);
    add("i", enums![Open], Italic, &[]);
    add("em", enums![Open], Italic, &[]);
    const COLOR_ARGS: &[&CaseFold<str>] = &[CaseFold::borrow("fore"), CaseFold::borrow("back")];
    add("color", enums![Open], Color, COLOR_ARGS);
    add("c", enums![Open], Color, COLOR_ARGS);
    add("s", enums![Open, NotImp], Strike, &[]);
    add("strike", enums![Open, NotImp], Strike, &[]);
    add("strong", enums![Open], Bold, &[]);
    add("small", enums![Open, NotImp], Small, &[]);
    add("tt", enums![Open, NotImp], Tt, &[]);
    add("frame", enums![NotImp], Frame, &[]);
    add("dest", enums![NotImp], Dest, &[]);
    const IMAGE_ARGS: &[&CaseFold<str>] = &[CaseFold::borrow("url"), CaseFold::borrow("fname")];
    add("image", enums![Command, NotImp], Image, IMAGE_ARGS);
    add("filter", enums![NotImp], Filter, &[]);
    const A_ARGS: &[&CaseFold<str>] = &[
        CaseFold::borrow("href"),
        CaseFold::borrow("xch_cmd"),
        CaseFold::borrow("xch_hint"),
    ];
    add("a", enums![], Hyperlink, A_ARGS);
    add("h1", enums![NotImp], H1, &[]);
    add("h2", enums![NotImp], H2, &[]);
    add("h3", enums![NotImp], H3, &[]);
    add("h4", enums![NotImp], H4, &[]);
    add("h5", enums![NotImp], H5, &[]);
    add("h6", enums![NotImp], H6, &[]);
    add("hr", enums![Command], Hr, &[]);
    add("nobr", enums![NotImp], NoBr, &[]);
    add("p", enums![], P, &[]);
    add("script", enums![NotImp], Script, &[]);
    add("ul", enums![], Ul, &[]);
    add("ol", enums![], Ol, &[]);
    add("samp", enums![], Samp, &[]);
    add("center", enums![NotImp], Center, &[]);
    add("var", enums![], Var, &[]);
    add("v", enums![], Var, &[]);
    add("gauge", enums![NotImp], Gauge, &[]);
    add("stat", enums![NotImp], Stat, &[]);
    add("expire", enums![NotImp], Expire, &[]);
    add("li", enums![Command], Li, &[]);
    add("sound", enums![Command, NotImp], Sound, &[]);
    add("music", enums![Command, NotImp], Sound, &[]);
    add("br", enums![Command], Br, &[]);
    add("username", enums![Command], User, &[]);
    add("user", enums![Command], User, &[]);
    add("password", enums![Command], Password, &[]);
    add("pass", enums![Command], Password, &[]);
    add("relocate", enums![Command, NotImp], Relocate, &[]);
    add("version", enums![Command], Version, &[]);
    add("reset", enums![Command], Reset, &[]);
    const MXP_ARGS: &[&CaseFold<str>] = &[CaseFold::borrow("off")];
    add("mxp", enums![Command], Mxp, MXP_ARGS);
    add("support", enums![Command], Support, &[]);
    add("option", enums![Command], SetOption, &[]);
    add("afk", enums![Command], Afk, &[]);
    add("recommend_option", enums![Command], RecommendOption, &[]);
    add("pre", enums![Pueblo], Pre, &[]);
    add("body", enums![Pueblo, NoReset], Body, &[]);
    add("head", enums![Pueblo, NoReset], Head, &[]);
    add("html", enums![Pueblo, NoReset], Html, &[]);
    add("title", enums![Pueblo], Title, &[]);
    const IMG_ARGS: &[&CaseFold<str>] = &[CaseFold::borrow("src"), CaseFold::borrow("xch_mode")];
    add("img", enums![Pueblo, Command], Img, IMG_ARGS);
    add("xch_page", enums![Pueblo, Command], XchPage, &[]);
    add("xch_pane", enums![Pueblo, Command, NotImp], XchPane, &[]);
    const FONT_ARGS: &[&CaseFold<str>] = &[
        CaseFold::borrow("color"),
        CaseFold::borrow("back"),
        CaseFold::borrow("fgcolor"),
        CaseFold::borrow("bgcolor"),
    ];
    add("font", enums![Open], Font, FONT_ARGS);
    const ADD_ARGS: &[&CaseFold<str>] = &[
        CaseFold::borrow("href"),
        CaseFold::borrow("hint"),
        CaseFold::borrow("xch_cmd"),
        CaseFold::borrow("xch_hint"),
        CaseFold::borrow("prompt"),
    ];
    add("send", enums![], Send, ADD_ARGS);

    all
});
