use std::io;

use qt_network as q;

use crate::io::{QIOError, QIO};

impl QIO for q::QAbstractSocket {
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

qio_binding!(QAbstractSocket);

impl QIO for q::QSslSocket {
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

qio_binding!(QSslSocket);

impl QIO for q::QTcpSocket {
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

qio_binding!(QTcpSocket);

impl QIO for q::QUdpSocket {
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

qio_binding!(QUdpSocket);

impl QIO for q::QLocalSocket {
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

qio_binding!(QLocalSocket);

impl QIO for q::QNetworkReply {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        Ok(())
    }
}

qio_binding!(QNetworkReply);
