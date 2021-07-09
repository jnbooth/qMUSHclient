use std::io::{self, BufReader, Cursor, IoSliceMut, Read};

use qt_network::QTcpSocket;

use crate::binding::RIODevice;
use crate::constants::config;

type Decompress<T> = flate2::bufread::ZlibDecoder<T>;

#[derive(Debug)]
enum ZState {
    Prepend(Cursor<Vec<u8>>, RIODevice<QTcpSocket>),
    Direct(RIODevice<QTcpSocket>),
}

macro_rules! impl_read {
    ($me:ident, $buft:ty) => {
        fn $me(&mut self, buf: &mut $buft) -> io::Result<usize> {
            #[allow(unused_mut)]
            let mut socket = match self {
                Self::Direct(socket) => return socket.$me(buf),
                Self::Prepend(prepend, socket) => {
                    if prepend.position() as usize != prepend.get_ref().len() {
                        return prepend.$me(buf);
                    }
                    socket.clone()
                }
            };
            let res = socket.$me(buf);
            *self = Self::Direct(socket);
            res
        }
    };
}

impl Read for ZState {
    impl_read!(read, [u8]);
    impl_read!(read_vectored, [IoSliceMut<'_>]);
}

enum StreamState {
    Uncompressed(RIODevice<QTcpSocket>),
    Compressed(Decompress<BufReader<ZState>>),
}

pub struct MudStream(StreamState);

impl MudStream {
    pub const fn new(socket: RIODevice<QTcpSocket>) -> Self {
        Self(StreamState::Uncompressed(socket))
    }

    pub fn start_decompressing(&mut self, prepend: Vec<u8>) {
        let socket = match &self.0 {
            StreamState::Uncompressed(socket) => socket.clone(),
            _ => return,
        };
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
}

impl Read for MudStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &mut self.0 {
            StreamState::Uncompressed(r) => r.read(buf),
            StreamState::Compressed(r) => r.read(buf),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        match &mut self.0 {
            StreamState::Uncompressed(r) => r.read_vectored(bufs),
            StreamState::Compressed(r) => r.read_vectored(bufs),
        }
    }
}
