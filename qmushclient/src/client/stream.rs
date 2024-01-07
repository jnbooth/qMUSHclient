use std::io::{self, BufReader, Cursor, IoSliceMut, Read};
use std::mem;

use qt::QIODevice;
use qt_network::QTcpSocket;

use crate::constants::config;

type Decompress<T> = flate2::bufread::ZlibDecoder<T>;

#[derive(Debug)]
enum ZState {
    Prepend(Cursor<Vec<u8>>, QIODevice<QTcpSocket>),
    Direct(QIODevice<QTcpSocket>),
}

impl ZState {
    pub fn into_socket(self) -> QIODevice<QTcpSocket> {
        match self {
            Self::Prepend(_, socket) => socket,
            Self::Direct(socket) => socket,
        }
    }
}

macro_rules! impl_read {
    ($me:ident, $buft:ty) => {
        fn $me(&mut self, buf: &mut $buft) -> io::Result<usize> {
            #[allow(unused_mut)]
            let (mut socket, mut reached) = match self {
                Self::Direct(socket) => return socket.$me(buf),
                Self::Prepend(prepend, socket) => {
                    let reached = prepend.$me(buf)?;
                    if (reached == buf.len()) {
                        return Ok(reached);
                    }
                    (socket.clone(), reached)
                }
            };
            reached += socket.$me(&mut buf[reached..])?;
            *self = Self::Direct(socket);
            Ok(reached)
        }
    };
}

impl Read for ZState {
    impl_read!(read, [u8]);
    impl_read!(read_vectored, [IoSliceMut]);
}

enum StreamState {
    Uncompressed(QIODevice<QTcpSocket>),
    Compressed(Decompress<BufReader<ZState>>),
    Transitioning,
}

impl StreamState {
    pub fn into_socket(self) -> QIODevice<QTcpSocket> {
        match self {
            StreamState::Uncompressed(socket) => socket,
            StreamState::Compressed(reader) => reader.into_inner().into_inner().into_socket(),
            StreamState::Transitioning => unreachable!(),
        }
    }
}

pub struct MudStream(StreamState);

impl MudStream {
    pub const fn new(socket: QIODevice<QTcpSocket>) -> Self {
        Self(StreamState::Uncompressed(socket))
    }

    pub fn start_decompressing(&mut self, prepend: Vec<u8>) {
        let mut buf = StreamState::Transitioning;
        mem::swap(&mut self.0, &mut buf);
        let socket = buf.into_socket();
        let inner = if prepend.is_empty() {
            ZState::Direct(socket)
        } else {
            ZState::Prepend(Cursor::new(prepend), socket)
        };
        self.0 = StreamState::Compressed(Decompress::new(BufReader::with_capacity(
            config::COMPRESS_BUFFER,
            inner,
        )));
    }

    pub fn reset(&mut self) {
        let mut buf = StreamState::Transitioning;
        mem::swap(&mut self.0, &mut buf);
        self.0 = StreamState::Uncompressed(buf.into_socket());
    }
}

impl Read for MudStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &mut self.0 {
            StreamState::Uncompressed(r) => r.read(buf),
            StreamState::Compressed(r) => r.read(buf),
            StreamState::Transitioning => unreachable!(),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        match &mut self.0 {
            StreamState::Uncompressed(r) => r.read_vectored(bufs),
            StreamState::Compressed(r) => r.read_vectored(bufs),
            StreamState::Transitioning => unreachable!(),
        }
    }
}
