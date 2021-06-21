#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! C++ namespace: <span style='color: green;'>```QSsl```</span>

/// <p>Describes the two types of keys <a href="http://doc.qt.io/qt-5/qsslkey.html">QSslKey</a> supports.</p>
///
/// C++ enum: <span style='color: green;'>```QSsl::KeyType```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qssl.html#KeyType-enum">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Describes the two types of keys <a href="http://doc.qt.io/qt-5/qsslkey.html">QSslKey</a> supports.</p></div>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyType(::std::os::raw::c_int);

impl From<::std::os::raw::c_int> for KeyType {
    fn from(value: ::std::os::raw::c_int) -> Self {
        KeyType(value)
    }
}

impl From<KeyType> for ::std::os::raw::c_int {
    fn from(value: KeyType) -> Self {
        value.0
    }
}

impl KeyType {
    pub fn to_int(&self) -> ::std::os::raw::c_int {
        self.0
    }
}

impl KeyType {
    /// A private key. (C++ enum variant: <span style='color: green;'>```PrivateKey = 0```</span>)
    #[allow(non_upper_case_globals)]
    pub const PrivateKey: crate::q_ssl::KeyType = crate::q_ssl::KeyType(0);
    /// A public key. (C++ enum variant: <span style='color: green;'>```PublicKey = 1```</span>)
    #[allow(non_upper_case_globals)]
    pub const PublicKey: crate::q_ssl::KeyType = crate::q_ssl::KeyType(1);
}

/// <p>Describes supported encoding formats for certificates and keys.</p>
///
/// C++ enum: <span style='color: green;'>```QSsl::EncodingFormat```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qssl.html#EncodingFormat-enum">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Describes supported encoding formats for certificates and keys.</p></div>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EncodingFormat(::std::os::raw::c_int);

impl From<::std::os::raw::c_int> for EncodingFormat {
    fn from(value: ::std::os::raw::c_int) -> Self {
        EncodingFormat(value)
    }
}

impl From<EncodingFormat> for ::std::os::raw::c_int {
    fn from(value: EncodingFormat) -> Self {
        value.0
    }
}

impl EncodingFormat {
    pub fn to_int(&self) -> ::std::os::raw::c_int {
        self.0
    }
}

impl EncodingFormat {
    /// The PEM format. (C++ enum variant: <span style='color: green;'>```Pem = 0```</span>)
    #[allow(non_upper_case_globals)]
    pub const Pem: crate::q_ssl::EncodingFormat = crate::q_ssl::EncodingFormat(0);
    /// The DER format. (C++ enum variant: <span style='color: green;'>```Der = 1```</span>)
    #[allow(non_upper_case_globals)]
    pub const Der: crate::q_ssl::EncodingFormat = crate::q_ssl::EncodingFormat(1);
}

/// <p>Describes the different key algorithms supported by <a href="http://doc.qt.io/qt-5/qsslkey.html">QSslKey</a>.</p>
///
/// C++ enum: <span style='color: green;'>```QSsl::KeyAlgorithm```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qssl.html#KeyAlgorithm-enum">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Describes the different key algorithms supported by <a href="http://doc.qt.io/qt-5/qsslkey.html">QSslKey</a>.</p>
///
/// <p>The opaque key facility allows applications to add support for facilities such as PKCS#11 that Qt does not currently offer natively.</p></div>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyAlgorithm(::std::os::raw::c_int);

impl From<::std::os::raw::c_int> for KeyAlgorithm {
    fn from(value: ::std::os::raw::c_int) -> Self {
        KeyAlgorithm(value)
    }
}

impl From<KeyAlgorithm> for ::std::os::raw::c_int {
    fn from(value: KeyAlgorithm) -> Self {
        value.0
    }
}

impl KeyAlgorithm {
    pub fn to_int(&self) -> ::std::os::raw::c_int {
        self.0
    }
}

