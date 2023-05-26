use std::{mem, ptr};

use mem::MaybeUninit;

/// Pulls `N` items from `iter` and returns them as an array. If the iterator
/// yields fewer than `N` items, `None` is returned and all already yielded
/// items are dropped.
///
/// Since the iterator is passed as a mutable reference and this function calls
/// `next` at most `N` times, the iterator can still be used afterwards to
/// retrieve the remaining items.
///
/// If `iter.next()` panicks, all items already yielded by the iterator are
/// dropped.
pub fn collect_into_array<I, const N: usize>(iter: &mut I) -> Option<[I::Item; N]>
where
    I: Iterator,
{
    if N == 0 {
        // SAFETY: An empty array is always inhabited and has no validity invariants.
        return unsafe { Some(mem::zeroed()) };
    }

    struct Guard<'a, T, const N: usize> {
        array_mut: &'a mut [MaybeUninit<T>; N],
        initialized: usize,
    }

    impl<T, const N: usize> Drop for Guard<'_, T, N> {
        fn drop(&mut self) {
            debug_assert!(self.initialized <= N);

            let initialized_part =
                ptr::slice_from_raw_parts_mut(self.array_mut.as_mut_ptr(), self.initialized);

            // SAFETY: this raw slice will contain only initialized objects.
            unsafe {
                ptr::drop_in_place(initialized_part);
            }
        }
    }

    // SAFETY: An uninitialized `[MaybeUninit<I::Item>; N]` is valid.
    let mut array = unsafe { MaybeUninit::<[MaybeUninit<I::Item>; N]>::uninit().assume_init() };
    let mut guard = Guard {
        array_mut: &mut array,
        initialized: 0,
    };

    while let Some(item) = iter.next() {
        // SAFETY: `guard.initialized` starts at 0, is increased by one in the
        // loop and the loop is aborted once it reaches N (which is
        // `array.len()`).
        unsafe {
            guard.array_mut.get_unchecked_mut(guard.initialized).write(item);
        }
        guard.initialized += 1;

        // Check if the whole array was initialized.
        if guard.initialized == N {
            mem::forget(guard);

            // SAFETY: the condition above asserts that all elements are
            // initialized.
            let out = unsafe { (&array as *const _ as *const [I::Item; N]).read() };
            return Some(out);
        }
    }

    // This is only reached if the iterator is exhausted before
    // `guard.initialized` reaches `N`. Also note that `guard` is dropped here,
    // dropping all already initialized elements.
    None
}
