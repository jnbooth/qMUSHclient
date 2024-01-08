use std::io;

use qt_core as q;

use crate::io::{QIOError, QIO};

impl QIO for q::QBuffer {
    unsafe fn io_error(&self) -> io::ErrorKind {
        io::ErrorKind::Other
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        Ok(())
    }
}

qio_binding!(QBuffer);

impl QIO for q::QFileDevice {
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

qio_binding!(QFileDevice);

impl QIO for q::QFile {
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

qio_binding!(QFile);

impl QIO for q::QSaveFile {
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

qio_binding!(QSaveFile);

impl QIO for q::QProcess {
    unsafe fn io_error(&self) -> io::ErrorKind {
        unsafe { self.error2().to_io_error() }
    }

    unsafe fn io_flush(&self) -> io::Result<()> {
        Ok(())
    }
}

qio_binding!(QProcess);
