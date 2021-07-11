use std::fmt::{self, Debug, Formatter};
use std::io::{self, Read, Write};
use std::ops::Drop;
use std::os::raw::c_char;
use std::rc::Rc;

use cpp_core::{CppDeletable, StaticUpcast};
use qt_core::q_file_device::FileError;
use qt_core::q_process::ProcessError;
use qt_core::{
    QBox, QBuffer, QFile, QFileDevice, QIODevice, QObject, QProcess, QPtr, QSaveFile, QString,
};
use qt_network::q_abstract_socket::SocketError;
use qt_network::q_local_socket::LocalSocketError;
use qt_network::q_network_reply::NetworkError;
use qt_network::{QAbstractSocket, QLocalSocket, QNetworkReply, QSslSocket, QTcpSocket, QUdpSocket};

pub struct RIODevice<Q: QIO> {
    dropper: Rc<QBox<Q>>,
    inner: QPtr<Q>,
    device: QPtr<QIODevice>,
}

// Manually implemented in order to avoid a Q: Clone bound.
impl<Q: QIO> Clone for RIODevice<Q> {
    fn clone(&self) -> Self {
        Self {
            dropper: self.dropper.clone(),
            inner: self.inner.clone(),
            device: self.device.clone(),
        }
    }
}

impl<Q: QIO> Debug for RIODevice<Q> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RIODevice")
            .field("dropper", &self.dropper)
            .field("inner", &self.inner)
            .field("device", &self.device)
            .finish()
    }
}

impl<Q: QIO> Drop for RIODevice<Q> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.dropper) == 1 {
            self.close(); // this is the last one
        }
    }
}

impl<Q: QIO> RIODevice<Q> {
    pub fn new(inner: QBox<Q>) -> Self {
        unsafe {
            Self {
                device: inner.static_upcast(),
                inner: inner.static_upcast(),
                dropper: Rc::new(inner),
            }
        }
    }
    pub fn as_ptr(&self) -> &QPtr<Q> {
        &self.inner
    }
    pub fn readable(&self) -> bool {
        unsafe { self.device.is_readable() }
    }
    pub fn writable(&self) -> bool {
        unsafe { self.device.is_writable() }
    }

    #[inline]
    fn qtry(&self, res: i64) -> io::Result<usize> {
        if res < 0 {
            Err(unsafe {
                io::Error::new(
                    self.inner.io_error(),
                    self.device.error_string().to_std_string(),
                )
            })
        } else {
            Ok(res as usize)
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.qtry(unsafe {
            self.device
                .read_2a(buf.as_mut_ptr() as *mut c_char, buf.len() as i64)
        })
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.qtry(unsafe {
            self.device
                .write_char_i64(buf.as_ptr() as *const c_char, buf.len() as i64)
        })
    }

    pub fn flush(&self) -> io::Result<()> {
        unsafe { self.inner.io_flush() }
    }

    pub fn close(&self) {
        unsafe {
            if !self.device.is_null() {
                self.device.close();
            }
        }
    }
    #[inline]
    pub fn io(&self) -> RIODeviceMut<Q> {
        RIODeviceMut(self)
    }

    pub fn write_fmt(&self, args: fmt::Arguments<'_>) -> Result<(), io::Error> {
        RIODeviceMut(self).write_fmt(args)
    }
}

impl<Q: QIO> Read for RIODevice<Q> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        RIODevice::read(self, buf)
    }
}

impl<Q: QIO> Write for RIODevice<Q> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        RIODevice::write(self, buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        RIODevice::flush(self)
    }
}

pub struct RIODeviceMut<'a, Q: QIO>(&'a RIODevice<Q>);

impl<Q: QIO> Read for RIODeviceMut<'_, Q> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<Q: QIO> Write for RIODeviceMut<'_, Q> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<Q: QIO + StaticUpcast<QAbstractSocket>> RIODevice<Q> {
    pub fn connect(&self, address: &str, port: u16) {
        unsafe {
            self.inner
                .static_upcast::<QAbstractSocket>()
                .connect_to_host_q_string_u16(&QString::from_std_str(address), port);
        }
    }
}

trait QIOError {
    fn to_io_error(self) -> io::ErrorKind;
}