impl KeyAlgorithm {
    /// A key that should be treated as a 'black box' by <a href="http://doc.qt.io/qt-5/qsslkey.html">QSslKey</a>. (C++ enum variant: <span style='color: green;'>```Opaque = 0```</span>)
    #[allow(non_upper_case_globals)]
    pub const Opaque: crate::q_ssl::KeyAlgorithm = crate::q_ssl::KeyAlgorithm(0);
    /// The RSA algorithm. (C++ enum variant: <span style='color: green;'>```Rsa = 1```</span>)
    #[allow(non_upper_case_globals)]
    pub const Rsa: crate::q_ssl::KeyAlgorithm = crate::q_ssl::KeyAlgorithm(1);
    /// The DSA algorithm. (C++ enum variant: <span style='color: green;'>```Dsa = 2```</span>)
    #[allow(non_upper_case_globals)]
    pub const Dsa: crate::q_ssl::KeyAlgorithm = crate::q_ssl::KeyAlgorithm(2);
    /// The Elliptic Curve algorithm. (C++ enum variant: <span style='color: green;'>```Ec = 3```</span>)
    #[allow(non_upper_case_globals)]
    pub const Ec: crate::q_ssl::KeyAlgorithm = crate::q_ssl::KeyAlgorithm(3);
    /// The Diffie-Hellman algorithm. (C++ enum variant: <span style='color: green;'>```Dh = 4```</span>)
    #[allow(non_upper_case_globals)]
    pub const Dh: crate::q_ssl::KeyAlgorithm = crate::q_ssl::KeyAlgorithm(4);
}

/// <p>Describes the key types for alternative name entries in <a href="http://doc.qt.io/qt-5/qsslcertificate.html">QSslCertificate</a>.</p>
///
/// C++ enum: <span style='color: green;'>```QSsl::AlternativeNameEntryType```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qssl.html#AlternativeNameEntryType-enum">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Describes the key types for alternative name entries in <a href="http://doc.qt.io/qt-5/qsslcertificate.html">QSslCertificate</a>.</p>
///
/// <p><b>Note: </b>In Qt 4, this enum was called <code>AlternateNameEntryType</code>. That name is deprecated in Qt 5.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qsslcertificate.html#subjectAlternativeNames">QSslCertificate::subjectAlternativeNames</a>().</p></div>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AlternativeNameEntryType(::std::os::raw::c_int);

impl From<::std::os::raw::c_int> for AlternativeNameEntryType {
    fn from(value: ::std::os::raw::c_int) -> Self {
        AlternativeNameEntryType(value)
    }
}

impl From<AlternativeNameEntryType> for ::std::os::raw::c_int {
    fn from(value: AlternativeNameEntryType) -> Self {
        value.0
    }
}

impl AlternativeNameEntryType {
    pub fn to_int(&self) -> ::std::os::raw::c_int {
        self.0
    }
}

impl AlternativeNameEntryType {
    /// An email entry; the entry contains an email address that the certificate is valid for. (C++ enum variant: <span style='color: green;'>```EmailEntry = 0```</span>)
    #[allow(non_upper_case_globals)]
    pub const EmailEntry: crate::q_ssl::AlternativeNameEntryType =
        crate::q_ssl::AlternativeNameEntryType(0);
    /// A DNS host name entry; the entry contains a host name entry that the certificate is valid for. The entry may contain wildcards. (C++ enum variant: <span style='color: green;'>```DnsEntry = 1```</span>)
    #[allow(non_upper_case_globals)]
    pub const DnsEntry: crate::q_ssl::AlternativeNameEntryType =
        crate::q_ssl::AlternativeNameEntryType(1);
    /// An IP address entry; the entry contains an IP address entry that the certificate is valid for, introduced in Qt 5.13. (C++ enum variant: <span style='color: green;'>```IpAddressEntry = 2```</span>)
    #[allow(non_upper_case_globals)]
    pub const IpAddressEntry: crate::q_ssl::AlternativeNameEntryType =
        crate::q_ssl::AlternativeNameEntryType(2);
}

/// <p>Describes the protocol of the cipher.</p>
///
/// C++ enum: <span style='color: green;'>```QSsl::SslProtocol```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qssl.html#SslProtocol-enum">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Describes the protocol of the cipher.</p></div>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SslProtocol(::std::os::raw::c_int);

impl From<::std::os::raw::c_int> for SslProtocol {
    fn from(value: ::std::os::raw::c_int) -> Self {
        SslProtocol(value)
    }
}

impl From<SslProtocol> for ::std::os::raw::c_int {
    fn from(value: SslProtocol) -> Self {
        value.0
    }
}

impl SslProtocol {
    pub fn to_int(&self) -> ::std::os::raw::c_int {
        self.0
    }
}

