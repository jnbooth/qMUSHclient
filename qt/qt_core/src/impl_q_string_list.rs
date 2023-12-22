use std::ops::Deref;

use cpp_core::ops::{Begin, BeginMut, End, EndMut};
use cpp_core::vector_ops::Size;

use crate::iter::{TakeFirst, TakeLast};
use crate::QStringList;

impl Size for QStringList {
    #[inline(always)]
    unsafe fn size(&self) -> usize {
        Size::size(self.deref())
    }
}

impl TakeFirst for QStringList {
    type Output = <<Self as Deref>::Target as TakeFirst>::Output;
    #[inline(always)]
    unsafe fn take_first(&self) -> Self::Output {
        self.deref().take_first()
    }
}

impl TakeLast for QStringList {
    type Output = <<Self as Deref>::Target as TakeLast>::Output;
    #[inline(always)]
    unsafe fn take_last(&self) -> Self::Output {
        self.deref().take_last()
    }
}

impl BeginMut for QStringList {
    type Output = <<Self as Deref>::Target as BeginMut>::Output;
    #[inline(always)]
    unsafe fn begin_mut(&self) -> Self::Output {
        self.deref().begin_mut()
    }
}

impl EndMut for QStringList {
    type Output = <<Self as Deref>::Target as EndMut>::Output;
    #[inline(always)]
    unsafe fn end_mut(&self) -> Self::Output {
        self.deref().end_mut()
    }
}

impl Begin for QStringList {
    type Output = <<Self as Deref>::Target as Begin>::Output;
    #[inline(always)]
    unsafe fn begin(&self) -> Self::Output {
        self.deref().begin()
    }
}

impl End for QStringList {
    type Output = <<Self as Deref>::Target as End>::Output;
    #[inline(always)]
    unsafe fn end(&self) -> Self::Output {
        self.deref().end()
    }
}