impl QIOError for SocketError {
    fn to_io_error(self) -> io::ErrorKind {
        use io::ErrorKind::*;
        match self {
            Self::ConnectionRefusedError => ConnectionRefused,
            Self::RemoteHostClosedError => ConnectionAborted,
            Self::HostNotFoundError => NotFound,
            Self::SocketAccessError => PermissionDenied,
            Self::SocketTimeoutError => TimedOut,
            Self::DatagramTooLargeError => InvalidData,
            Self::NetworkError => BrokenPipe,
            Self::AddressInUseError => AddrInUse,
            Self::ProxyConnectionRefusedError => ConnectionRefused,
            Self::ProxyConnectionClosedError => ConnectionAborted,
            Self::ProxyConnectionTimeoutError => TimedOut,
            Self::ProxyNotFoundError => NotFound,
            Self::ProxyProtocolError => InvalidData,
            Self::TemporaryError => WouldBlock,
            _ => Other,
        }
    }
}

impl QIOError for FileError {
    fn to_io_error(self) -> io::ErrorKind {
        use io::ErrorKind::*;
        match self {
            Self::AbortError => Interrupted,
            Self::TimeOutError => TimedOut,
            Self::PermissionsError => PermissionDenied,
            _ => Other,
        }
    }
}

impl QIOError for LocalSocketError {
    fn to_io_error(self) -> io::ErrorKind {
        use io::ErrorKind::*;
        match self {
            Self::ConnectionRefusedError => ConnectionRefused,
            Self::PeerClosedError => ConnectionAborted,
            Self::ServerNotFoundError => NotFound,
            Self::SocketAccessError => PermissionDenied,
            Self::SocketTimeoutError => TimedOut,
            Self::DatagramTooLargeError => InvalidData,
            Self::ConnectionError => BrokenPipe,
            _ => Other,
        }
    }
}

impl QIOError for NetworkError {
    fn to_io_error(self) -> io::ErrorKind {
        use io::ErrorKind::*;
        match self {
            Self::ConnectionRefusedError => ConnectionRefused,
            Self::RemoteHostClosedError => ConnectionAborted,
            Self::HostNotFoundError => NotFound,
            Self::TimeoutError => TimedOut,
            Self::OperationCanceledError => Interrupted,
            Self::TemporaryNetworkFailureError => Interrupted,
            Self::NetworkSessionFailedError => ConnectionAborted,
            Self::ProxyConnectionRefusedError => ConnectionRefused,
            Self::ProxyConnectionClosedError => ConnectionAborted,
            Self::ProxyNotFoundError => NotFound,
            Self::ProxyTimeoutError => TimedOut,
            Self::ProtocolFailure => InvalidData,
            _ => Other,
        }
    }
}

impl QIOError for ProcessError {
    fn to_io_error(self) -> io::ErrorKind {
        use io::ErrorKind::*;

        match self {
            Self::FailedToStart => PermissionDenied,
            Self::Crashed => Interrupted,
            Self::Timedout => TimedOut,
            _ => Other,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub trait QIO: CppDeletable + StaticUpcast<QIODevice> + StaticUpcast<QObject> {
    unsafe fn io_error(&self) -> io::ErrorKind;
    unsafe fn io_flush(&self) -> io::Result<()>;
}

impl QIO for QBuffer {
    unsafe fn io_error(&self) -> io::ErrorKind {
        io::ErrorKind::Other
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl QIO for QAbstractSocket {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        unsafe {
            self.flush();
        }
        Ok(())
    }
}
impl QIO for QSslSocket {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        unsafe {
            self.flush();
        }
        Ok(())
    }
}
impl QIO for QTcpSocket {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        unsafe {
            self.flush();
        }
        Ok(())
    }
}
impl QIO for QUdpSocket {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        unsafe {
            self.flush();
        }
        Ok(())
    }
}
impl QIO for QLocalSocket {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        unsafe {
            self.flush();
        }
        Ok(())
    }
}

impl QIO for QFileDevice {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        unsafe {
            self.flush();
        }
        Ok(())
    }
}
impl QIO for QFile {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        unsafe {
            self.flush();
        }
        Ok(())
    }
}
impl QIO for QSaveFile {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        unsafe {
            self.flush();
        }
        Ok(())
    }
}

impl QIO for QNetworkReply {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl QIO for QProcess {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        Ok(())
    }
}
