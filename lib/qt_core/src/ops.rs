#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! Functions that provide access to C++ operators

/// <p>This is an overloaded function.</p>
///
/// Calls C++ function: <span style='color: green;'>```QByteArray operator+(const char* a1, const QByteArray& a2)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qbytearray.html#operator-2b-3">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
/// <p>Returns a byte array that is the result of concatenating string <i>a1</i> and byte array <i>a2</i>.</p></div>
#[inline(always)]
pub unsafe fn add_char_q_byte_array(
    a1: *const ::std::os::raw::c_char,
    a2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QByteArray>>,
) -> ::cpp_core::CppBox<crate::QByteArray> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_11(
            a1,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QByteArray>>::cast_into(a2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>This is an overloaded function.</p>
///
/// Calls C++ function: <span style='color: green;'>```QByteArray operator+(char a1, const QByteArray& a2)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qbytearray.html#operator-2b-4">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
/// <p>Returns a byte array that is the result of concatenating character <i>a1</i> and byte array <i>a2</i>.</p></div>
#[inline(always)]
pub unsafe fn add_char_q_byte_array2(
    a1: ::std::os::raw::c_char,
    a2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QByteArray>>,
) -> ::cpp_core::CppBox<crate::QByteArray> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_12(
            a1,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QByteArray>>::cast_into(a2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QString operator+(const char* s1, const QString& s2)```</span>.
#[inline(always)]
pub unsafe fn add_char_q_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QString>>,
) -> ::cpp_core::CppBox<crate::QString> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_23(
            s1,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QString>>::cast_into(s2).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QString operator+(char c, const QString& s)```</span>.
#[inline(always)]
pub unsafe fn add_char_q_string2(
    c: ::std::os::raw::c_char,
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QString>>,
) -> ::cpp_core::CppBox<crate::QString> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_24(
            c,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QString>>::cast_into(s).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QMargins operator+(int lhs, const QMargins& rhs)```</span>.
#[inline(always)]
pub unsafe fn add_int_q_margins(
    lhs: ::std::os::raw::c_int,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMargins>>,
) -> ::cpp_core::CppBox<crate::QMargins> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_91(
            lhs,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMargins>>::cast_into(rhs).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QMarginsF operator+(double lhs, const QMarginsF& rhs)```</span>.
#[inline(always)]
pub unsafe fn add_double_q_margins_f(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMarginsF>>,
) -> ::cpp_core::CppBox<crate::QMarginsF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_104(
            lhs,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMarginsF>>::cast_into(rhs).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```double operator+(double lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn add_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_double {
    crate::__ffi::ctr_qt_core_ffi_operator_450(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```float operator+(float lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn add_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_float {
    crate::__ffi::ctr_qt_core_ffi_operator_458(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```double operator+(int lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn add_int_qfloat16(
    lhs: ::std::os::raw::c_int,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_double {
    crate::__ffi::ctr_qt_core_ffi_operator_466(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> operator|(QUrl::UrlFormattingOption f1, QUrl::UrlFormattingOption f2)```</span>.
#[inline(always)]
pub unsafe fn bit_or_2_url_formatting_option(
    f1: crate::q_url::UrlFormattingOption,
    f2: crate::q_url::UrlFormattingOption,
) -> ::cpp_core::CppBox<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption> {
    let ffi_result = { crate::__ffi::ctr_qt_core_ffi_operator_127(f1, f2) };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> operator|(QUrl::UrlFormattingOption f1, QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> f2)```</span>.
#[inline(always)]
pub unsafe fn bit_or_url_formatting_option_q_url_two_flags_of_url_formatting_option_component_formatting_option(
    f1: crate::q_url::UrlFormattingOption,
    f2: impl ::cpp_core::CastInto<
        ::cpp_core::Ref<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption>,
    >,
) -> ::cpp_core::CppBox<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_128(
            f1,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption>,
            >::cast_into(f2)
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> operator|(QUrl::UrlFormattingOption i, QUrl::ComponentFormattingOption f)```</span>.
#[inline(always)]
pub unsafe fn bit_or_url_formatting_option_component_formatting_option(
    i: crate::q_url::UrlFormattingOption,
    f: crate::q_url::ComponentFormattingOption,
) -> ::cpp_core::CppBox<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption> {
    let ffi_result = { crate::__ffi::ctr_qt_core_ffi_operator_129(i, f) };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> operator|(QUrl::UrlFormattingOption i, QFlags<QUrl::ComponentFormattingOption> f)```</span>.
#[inline(always)]
pub unsafe fn bit_or_url_formatting_option_q_flags_component_formatting_option(
    i: crate::q_url::UrlFormattingOption,
    f: crate::QFlags<crate::q_url::ComponentFormattingOption>,
) -> ::cpp_core::CppBox<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption> {
    let ffi_result = { crate::__ffi::ctr_qt_core_ffi_operator_130(i, f.to_int()) };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> operator|(QUrl::ComponentFormattingOption f, QUrl::UrlFormattingOption i)```</span>.
#[inline(always)]
pub unsafe fn bit_or_component_formatting_option_url_formatting_option(
    f: crate::q_url::ComponentFormattingOption,
    i: crate::q_url::UrlFormattingOption,
) -> ::cpp_core::CppBox<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption> {
    let ffi_result = { crate::__ffi::ctr_qt_core_ffi_operator_131(f, i) };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> operator|(QFlags<QUrl::ComponentFormattingOption> f, QUrl::UrlFormattingOption i)```</span>.
#[inline(always)]
pub unsafe fn bit_or_q_flags_component_formatting_option_url_formatting_option(
    f: crate::QFlags<crate::q_url::ComponentFormattingOption>,
    i: crate::q_url::UrlFormattingOption,
) -> ::cpp_core::CppBox<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption> {
    let ffi_result = { crate::__ffi::ctr_qt_core_ffi_operator_132(f.to_int(), i) };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> operator|(QUrl::ComponentFormattingOption f, QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> i)```</span>.
#[inline(always)]
pub unsafe fn bit_or_component_formatting_option_q_url_two_flags_of_url_formatting_option_component_formatting_option(
    f: crate::q_url::ComponentFormattingOption,
    i: impl ::cpp_core::CastInto<
        ::cpp_core::Ref<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption>,
    >,
) -> ::cpp_core::CppBox<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_134(
            f,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption>,
            >::cast_into(i)
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> operator|(QFlags<QUrl::ComponentFormattingOption> f, QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> i)```</span>.
#[inline(always)]
pub unsafe fn bit_or_q_flags_component_formatting_option_q_url_two_flags_of_url_formatting_option_component_formatting_option(
    f: crate::QFlags<crate::q_url::ComponentFormattingOption>,
    i: impl ::cpp_core::CastInto<
        ::cpp_core::Ref<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption>,
    >,
) -> ::cpp_core::CppBox<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_135(
            f.to_int(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::QUrlTwoFlagsOfUrlFormattingOptionComponentFormattingOption>,
            >::cast_into(i)
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```double operator/(double lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn div_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_double {
    crate::__ffi::ctr_qt_core_ffi_operator_456(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```float operator/(float lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn div_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_float {
    crate::__ffi::ctr_qt_core_ffi_operator_464(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```double operator/(int lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn div_int_qfloat16(
    lhs: ::std::os::raw::c_int,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_double {
    crate::__ffi::ctr_qt_core_ffi_operator_472(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(const char* a1, const QByteArray& a2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn eq_char_q_byte_array(
    a1: *const ::std::os::raw::c_char,
    a2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QByteArray>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__8(
        a1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QByteArray>>::cast_into(a2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(const char* s1, const QString& s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn eq_char_q_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QString>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__24(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QString>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(const char* s1, QLatin1String s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn eq_char_q_latin1_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1String>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__28(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1String>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(const char* s1, const QStringRef& s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn eq_char_q_string_ref(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QStringRef>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__86(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QStringRef>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(QCborTag t, QCborKnownTags kt)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn eq_q_cbor_tag_q_cbor_known_tags(
    t: crate::QCborTag,
    kt: crate::QCborKnownTags,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1128(t, kt)
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(QCborKnownTags kt, QCborTag t)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn eq_q_cbor_known_tags_q_cbor_tag(
    kt: crate::QCborKnownTags,
    t: crate::QCborTag,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1129(kt, t)
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(double lhs, qfloat16 rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn eq_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1167(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(float lhs, qfloat16 rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn eq_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1175(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(int a, qfloat16 b)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn eq_int_qfloat16(
    a: ::std::os::raw::c_int,
    b: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1183(
        a,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(b).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(char lhs, QLatin1Char rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-eq-eq">C++ documentation</a> for <span style='color: green;'>```bool operator==(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if <i>c1</i> and <i>c2</i> are the same Unicode character; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn eq_char_q_latin1_char(
    lhs: ::std::os::raw::c_char,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1Char>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1340(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1Char>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator>=(const char* a1, const QByteArray& a2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-gt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn ge_char_q_byte_array(
    a1: *const ::std::os::raw::c_char,
    a2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QByteArray>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__17(
        a1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QByteArray>>::cast_into(a2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator>=(const char* s1, const QString& s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-gt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn ge_char_q_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QString>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__27(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QString>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator>=(const char* s1, QLatin1String s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-gt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn ge_char_q_latin1_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1String>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__31(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1String>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator>=(const char* s1, const QStringRef& s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-gt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn ge_char_q_string_ref(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QStringRef>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__89(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QStringRef>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator>=(double lhs, qfloat16 rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-gt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn ge_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1163(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator>=(float lhs, qfloat16 rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-gt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn ge_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1171(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator>=(int a, qfloat16 b)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-gt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn ge_int_qfloat16(
    a: ::std::os::raw::c_int,
    b: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1179(
        a,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(b).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator>=(char lhs, QLatin1Char rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-gt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is greater than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn ge_char_q_latin1_char(
    lhs: ::std::os::raw::c_char,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1Char>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1343(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1Char>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator>(const char* a1, const QByteArray& a2)```</span>.
#[inline(always)]
pub unsafe fn gt_char_q_byte_array(
    a1: *const ::std::os::raw::c_char,
    a2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QByteArray>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_7(
        a1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QByteArray>>::cast_into(a2).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator>(const char* s1, const QString& s2)```</span>.
#[inline(always)]
pub unsafe fn gt_char_q_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QString>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_16(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QString>>::cast_into(s2).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator>(const char* s1, QLatin1String s2)```</span>.
#[inline(always)]
pub unsafe fn gt_char_q_latin1_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1String>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_18(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1String>>::cast_into(s2).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator>(const char* s1, const QStringRef& s2)```</span>.
#[inline(always)]
pub unsafe fn gt_char_q_string_ref(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QStringRef>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_55(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QStringRef>>::cast_into(s2).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator>(double lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn gt_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_482(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator>(float lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn gt_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_486(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator>(int a, qfloat16 b)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn gt_int_qfloat16(
    a: ::std::os::raw::c_int,
    b: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_488(
        a,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(b).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator>(char lhs, QLatin1Char rhs)```</span>.
#[inline(always)]
pub unsafe fn gt_char_q_latin1_char(
    lhs: ::std::os::raw::c_char,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1Char>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_563(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1Char>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator<=(const char* a1, const QByteArray& a2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-lt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn le_char_q_byte_array(
    a1: *const ::std::os::raw::c_char,
    a2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QByteArray>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__14(
        a1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QByteArray>>::cast_into(a2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator<=(const char* s1, const QString& s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-lt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn le_char_q_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QString>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__26(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QString>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator<=(const char* s1, QLatin1String s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-lt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn le_char_q_latin1_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1String>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__30(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1String>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator<=(const char* s1, const QStringRef& s2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-lt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn le_char_q_string_ref(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QStringRef>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__88(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QStringRef>>::cast_into(s2).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator<=(double lhs, qfloat16 rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-lt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn le_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1165(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator<=(float lhs, qfloat16 rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-lt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn le_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1173(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator<=(int a, qfloat16 b)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-lt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn le_int_qfloat16(
    a: ::std::os::raw::c_int,
    b: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1181(
        a,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(b).as_raw_ptr(),
    )
}

/// <p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator<=(char lhs, QLatin1Char rhs)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qchar.html#operator-lt-eq">C++ documentation</a> for <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if the numeric Unicode value of <i>c1</i> is less than or equal to that of <i>c2</i>; otherwise returns <code>false</code>.</p></div>
#[inline(always)]
pub unsafe fn le_char_q_latin1_char(
    lhs: ::std::os::raw::c_char,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1Char>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator__1342(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1Char>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator<(const char* a1, const QByteArray& a2)```</span>.
#[inline(always)]
pub unsafe fn lt_char_q_byte_array(
    a1: *const ::std::os::raw::c_char,
    a2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QByteArray>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_4(
        a1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QByteArray>>::cast_into(a2).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator<(const char* s1, const QString& s2)```</span>.
#[inline(always)]
pub unsafe fn lt_char_q_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QString>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_15(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QString>>::cast_into(s2).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator<(const char* s1, QLatin1String s2)```</span>.
#[inline(always)]
pub unsafe fn lt_char_q_latin1_string(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1String>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_17(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1String>>::cast_into(s2).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator<(const char* s1, const QStringRef& s2)```</span>.
#[inline(always)]
pub unsafe fn lt_char_q_string_ref(
    s1: *const ::std::os::raw::c_char,
    s2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QStringRef>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_54(
        s1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QStringRef>>::cast_into(s2).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator<(double lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn lt_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_480(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator<(float lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn lt_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_484(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator<(int a, qfloat16 b)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn lt_int_qfloat16(
    a: ::std::os::raw::c_int,
    b: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_490(
        a,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(b).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```bool operator<(char lhs, QLatin1Char rhs)```</span>.
#[inline(always)]
pub unsafe fn lt_char_q_latin1_char(
    lhs: ::std::os::raw::c_char,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QLatin1Char>>,
) -> bool {
    crate::__ffi::ctr_qt_core_ffi_operator_562(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QLatin1Char>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```QPoint operator*(float factor, const QPoint& p)```</span>.
#[inline(always)]
pub unsafe fn mul_float_q_point(
    factor: ::std::os::raw::c_float,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPoint>>,
) -> ::cpp_core::CppBox<crate::QPoint> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_75(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPoint>>::cast_into(p).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPoint operator*(double factor, const QPoint& p)```</span>.
#[inline(always)]
pub unsafe fn mul_double_q_point(
    factor: ::std::os::raw::c_double,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPoint>>,
) -> ::cpp_core::CppBox<crate::QPoint> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_76(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPoint>>::cast_into(p).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPoint operator*(int factor, const QPoint& p)```</span>.
#[inline(always)]
pub unsafe fn mul_int_q_point(
    factor: ::std::os::raw::c_int,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPoint>>,
) -> ::cpp_core::CppBox<crate::QPoint> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_77(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPoint>>::cast_into(p).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QPointF operator*(double c, const QPointF& p)```</span>.
#[inline(always)]
pub unsafe fn mul_double_q_point_f(
    c: ::std::os::raw::c_double,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPointF>>,
) -> ::cpp_core::CppBox<crate::QPointF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_84(
            c,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPointF>>::cast_into(p).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>This is an overloaded function.</p>
///
/// Calls C++ function: <span style='color: green;'>```QMargins operator*(int factor, const QMargins& margins)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qmargins.html#operator-2a-1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
/// <p>Returns a <a href="http://doc.qt.io/qt-5/qmargins.html">QMargins</a> object that is formed by multiplying each component of the given <i>margins</i> by <i>factor</i>.</p>
/// <p>This function was introduced in  Qt 5.1.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qmargins.html#operator-2a-eq">QMargins::operator*=</a>() and <a href="http://doc.qt.io/qt-5/qmargins.html#operator-2f-eq">QMargins::operator/=</a>().</p></div>
#[inline(always)]
pub unsafe fn mul_int_q_margins(
    factor: ::std::os::raw::c_int,
    margins: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMargins>>,
) -> ::cpp_core::CppBox<crate::QMargins> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_94(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMargins>>::cast_into(margins)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>This is an overloaded function.</p>
///
/// Calls C++ function: <span style='color: green;'>```QMargins operator*(double factor, const QMargins& margins)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qmargins.html#operator-2a-3">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This is an overloaded function.</p>
/// <p>Returns a <a href="http://doc.qt.io/qt-5/qmargins.html">QMargins</a> object that is formed by multiplying each component of the given <i>margins</i> by <i>factor</i>.</p>
/// <p>This function was introduced in  Qt 5.1.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qmargins.html#operator-2a-eq">QMargins::operator*=</a>() and <a href="http://doc.qt.io/qt-5/qmargins.html#operator-2f-eq">QMargins::operator/=</a>().</p></div>
#[inline(always)]
pub unsafe fn mul_double_q_margins(
    factor: ::std::os::raw::c_double,
    margins: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMargins>>,
) -> ::cpp_core::CppBox<crate::QMargins> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_96(
            factor,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMargins>>::cast_into(margins)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QMarginsF operator*(double lhs, const QMarginsF& rhs)```</span>.
#[inline(always)]
pub unsafe fn mul_double_q_margins_f(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMarginsF>>,
) -> ::cpp_core::CppBox<crate::QMarginsF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_107(
            lhs,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMarginsF>>::cast_into(rhs).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QSize operator*(double c, const QSize& s)```</span>.
#[inline(always)]
pub unsafe fn mul_double_q_size(
    c: ::std::os::raw::c_double,
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSize>>,
) -> ::cpp_core::CppBox<crate::QSize> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_114(
            c,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSize>>::cast_into(s).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```QSizeF operator*(double c, const QSizeF& s)```</span>.
#[inline(always)]
pub unsafe fn mul_double_q_size_f(
    c: ::std::os::raw::c_double,
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSizeF>>,
) -> ::cpp_core::CppBox<crate::QSizeF> {
    let ffi_result = {
        crate::__ffi::ctr_qt_core_ffi_operator_119(
            c,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSizeF>>::cast_into(s).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// Calls C++ function: <span style='color: green;'>```double operator*(double lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn mul_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_double {
    crate::__ffi::ctr_qt_core_ffi_operator_454(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```float operator*(float lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn mul_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_float {
    crate::__ffi::ctr_qt_core_ffi_operator_462(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```double operator*(int lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn mul_int_qfloat16(
    lhs: ::std::os::raw::c_int,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_double {
    crate::__ffi::ctr_qt_core_ffi_operator_470(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```double operator-(double lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn sub_double_qfloat16(
    lhs: ::std::os::raw::c_double,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_double {
    crate::__ffi::ctr_qt_core_ffi_operator_452(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```float operator-(float lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn sub_float_qfloat16(
    lhs: ::std::os::raw::c_float,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_float {
    crate::__ffi::ctr_qt_core_ffi_operator_460(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}

/// Calls C++ function: <span style='color: green;'>```double operator-(int lhs, qfloat16 rhs)```</span>.
#[inline(always)]
#[cfg_attr(
    feature = "ritual_rustdoc_nightly",
    doc(cfg(any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    )))
)]
#[cfg(any(
    any(
        cpp_lib_version = "5.12.2",
        cpp_lib_version = "5.13.0",
        cpp_lib_version = "5.14.0"
    ),
    feature = "ritual_rustdoc"
))]
pub unsafe fn sub_int_qfloat16(
    lhs: ::std::os::raw::c_int,
    rhs: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::Qfloat16>>,
) -> ::std::os::raw::c_double {
    crate::__ffi::ctr_qt_core_ffi_operator_468(
        lhs,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::Qfloat16>>::cast_into(rhs).as_raw_ptr(),
    )
}
