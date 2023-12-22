#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! C++ namespace: <span style='color: green;'>```QPasswordDigestor```</span>

/// <p>Returns a hash computed using the PBKDF1-algorithm as defined in <a href="https://tools.ietf.org/html/rfc8018#section-5.1">RFC 8018</a>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QByteArray QPasswordDigestor::deriveKeyPbkdf1(QCryptographicHash::Algorithm algorithm, const QByteArray& password, const QByteArray& salt, int iterations, quint64 dkLen)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qpassworddigestor.html#deriveKeyPbkdf1">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a hash computed using the PBKDF1-algorithm as defined in <a href="https://tools.ietf.org/html/rfc8018#section-5.1">RFC 8018</a>.</p>
/// <p>The function takes the <i>data</i> and <i>salt</i>, and then hashes it repeatedly for <i>iterations</i> iterations using the specified hash <i>algorithm</i>. If the resulting hash is longer than <i>dkLen</i> then it is truncated before it is returned.</p>
/// <p>This function only supports SHA-1 and MD5! The max output size is 160 bits (20 bytes) when using SHA-1, or 128 bits (16 bytes) when using MD5. Specifying a value for <i>dkLen</i> which is greater than this results in a warning and an empty <a href="http://doc.qt.io/qt-5/qbytearray.html">QByteArray</a> is returned. To programmatically check this limit you can use <a href="http://doc.qt.io/qt-5/qcryptographichash.html#hashLength">QCryptographicHash::hashLength</a>. Furthermore: the <i>salt</i> must always be 8 bytes long!</p>
/// <p><b>Note: </b>This function is provided for use with legacy applications and all new applications are recommended to use <a href="http://doc.qt.io/qt-5/qpassworddigestor.html#deriveKeyPbkdf2">PBKDF2</a>.</p>
/// <p>This function was introduced in Qt 5.12.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qpassworddigestor.html#deriveKeyPbkdf2">deriveKeyPbkdf2</a>, <a href="http://doc.qt.io/qt-5/qcryptographichash.html">QCryptographicHash</a>, and <a href="http://doc.qt.io/qt-5/qcryptographichash.html#hashLength">QCryptographicHash::hashLength</a>.</p></div>
#[inline(always)]
pub unsafe fn derive_key_pbkdf1(
    algorithm: ::qt_core::q_cryptographic_hash::Algorithm,
    password: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QByteArray>>,
    salt: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QByteArray>>,
    iterations: ::std::os::raw::c_int,
    dk_len: u64,
) -> ::cpp_core::CppBox<::qt_core::QByteArray> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_QPasswordDigestor_deriveKeyPbkdf1(
            algorithm,
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QByteArray>>::cast_into(password)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QByteArray>>::cast_into(salt)
                .as_raw_ptr(),
            iterations,
            dk_len,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Derive a key using the PBKDF2-algorithm as defined in <a href="https://tools.ietf.org/html/rfc8018#section-5.2">RFC 8018</a>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QByteArray QPasswordDigestor::deriveKeyPbkdf2(QCryptographicHash::Algorithm algorithm, const QByteArray& password, const QByteArray& salt, int iterations, quint64 dkLen)```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qpassworddigestor.html#deriveKeyPbkdf2">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Derive a key using the PBKDF2-algorithm as defined in <a href="https://tools.ietf.org/html/rfc8018#section-5.2">RFC 8018</a>.</p>
/// <p>This function takes the <i>data</i> and <i>salt</i>, and then applies HMAC-X, where the X is <i>algorithm</i>, repeatedly. It internally concatenates intermediate results to the final output until at least <i>dkLen</i> amount of bytes have been computed and it will execute HMAC-X <i>iterations</i> times each time a concatenation is required. The total number of times it will execute HMAC-X depends on <i>iterations</i>, <i>dkLen</i> and <i>algorithm</i> and can be calculated as <code>iterations * ceil(dkLen / QCryptographicHash::hashLength(algorithm))</code>.</p>
/// <p>This function was introduced in Qt 5.12.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qpassworddigestor.html#deriveKeyPbkdf1">deriveKeyPbkdf1</a>, <a href="http://doc.qt.io/qt-5/qmessageauthenticationcode.html">QMessageAuthenticationCode</a>, and <a href="http://doc.qt.io/qt-5/qcryptographichash.html">QCryptographicHash</a>.</p></div>
#[inline(always)]
pub unsafe fn derive_key_pbkdf2(
    algorithm: ::qt_core::q_cryptographic_hash::Algorithm,
    password: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QByteArray>>,
    salt: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QByteArray>>,
    iterations: ::std::os::raw::c_int,
    dk_len: u64,
) -> ::cpp_core::CppBox<::qt_core::QByteArray> {
    let ffi_result = {
        crate::__ffi::ctr_qt_network_ffi_QPasswordDigestor_deriveKeyPbkdf2(
            algorithm,
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QByteArray>>::cast_into(password)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QByteArray>>::cast_into(salt)
                .as_raw_ptr(),
            iterations,
            dk_len,
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}
