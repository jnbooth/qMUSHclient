// This is the worst piece of code I've ever written.
// I hope it will not be taken as indicative of my overall approach to programming.

use std::io::{BufReader, Read};
use std::mem::ManuallyDrop;

// Yep—it all depends on creating a struct with exactly the same layout as the official BufReader
// so I can poke at fields that are supposed to be private. Obviously, this is terribly unsafe!
struct PrependBufReader<R> {
    _inner: R,
    buf: Box<[u8]>,
    _pos: usize,
    cap: usize,
}

union MaybePrepend<R> {
    prepend: ManuallyDrop<PrependBufReader<R>>,
    bufread: ManuallyDrop<BufReader<R>>,
}

// Long-term options:
// * Make PrependBufReader a real struct, with the functionality copied over from BufReader
// * Use something like Read::chain
/// Yields a BufReader with some of its buffer filled from the start.
pub fn prepend_buf_reader<R: Read>(capacity: usize, inner: R, to_prepend: &[u8]) -> BufReader<R> {
    let mut buf = MaybePrepend {
        bufread: ManuallyDrop::new(BufReader::with_capacity(capacity, inner)),
    };
    let prepend = unsafe { &mut buf.prepend };
    let len = to_prepend.len();
    prepend.buf[..len].copy_from_slice(to_prepend);
    prepend.cap = len;
    unsafe { ManuallyDrop::into_inner(buf.bufread) }
}
