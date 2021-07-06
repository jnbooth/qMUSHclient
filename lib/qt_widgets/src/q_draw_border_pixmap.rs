#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! C++ namespace: <span style='color: green;'>```QDrawBorderPixmap```</span>

/// C++ enum: <span style='color: green;'>```QDrawBorderPixmap::DrawingHint```</span>.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DrawingHint(::std::os::raw::c_int);

impl From<::std::os::raw::c_int> for DrawingHint {
    fn from(value: ::std::os::raw::c_int) -> Self {
        DrawingHint(value)
    }
}

impl From<DrawingHint> for ::std::os::raw::c_int {
    fn from(value: DrawingHint) -> Self {
        value.0
    }
}

impl DrawingHint {
    pub const fn from_int(value: ::std::os::raw::c_int) -> Self {
        DrawingHint(value)
    }

    pub const fn to_int(self) -> ::std::os::raw::c_int {
        self.0
    }
}

impl DrawingHint {
    /// C++ enum variant: <span style='color: green;'>```OpaqueTopLeft = 1```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueTopLeft: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(1);
    /// C++ enum variant: <span style='color: green;'>```OpaqueTop = 2```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueTop: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(2);
    /// C++ enum variant: <span style='color: green;'>```OpaqueTopRight = 4```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueTopRight: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(4);
    /// C++ enum variant: <span style='color: green;'>```OpaqueLeft = 8```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueLeft: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(8);
    /// C++ enum variant: <span style='color: green;'>```OpaqueCenter = 16```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueCenter: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(16);
    /// C++ enum variant: <span style='color: green;'>```OpaqueRight = 32```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueRight: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(32);
    /// C++ enum variant: <span style='color: green;'>```OpaqueBottomLeft = 64```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueBottomLeft: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(64);
    /// C++ enum variant: <span style='color: green;'>```OpaqueBottom = 128```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueBottom: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(128);
    /// C++ enum variant: <span style='color: green;'>```OpaqueBottomRight = 256```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueBottomRight: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(256);
    /// C++ enum variant: <span style='color: green;'>```OpaqueCorners = 325```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueCorners: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(325);
    /// C++ enum variant: <span style='color: green;'>```OpaqueEdges = 170```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueEdges: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(170);
    /// C++ enum variant: <span style='color: green;'>```OpaqueFrame = 495```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueFrame: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(495);
    /// C++ enum variant: <span style='color: green;'>```OpaqueAll = 511```</span>
    #[allow(non_upper_case_globals)]
    pub const OpaqueAll: crate::q_draw_border_pixmap::DrawingHint =
        crate::q_draw_border_pixmap::DrawingHint(511);
}

impl From<crate::q_draw_border_pixmap::DrawingHint>
    for ::qt_core::QFlags<crate::q_draw_border_pixmap::DrawingHint>
{
    fn from(value: crate::q_draw_border_pixmap::DrawingHint) -> Self {
        Self::from(value.to_int())
    }
}

impl<T: Into<::qt_core::QFlags<crate::q_draw_border_pixmap::DrawingHint>>> std::ops::BitOr<T>
    for crate::q_draw_border_pixmap::DrawingHint
{
    type Output = ::qt_core::QFlags<crate::q_draw_border_pixmap::DrawingHint>;
    fn bitor(self, rhs: T) -> ::qt_core::QFlags<crate::q_draw_border_pixmap::DrawingHint> {
        Into::<::qt_core::QFlags<crate::q_draw_border_pixmap::DrawingHint>>::into(self) | rhs
    }
}
