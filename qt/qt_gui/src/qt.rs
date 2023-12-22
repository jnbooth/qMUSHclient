#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! C++ namespace: <span style='color: green;'>```Qt```</span>

/// Calls C++ function: <span style='color: green;'>```QTextCodec* Qt::codecForHtml(const QByteArray& ba)```</span>.
#[inline(always)]
pub unsafe fn codec_for_html(
    ba: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QByteArray>>,
) -> ::cpp_core::Ptr<::qt_core::QTextCodec> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_Qt_codecForHtml(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QByteArray>>::cast_into(ba)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ptr::from_raw(ffi_result)
}

/// <p>Converts the plain text string <i>plain</i> to an HTML-formatted paragraph while preserving most of its look.</p>
///
/// Calls C++ function: <span style='color: green;'>```QString Qt::convertFromPlainText(const QString& plain, Qt::WhiteSpaceMode mode = …)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qt-sub-qtgui.html#convertFromPlainText">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Converts the plain text string <i>plain</i> to an HTML-formatted paragraph while preserving most of its look.</p>
/// <p><i>mode</i> defines how whitespace is handled.</p>
/// <p>This function is defined in the <code>&lt;QTextDocument&gt;</code> header file.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qstring.html#toHtmlEscaped">QString::toHtmlEscaped</a>() and <a href="http://doc.qt.io/qt-5/qt-sub-qtgui.html#mightBeRichText">mightBeRichText</a>().</p></div>
#[inline(always)]
pub unsafe fn convert_from_plain_text_2a(
    plain: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QString>>,
    mode: ::qt_core::WhiteSpaceMode,
) -> ::cpp_core::CppBox<::qt_core::QString> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_Qt_convertFromPlainText(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QString>>::cast_into(plain)
                .as_raw_ptr(),
            mode,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Converts the plain text string <i>plain</i> to an HTML-formatted paragraph while preserving most of its look.</p>
///
/// Calls C++ function: <span style='color: green;'>```QString Qt::convertFromPlainText(const QString& plain)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qt-sub-qtgui.html#convertFromPlainText">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Converts the plain text string <i>plain</i> to an HTML-formatted paragraph while preserving most of its look.</p>
/// <p><i>mode</i> defines how whitespace is handled.</p>
/// <p>This function is defined in the <code>&lt;QTextDocument&gt;</code> header file.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qstring.html#toHtmlEscaped">QString::toHtmlEscaped</a>() and <a href="http://doc.qt.io/qt-5/qt-sub-qtgui.html#mightBeRichText">mightBeRichText</a>().</p></div>
#[inline(always)]
pub unsafe fn convert_from_plain_text_1a(
    plain: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QString>>,
) -> ::cpp_core::CppBox<::qt_core::QString> {
    let ffi_result = {
        crate::__ffi::ctr_qt_gui_ffi_Qt_convertFromPlainText1(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QString>>::cast_into(plain)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Returns <code>true</code> if the string <i>text</i> is likely to be rich text; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool Qt::mightBeRichText(const QString& arg1)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qt-sub-qtgui.html#mightBeRichText">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the string <i>text</i> is likely to be rich text; otherwise returns <code>false</code>.</p>
/// <p>This function uses a fast and therefore simple heuristic. It mainly checks whether there is something that looks like a tag before the first line break. Although the result may be correct for common cases, there is no guarantee.</p>
/// <p>This function is defined in the <code>&lt;QTextDocument&gt;</code> header file.</p></div>
#[inline(always)]
pub unsafe fn might_be_rich_text(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QString>>,
) -> bool {
    crate::__ffi::ctr_qt_gui_ffi_Qt_mightBeRichText(
        ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QString>>::cast_into(arg1).as_raw_ptr(),
    )
}