impl SslProtocol {
    /// SSLv3; not supported by <a href="http://doc.qt.io/qt-5/qsslsocket.html">QSslSocket</a>. (C++ enum variant: <span style='color: green;'>```SslV3 = 0```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslV3: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(0);
    /// SSLv2; not supported by <a href="http://doc.qt.io/qt-5/qsslsocket.html">QSslSocket</a>. (C++ enum variant: <span style='color: green;'>```SslV2 = 1```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslV2: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(1);
    /// TLSv1.0 (C++ enum variant: <span style='color: green;'>```TlsV1_0 = 2```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV10: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(2);
    /// TLSv1.1. When using the WinRT backend this option will also enable TLSv1.0. (C++ enum variant: <span style='color: green;'>```TlsV1_1 = 3```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV11: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(3);
    /// TLSv1.2. When using the WinRT backend this option will also enable TLSv1.0 and TLSv1.1. (C++ enum variant: <span style='color: green;'>```TlsV1_2 = 4```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV12: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(4);
    /// Any supported protocol. This value is used by <a href="http://doc.qt.io/qt-5/qsslsocket.html">QSslSocket</a> only. (C++ enum variant: <span style='color: green;'>```AnyProtocol = 5```</span>)
    #[allow(non_upper_case_globals)]
    pub const AnyProtocol: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(5);
    /// Same as TlsV1_0. This enumerator is deprecated, use TlsV1_0 instead. (C++ enum variant: <span style='color: green;'>```TlsV1SslV3 = 6```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV1SslV3: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(6);
    /// The default option, using protocols known to be secure. (C++ enum variant: <span style='color: green;'>```SecureProtocols = 7```</span>)
    #[allow(non_upper_case_globals)]
    pub const SecureProtocols: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(7);
    /// TLSv1.0 and later versions. This option is not available when using the WinRT backend due to platform limitations. (C++ enum variant: <span style='color: green;'>```TlsV1_0OrLater = 8```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV10OrLater: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(8);
    /// TLSv1.1 and later versions. This option is not available when using the WinRT backend due to platform limitations. (C++ enum variant: <span style='color: green;'>```TlsV1_1OrLater = 9```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV11OrLater: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(9);
    /// TLSv1.2 and later versions. This option is not available when using the WinRT backend due to platform limitations. (C++ enum variant: <span style='color: green;'>```TlsV1_2OrLater = 10```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV12OrLater: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(10);
    /// DTLSv1.0 (C++ enum variant: <span style='color: green;'>```DtlsV1_0 = 11```</span>)
    #[allow(non_upper_case_globals)]
    pub const DtlsV10: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(11);
    /// DTLSv1.0 and later versions. (C++ enum variant: <span style='color: green;'>```DtlsV1_0OrLater = 12```</span>)
    #[allow(non_upper_case_globals)]
    pub const DtlsV10OrLater: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(12);
    /// DTLSv1.2 (C++ enum variant: <span style='color: green;'>```DtlsV1_2 = 13```</span>)
    #[allow(non_upper_case_globals)]
    pub const DtlsV12: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(13);
    /// DTLSv1.2 and later versions. (C++ enum variant: <span style='color: green;'>```DtlsV1_2OrLater = 14```</span>)
    #[allow(non_upper_case_globals)]
    pub const DtlsV12OrLater: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(14);
    /// TLSv1.3. (Since Qt 5.12) (C++ enum variant: <span style='color: green;'>```TlsV1_3 = 15```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV13: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(15);
    /// TLSv1.3 and later versions. (Since Qt 5.12) (C++ enum variant: <span style='color: green;'>```TlsV1_3OrLater = 16```</span>)
    #[allow(non_upper_case_globals)]
    pub const TlsV13OrLater: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(16);
    /// The cipher's protocol cannot be determined. (C++ enum variant: <span style='color: green;'>```UnknownProtocol = -1```</span>)
    #[allow(non_upper_case_globals)]
    pub const UnknownProtocol: crate::q_ssl::SslProtocol = crate::q_ssl::SslProtocol(-1);
}

