use std::io;

use qt_core::q_file_device::FileError;
use qt_core::q_process::ProcessError;
use qt_network::q_abstract_socket::SocketError;
use qt_network::q_local_socket::LocalSocketError;
use qt_network::q_network_reply::NetworkError;

pub trait QIOError {
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
