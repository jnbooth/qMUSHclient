#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! C++ namespace: <span style='color: green;'>```QtPrivate```</span>

/// Calls C++ function: <span style='color: green;'>```QPixelFormat QtPrivate::QPixelFormat_createYUV(QPixelFormat::YUVLayout yuvLayout, unsigned char alphaSize, QPixelFormat::AlphaUsage alphaUsage, QPixelFormat::AlphaPosition alphaPosition, QPixelFormat::AlphaPremultiplied premultiplied, QPixelFormat::TypeInterpretation typeInterpretation, QPixelFormat::ByteOrder byteOrder)```</span>.
#[inline(always)]
pub unsafe fn q_pixel_format_create_y_u_v(
    yuv_layout: crate::q_pixel_format::YUVLayout,
    alpha_size: ::std::os::raw::c_uchar,
    alpha_usage: crate::q_pixel_format::AlphaUsage,
    alpha_position: crate::q_pixel_format::AlphaPosition,
    premultiplied: crate::q_pixel_format::AlphaPremultiplied,
    type_interpretation: crate::q_pixel_format::TypeInterpretation,
    byte_order: crate::q_pixel_format::ByteOrder,
) -> ::cpp_core::CppBox<crate::QPixelFormat> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_QtPrivate_QPixelFormat_createYUV(
            yuv_layout,
            alpha_size,
            alpha_usage,
            alpha_position,
            premultiplied,
            type_interpretation,
            byte_order,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}