/// <p>Describes the options that can be used to control the details of SSL behaviour. These options are generally used to turn features off to work around buggy servers.</p>
///
/// C++ enum: <span style='color: green;'>```QSsl::SslOption```</span>.
///
/// <a href="http://doc.qt.io/qt-5/qssl.html#SslOption-enum">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Describes the options that can be used to control the details of SSL behaviour. These options are generally used to turn features off to work around buggy servers.</p>
///
/// <p>By default, SslOptionDisableEmptyFragments is turned on since this causes problems with a large number of servers. SslOptionDisableLegacyRenegotiation is also turned on, since it introduces a security risk. SslOptionDisableCompression is turned on to prevent the attack publicised by CRIME. SslOptionDisableSessionPersistence is turned on to optimize memory usage. The other options are turned off.</p>
/// <p><b>Note: </b>Availability of above options depends on the version of the SSL backend in use.</p>
/// <p>The SslOptions type is a typedef for <a href="http://doc.qt.io/qt-5/qflags.html">QFlags</a>&lt;SslOption&gt;. It stores an OR combination of SslOption values.</p></div>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SslOption(::std::os::raw::c_int);

impl From<::std::os::raw::c_int> for SslOption {
    fn from(value: ::std::os::raw::c_int) -> Self {
        SslOption(value)
    }
}

impl From<SslOption> for ::std::os::raw::c_int {
    fn from(value: SslOption) -> Self {
        value.0
    }
}

impl SslOption {
    pub fn to_int(&self) -> ::std::os::raw::c_int {
        self.0
    }
}

impl SslOption {
    /// Disables the insertion of empty fragments into the data when using block ciphers. When enabled, this prevents some attacks (such as the BEAST attack), however it is incompatible with some servers. (C++ enum variant: <span style='color: green;'>```SslOptionDisableEmptyFragments = 1```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslOptionDisableEmptyFragments: crate::q_ssl::SslOption = crate::q_ssl::SslOption(1);
    /// Disables the SSL session ticket extension. This can cause slower connection setup, however some servers are not compatible with the extension. (C++ enum variant: <span style='color: green;'>```SslOptionDisableSessionTickets = 2```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslOptionDisableSessionTickets: crate::q_ssl::SslOption = crate::q_ssl::SslOption(2);
    /// Disables the SSL compression extension. When enabled, this allows the data being passed over SSL to be compressed, however some servers are not compatible with this extension. (C++ enum variant: <span style='color: green;'>```SslOptionDisableCompression = 4```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslOptionDisableCompression: crate::q_ssl::SslOption = crate::q_ssl::SslOption(4);
    /// Disables the SSL server name indication extension. When enabled, this tells the server the virtual host being accessed allowing it to respond with the correct certificate. (C++ enum variant: <span style='color: green;'>```SslOptionDisableServerNameIndication = 8```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslOptionDisableServerNameIndication: crate::q_ssl::SslOption =
        crate::q_ssl::SslOption(8);
    /// Disables the older insecure mechanism for renegotiating the connection parameters. When enabled, this option can allow connections for legacy servers, but it introduces the possibility that an attacker could inject plaintext into the SSL session. (C++ enum variant: <span style='color: green;'>```SslOptionDisableLegacyRenegotiation = 16```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslOptionDisableLegacyRenegotiation: crate::q_ssl::SslOption =
        crate::q_ssl::SslOption(16);
    /// Disables SSL session sharing via the session ID handshake attribute. (C++ enum variant: <span style='color: green;'>```SslOptionDisableSessionSharing = 32```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslOptionDisableSessionSharing: crate::q_ssl::SslOption = crate::q_ssl::SslOption(32);
    /// Disables storing the SSL session in ASN.1 format as returned by <a href="http://doc.qt.io/qt-5/qsslconfiguration.html#sessionTicket">QSslConfiguration::sessionTicket</a>(). Enabling this feature adds memory overhead of approximately 1K per used session ticket. (C++ enum variant: <span style='color: green;'>```SslOptionDisableSessionPersistence = 64```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslOptionDisableSessionPersistence: crate::q_ssl::SslOption =
        crate::q_ssl::SslOption(64);
    /// Disables selecting the cipher chosen based on the servers preferences rather than the order ciphers were sent by the client. This option is only relevant to server sockets, and is only honored by the OpenSSL backend. (C++ enum variant: <span style='color: green;'>```SslOptionDisableServerCipherPreference = 128```</span>)
    #[allow(non_upper_case_globals)]
    pub const SslOptionDisableServerCipherPreference: crate::q_ssl::SslOption =
        crate::q_ssl::SslOption(128);
}
