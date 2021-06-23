use crate::enums::{Enum, EnumSet};
use std::str;

/// Outstanding (unclosed) tags.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag {
    /// Name of tag we opened
    pub name: String,
    /// Was it secure mode at the time?
    pub secure: bool,
    /// Protected from reset?
    pub no_reset: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum TagFlag {
    /// Tag is an open one (otherwise secure)
    Open,
    Command,
    Pueblo,
    Mxp,
    NoReset,
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

    // non-standard yet
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
    pub args: &'static [&'static str],
}

impl Atom {
    pub const fn exists(name: &str) -> bool {
        Self::get(name).is_some()
    }

    // Atoms, true to their name, are very small. An Atom is slightly more than twice the size of an
    // &Atom. Furthermore, creating an Atom involves no heap allocations. So there is no good
    // reason to use a repository of &'static Atoms instead of simply creating new ones as needed.
    pub const fn get(name: &str) -> Option<Self> {
        const fn atom(
            name: &'static str,
            flags: EnumSet<TagFlag>,
            action: Action,
            args: &'static [&'static str],
        ) -> Option<Atom> {
            Some(Atom {
                name,
                flags,
                action,
                args,
            })
        }
        use Action::*;
        use TagFlag::*;
        match name.as_bytes() {
            b"bold" => atom("bold", enums![Open], Bold, &[]),
            b"b" => atom("b", enums![Open], Bold, &[]),
            b"high" => atom("high", enums![Open], High, &[]),
            b"h" => atom("h", enums![Open], High, &[]),
            b"underline" => atom("underline", enums![Open], Underline, &[]),
            b"u" => atom("u", enums![Open], Underline, &[]),
            b"italic" => atom("italic", enums![Open], Italic, &[]),
            b"i" => atom("i", enums![Open], Italic, &[]),
            b"em" => atom("em", enums![Open], Italic, &[]),
            b"color" => atom("color", enums![Open], Color, &["fore", "back"]),
            b"c" => atom("c", enums![Open], Color, &["fore", "back"]),
            b"s" => atom("s", enums![Open, NotImp], Strike, &[]),
            b"strike" => atom("strike", enums![Open, NotImp], Strike, &[]),
            b"strong" => atom("strong", enums![Open], Bold, &[]),
            b"small" => atom("small", enums![Open, NotImp], Small, &[]),
            b"tt" => atom("tt", enums![Open, NotImp], Tt, &[]),
            b"frame" => atom("frame", enums![NotImp], Frame, &[]),
            b"dest" => atom("dest", enums![NotImp], Dest, &[]),
            b"image" => atom("image", enums![Command, NotImp], Image, &["url", "fname"]),
            b"filter" => atom("filter", enums![NotImp], Filter, &[]),
            b"a" => atom("a", enums![], Hyperlink, &["href", "xch_cmd", "xch_hint"]),
            b"h1" => atom("h1", enums![NotImp], H1, &[]),
            b"h2" => atom("h2", enums![NotImp], H2, &[]),
            b"h3" => atom("h3", enums![NotImp], H3, &[]),
            b"h4" => atom("h4", enums![NotImp], H4, &[]),
            b"h5" => atom("h5", enums![NotImp], H5, &[]),
            b"h6" => atom("h6", enums![NotImp], H6, &[]),
            b"hr" => atom("hr", enums![Command], Hr, &[]),
            b"nobr" => atom("nobr", enums![NotImp], NoBr, &[]),
            b"p" => atom("p", enums![], P, &[]),
            b"script" => atom("script", enums![NotImp], Script, &[]),
            b"ul" => atom("ul", enums![], Ul, &[]),
            b"ol" => atom("ol", enums![], Ol, &[]),
            b"samp" => atom("samp", enums![], Samp, &[]),
            b"center" => atom("center", enums![NotImp], Center, &[]),
            b"var" => atom("var", enums![], Var, &[]),
            b"v" => atom("v", enums![], Var, &[]),
            b"gauge" => atom("gauge", enums![NotImp], Gauge, &[]),
            b"stat" => atom("stat", enums![NotImp], Stat, &[]),
            b"expire" => atom("expire", enums![NotImp], Expire, &[]),
            // strictly speaking <LI> isn't a command, but few people bother with </li>
            b"li" => atom("li", enums![Command], Li, &[]),
            b"sound" => atom("sound", enums![Command, NotImp], Sound, &[]),
            b"music" => atom("music", enums![Command, NotImp], Sound, &[]),
            b"br" => atom("br", enums![Command], Br, &[]),
            b"username" => atom("username", enums![Command], User, &[]),
            b"user" => atom("user", enums![Command], User, &[]),
            b"password" => atom("password", enums![Command], Password, &[]),
            b"pass" => atom("pass", enums![Command], Password, &[]),
            b"relocate" => atom("relocate", enums![Command, NotImp], Relocate, &[]),
            b"version" => atom("version", enums![Command], Version, &[]),
            b"reset" => atom("reset", enums![Command], Reset, &[]),
            b"mxp" => atom("mxp", enums![Command], Reset, &["off"]),
            b"support" => atom("support", enums![Command], Support, &[]),
            b"option" => atom("option", enums![Command], SetOption, &[]),
            b"afk" => atom("afk", enums![Command], Afk, &[]),
            b"recommend_option" => atom("recommend_option", enums![Command], RecommendOption, &[]),
            b"pre" => atom("pre", enums![Pueblo], Pre, &[]),
            b"body" => atom("body", enums![Pueblo, NoReset], Body, &[]),
            b"head" => atom("head", enums![Pueblo, NoReset], Head, &[]),
            b"html" => atom("html", enums![Pueblo, NoReset], Html, &[]),
            b"title" => atom("title", enums![Pueblo], Title, &[]),
            b"img" => atom("img", enums![Pueblo, Command], Img, &["src", "xch_mode"]),
            b"xch_page" => atom("xch_page", enums![Pueblo, Command], XchPage, &[]),
            b"xch_pane" => atom("xch_pane", enums![Pueblo, Command, NotImp], XchPane, &[]),
            b"font" => atom(
                "font",
                enums![Open],
                Font,
                &["color", "back", "fgcolor", "bgcolor"],
            ),
            b"send" => atom(
                "send",
                enums![],
                Send,
                &["href", "hint", "xch_cmd", "xch_hint", "prompt"],
            ),
            _ => None,
        }
    }
}
