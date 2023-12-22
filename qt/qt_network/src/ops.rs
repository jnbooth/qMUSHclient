#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! Functions that provide access to C++ operators

/// <p>Returns <code>true</code> if special address <i>lhs</i> is the same as host address <i>rhs</i>; otherwise returns <code>false</code>.</p>
///
/// Calls C++ function: <span style='color: green;'>```bool operator==(QHostAddress::SpecialAddress address1, const QHostAddress& address2)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-eq-eq">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns <code>true</code> if special address <i>lhs</i> is the same as host address <i>rhs</i>; otherwise returns <code>false</code>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qhostaddress.html#isEqual">isEqual</a>().</p></div>
#[inline(always)]
pub unsafe fn eq(
    address1: crate::q_host_address::SpecialAddress,
    address2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHostAddress>>,
) -> bool {
    crate::__ffi::ctr_qt_network_ffi_operator__8(
        address1,
        ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHostAddress>>::cast_into(address2)
            .as_raw_ptr(),
    )
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QNetworkCacheMetaData& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_network_cache_meta_data(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QNetworkCacheMetaData>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QNetworkCacheMetaData>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, QAbstractSocket::SocketError arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_abstract_socket_socket_error(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: crate::q_abstract_socket::SocketError,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__2(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1).as_raw_ptr(),
            arg2,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, QAbstractSocket::SocketState arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_abstract_socket_socket_state(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: crate::q_abstract_socket::SocketState,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__3(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1).as_raw_ptr(),
            arg2,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QSslCertificate& certificate)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_ssl_certificate(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    certificate: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSslCertificate>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__4(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSslCertificate>>::cast_into(certificate)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QSslCertificate::SubjectInfo info)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_ssl_certificate_subject_info(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    info: crate::q_ssl_certificate::SubjectInfo,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__5(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            info,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QSslError& error)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_ssl_error(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    error: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSslError>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__6(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSslError>>::cast_into(error).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QSslError::SslError& error)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_ssl_error_ssl_error(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    error: *const crate::q_ssl_error::SslError,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__7(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            error,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QHostAddress& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_host_address(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHostAddress>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__10(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHostAddress>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QHostAddress& arg2)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_host_address(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHostAddress>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__11(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHostAddress>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, QLocalSocket::LocalSocketError arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_local_socket_local_socket_error(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: crate::q_local_socket::LocalSocketError,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__17(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1).as_raw_ptr(),
            arg2,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, QLocalSocket::LocalSocketState arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_local_socket_local_socket_state(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: crate::q_local_socket::LocalSocketState,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__18(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1).as_raw_ptr(),
            arg2,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QNetworkCookie& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_network_cookie(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QNetworkCookie>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__20(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1).as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QNetworkCookie>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QNetworkInterface& networkInterface)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_network_interface(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    network_interface: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QNetworkInterface>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__21(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QNetworkInterface>>::cast_into(
                network_interface,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QNetworkProxy& proxy)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_network_proxy(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    proxy: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QNetworkProxy>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__22(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QNetworkProxy>>::cast_into(proxy)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QNetworkProxyQuery& proxyQuery)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_network_proxy_query(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    proxy_query: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QNetworkProxyQuery>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__23(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QNetworkProxyQuery>>::cast_into(
                proxy_query,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QSslCipher& cipher)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_ssl_cipher(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    cipher: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSslCipher>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__25(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSslCipher>>::cast_into(cipher)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QSslDiffieHellmanParameters& dhparams)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_ssl_diffie_hellman_parameters(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    dhparams: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSslDiffieHellmanParameters>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__26(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSslDiffieHellmanParameters>>::cast_into(
                dhparams,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, QSslEllipticCurve curve)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_ssl_elliptic_curve(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    curve: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSslEllipticCurve>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__31(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSslEllipticCurve>>::cast_into(curve)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QSslKey& key)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes host address <i>address</i> to the stream <i>out</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_ssl_key(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    key: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QSslKey>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__32(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QSslKey>>::cast_into(key).as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QPair<QByteArray, QByteArray>>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_pair_of_q_byte_array_q_byte_array(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfQByteArrayQByteArray>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__81(::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfQByteArrayQByteArray>>::cast_into(list).as_raw_ptr())
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QSslCertificate>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_ssl_certificate(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQSslCertificate>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__88(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQSslCertificate>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QSslError>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_ssl_error(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQSslError>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__89(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQSslError>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QSslCipher>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_ssl_cipher(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQSslCipher>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__90(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQSslCipher>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QHostAddress>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_host_address(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQHostAddress>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__91(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQHostAddress>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QNetworkCookie>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_network_cookie(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQNetworkCookie>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__93(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQNetworkCookie>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QNetworkInterface>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_network_interface(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQNetworkInterface>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__95(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQNetworkInterface>>::cast_into(
                list,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QNetworkProxy>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_list_of_q_network_proxy(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQNetworkProxy>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__96(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQNetworkProxy>>::cast_into(list)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QSslError>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_vector_of_q_ssl_error(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQSslError>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__98(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQSslError>>::cast_into(vec)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QVector<QSslEllipticCurve>& vec)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_vector_of_q_ssl_elliptic_curve(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    vec: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QVectorOfQSslEllipticCurve>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__99(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QVectorOfQSslEllipticCurve>>::cast_into(
                vec,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QMap<QByteArray, QVariant>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_map_of_q_byte_array_q_variant(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfQByteArrayQVariant>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__101(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfQByteArrayQVariant>>::cast_into(map)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QHash<QNetworkRequest::Attribute, QVariant>& hash)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_hash_of_attribute_q_variant(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    hash: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHashOfAttributeQVariant>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__102(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHashOfAttributeQVariant>>::cast_into(
                hash,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<QByteArray, QByteArray>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_pair_of_q_byte_array_q_byte_array(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfQByteArrayQByteArray>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__103(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfQByteArrayQByteArray>>::cast_into(
                pair,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QPair<QHostAddress, int>& pair)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_q_pair_of_q_host_address_int(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    pair: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfQHostAddressInt>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__104(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfQHostAddressInt>>::cast_into(pair)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QAbstractSocket::PauseMode>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_abstract_socket_pause_mode(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_abstract_socket::PauseMode>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__105(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QAbstractSocket::BindFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_abstract_socket_bind_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_abstract_socket::BindFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__106(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QHostAddress::ConversionModeFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_host_address_conversion_mode_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_host_address::ConversionModeFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__107(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QHstsPolicy::PolicyFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_hsts_policy_policy_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_hsts_policy::PolicyFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__108(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QLocalServer::SocketOption>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_local_server_socket_option(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_local_server::SocketOption>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__109(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QNetworkConfiguration::StateFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_network_configuration_state_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_network_configuration::StateFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__110(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QNetworkConfigurationManager::Capability>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_network_configuration_manager_capability(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_network_configuration_manager::Capability>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__111(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QNetworkInterface::InterfaceFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_network_interface_interface_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_network_interface::InterfaceFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__112(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QNetworkProxy::Capability>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_network_proxy_capability(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_network_proxy::Capability>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__113(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QNetworkSession::UsagePolicy>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_debug_qt_core_q_flags_q_network_session_usage_policy(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_network_session::UsagePolicy>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__114(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QAbstractSocket::PauseMode> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_abstract_socket_pause_mode(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_abstract_socket::PauseMode>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__115(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QAbstractSocket::BindFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_abstract_socket_bind_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_abstract_socket::BindFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__116(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QHostAddress::ConversionModeFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_host_address_conversion_mode_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_host_address::ConversionModeFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__117(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QHstsPolicy::PolicyFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_hsts_policy_policy_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_hsts_policy::PolicyFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__118(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QLocalServer::SocketOption> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_local_server_socket_option(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_local_server::SocketOption>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__119(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QNetworkConfiguration::StateFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_network_configuration_state_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_network_configuration::StateFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__120(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QNetworkConfigurationManager::Capability> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_network_configuration_manager_capability(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_network_configuration_manager::Capability>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__121(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QNetworkInterface::InterfaceFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_network_interface_interface_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_network_interface::InterfaceFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__122(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QNetworkProxy::Capability> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_network_proxy_capability(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_network_proxy::Capability>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__123(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QNetworkSession::UsagePolicy> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_qt_core_q_flags_q_network_session_usage_policy(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_network_session::UsagePolicy>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__124(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QPair<QByteArray, QByteArray>>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_pair_of_q_byte_array_q_byte_array(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfQByteArrayQByteArray>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__151(::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s).as_raw_ptr() as *mut ::qt_core::QDataStream, ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfQByteArrayQByteArray>>::cast_into(l).as_raw_ptr())
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QList<QHostAddress>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_list_of_q_host_address(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQHostAddress>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__161(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQHostAddress>>::cast_into(l)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QHash<QNetworkRequest::Attribute, QVariant>& hash)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_hash_of_attribute_q_variant(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    hash: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHashOfAttributeQVariant>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__176(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHashOfAttributeQVariant>>::cast_into(
                hash,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QMap<QByteArray, QVariant>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_map_of_q_byte_array_q_variant(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfQByteArrayQVariant>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__178(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfQByteArrayQVariant>>::cast_into(map)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<QByteArray, QByteArray>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_pair_of_q_byte_array_q_byte_array(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfQByteArrayQByteArray>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__181(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfQByteArrayQByteArray>>::cast_into(
                p,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QPair<QHostAddress, int>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_qt_core_q_data_stream_q_pair_of_q_host_address_int(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfQHostAddressInt>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__182(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfQHostAddressInt>>::cast_into(p)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a host address into <i>address</i> from the stream <i>in</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QNetworkCacheMetaData& arg2)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QHostAddress &address)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a host address into <i>address</i> from the stream <i>in</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_network_cache_meta_data(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QNetworkCacheMetaData>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__1(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QNetworkCacheMetaData>>::cast_into(arg2)
                .as_raw_ptr() as *mut crate::QNetworkCacheMetaData,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a host address into <i>address</i> from the stream <i>in</i> and returns a reference to the stream.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QHostAddress& arg2)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qhostaddress.html#operator-gt-gt">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a host address into <i>address</i> from the stream <i>in</i> and returns a reference to the stream.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Serializing Qt Data Types</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_host_address(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHostAddress>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__12(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(arg1)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHostAddress>>::cast_into(arg2)
                .as_raw_ptr() as *mut crate::QHostAddress,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QPair<QByteArray, QByteArray>>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_q_pair_of_q_byte_array_q_byte_array(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQPairOfQByteArrayQByteArray>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__135(::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s).as_raw_ptr() as *mut ::qt_core::QDataStream, ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQPairOfQByteArrayQByteArray>>::cast_into(l).as_raw_ptr() as *mut crate::QListOfQPairOfQByteArrayQByteArray)
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QList<QHostAddress>& l)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_list_of_q_host_address(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQHostAddress>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__145(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQHostAddress>>::cast_into(l)
                .as_raw_ptr() as *mut crate::QListOfQHostAddress,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QHash<QNetworkRequest::Attribute, QVariant>& hash)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_hash_of_attribute_q_variant(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    hash: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QHashOfAttributeQVariant>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__175(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QHashOfAttributeQVariant>>::cast_into(hash)
                .as_raw_ptr() as *mut crate::QHashOfAttributeQVariant,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QMap<QByteArray, QVariant>& map)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_map_of_q_byte_array_q_variant(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    map: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QMapOfQByteArrayQVariant>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__177(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QMapOfQByteArrayQVariant>>::cast_into(map)
                .as_raw_ptr() as *mut crate::QMapOfQByteArrayQVariant,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QPair<QByteArray, QByteArray>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pair_of_q_byte_array_q_byte_array(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfQByteArrayQByteArray>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__179(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfQByteArrayQByteArray>>::cast_into(p)
                .as_raw_ptr() as *mut crate::QPairOfQByteArrayQByteArray,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QPair<QHostAddress, int>& p)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-gt-gt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator>>(QDataStream &in, QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Reads a bit array into <i>ba</i> from stream <i>in</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shr_q_data_stream_q_pair_of_q_host_address_int(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QPairOfQHostAddressInt>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_operator__180(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QPairOfQHostAddressInt>>::cast_into(p)
                .as_raw_ptr() as *mut crate::QPairOfQHostAddressInt,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}
