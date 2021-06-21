#![allow(unused_unsafe, dead_code)]

use std::any::type_name;
use std::{fmt, mem, ptr};

#[derive(Copy)]
#[repr(transparent)]
pub struct MaybeUninit<T>(mem::MaybeUninit<T>);

impl<T: Copy> Clone for MaybeUninit<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        // Not calling `T::clone()`, we cannot know if we are initialized enough for that.
        *self
    }
}

impl<T> fmt::Debug for MaybeUninit<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(type_name::<Self>())
    }
}

impl<T> MaybeUninit<T> {
    /// Creates a new `MaybeUninit<T>` initialized with the given value.
    /// It is safe to call [`assume_init`] on the return value of this function.
    ///
    /// Note that dropping a `MaybeUninit<T>` will never call `T`'s drop code.
    /// It is your responsibility to make sure `T` gets dropped if it got initialized.
    ///
    /// # Example
    ///
    /// ```
    /// use std::mem::MaybeUninit;
    ///
    /// let v: MaybeUninit<Vec<u8>> = MaybeUninit::new(vec![42]);
    /// ```
    ///
    /// [`assume_init`]: MaybeUninit::assume_init
    #[inline(always)]
    pub const fn new(val: T) -> MaybeUninit<T> {
        MaybeUninit(mem::MaybeUninit::new(val))
    }

    /// Creates a new `MaybeUninit<T>` in an uninitialized state.
    ///
    /// Note that dropping a `MaybeUninit<T>` will never call `T`'s drop code.
    /// It is your responsibility to make sure `T` gets dropped if it got initialized.
    ///
    /// See the [type-level documentation][MaybeUninit] for some examples.
    ///
    /// # Example
    ///
    /// ```
    /// use std::mem::MaybeUninit;
    ///
    /// let v: MaybeUninit<String> = MaybeUninit::uninit();
    /// ```
    #[inline(always)]
    pub const fn uninit() -> MaybeUninit<T> {
        MaybeUninit(mem::MaybeUninit::uninit())
    }

    /// Create a new array of `MaybeUninit<T>` items, in an uninitialized state.
    ///
    /// Note: in a future Rust version this method may become unnecessary
    /// when array literal syntax allows
    /// [repeating const expressions](https://github.com/rust-lang/rust/issues/49147).
    /// The example below could then use `let mut buf = [MaybeUninit::<u8>::uninit(); 32];`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(maybe_uninit_uninit_array, maybe_uninit_extra, maybe_uninit_slice)]
    ///
    /// use std::mem::MaybeUninit;
    ///
    /// extern "C" {
    ///     fn read_into_buffer(ptr: *mut u8, max_len: usize) -> usize;
    /// }
    ///
    /// /// Returns a (possibly smaller) slice of data that was actually read
    /// fn read(buf: &mut [MaybeUninit<u8>]) -> &[u8] {
    ///     unsafe {
    ///         let len = read_into_buffer(buf.as_mut_ptr() as *mut u8, buf.len());
    ///         MaybeUninit::slice_assume_init_ref(&buf[..len])
    ///     }
    /// }
    ///
    /// let mut buf: [MaybeUninit<u8>; 32] = MaybeUninit::uninit_array();
    /// let data = read(&mut buf);
    /// ```
    #[inline(always)]
    pub fn uninit_array<const LEN: usize>() -> [Self; LEN] {
        // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
        unsafe { MaybeUninit::<[MaybeUninit<T>; LEN]>::uninit().assume_init() }
    }

    /// Creates a new `MaybeUninit<T>` in an uninitialized state, with the memory being
    /// filled with `0` bytes. It depends on `T` whether that already makes for
    /// proper initialization. For example, `MaybeUninit<usize>::zeroed()` is initialized,
    /// but `MaybeUninit<&'static i32>::zeroed()` is not because references must not
    /// be null.
    ///
    /// Note that dropping a `MaybeUninit<T>` will never call `T`'s drop code.
    /// It is your responsibility to make sure `T` gets dropped if it got initialized.
    ///
    /// # Example
    ///
    /// Correct usage of this function: initializing a struct with zero, where all
    /// fields of the struct can hold the bit-pattern 0 as a valid value.
    ///
    /// ```rust
    /// use std::mem::MaybeUninit;
    ///
    /// let x = MaybeUninit::<(u8, bool)>::zeroed();
    /// let x = unsafe { x.assume_init() };
    /// assert_eq!(x, (0, false));
    /// ```
    ///
    /// *Incorrect* usage of this function: calling `x.zeroed().assume_init()`
    /// when `0` is not a valid bit-pattern for the type:
    ///
    /// ```rust,no_run
    /// use std::mem::MaybeUninit;
    ///
    /// enum NotZero { One = 1, Two = 2 }
    ///
    /// let x = MaybeUninit::<(u8, NotZero)>::zeroed();
    /// let x = unsafe { x.assume_init() };
    /// // Inside a pair, we create a `NotZero` that does not have a valid discriminant.
    /// // This is undefined behavior. ⚠️
    /// ```
    #[inline]
    pub fn zeroed() -> MaybeUninit<T> {
        let mut u = MaybeUninit::<T>::uninit();
        // SAFETY: `u.as_mut_ptr()` points to allocated memory.
        unsafe {
            u.as_mut_ptr().write_bytes(0u8, 1);
        }
        u
    }

    /// Sets the value of the `MaybeUninit<T>`. This overwrites any previous value
    /// without dropping it, so be careful not to use this twice unless you want to
    /// skip running the destructor. For your convenience, this also returns a mutable
    /// reference to the (now safely initialized) contents of `self`.
    #[inline(always)]
    pub fn write(&mut self, val: T) -> &mut T {
        *self = MaybeUninit::new(val);
        // SAFETY: We just initialized this value.
        unsafe { self.assume_init_mut() }
    }

    /// Gets a pointer to the contained value. Reading from this pointer or turning it
    /// into a reference is undefined behavior unless the `MaybeUninit<T>` is initialized.
    /// Writing to memory that this pointer (non-transitively) points to is undefined behavior
    /// (except inside an `UnsafeCell<T>`).
    ///
    /// # Examples
    ///
    /// Correct usage of this method:
    ///
    /// ```rust
    /// use std::mem::MaybeUninit;
    ///
    /// let mut x = MaybeUninit::<Vec<u32>>::uninit();
    /// unsafe { x.as_mut_ptr().write(vec![0, 1, 2]); }
    /// // Create a reference into the `MaybeUninit<T>`. This is okay because we initialized it.
    /// let x_vec = unsafe { &*x.as_ptr() };
    /// assert_eq!(x_vec.len(), 3);
    /// ```
    ///
    /// *Incorrect* usage of this method:
    ///
    /// ```rust,no_run
    /// use std::mem::MaybeUninit;
    ///
    /// let x = MaybeUninit::<Vec<u32>>::uninit();
    /// let x_vec = unsafe { &*x.as_ptr() };
    /// // We have created a reference to an uninitialized vector! This is undefined behavior. ⚠️
    /// ```
    ///
    /// (Notice that the rules around references to uninitialized data are not finalized yet, but
    /// until they are, it is advisable to avoid them.)
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        // `MaybeUninit` and `ManuallyDrop` are both `repr(transparent)` so we can cast the pointer.
        self as *const _ as *const T
    }

    /// Gets a mutable pointer to the contained value. Reading from this pointer or turning it
    /// into a reference is undefined behavior unless the `MaybeUninit<T>` is initialized.
    ///
    /// # Examples
    ///
    /// Correct usage of this method:
    ///
    /// ```rust
    /// use std::mem::MaybeUninit;
    ///
    /// let mut x = MaybeUninit::<Vec<u32>>::uninit();
    /// unsafe { x.as_mut_ptr().write(vec![0, 1, 2]); }
    /// // Create a reference into the `MaybeUninit<Vec<u32>>`.
    /// // This is okay because we initialized it.
    /// let x_vec = unsafe { &mut *x.as_mut_ptr() };
    /// x_vec.push(3);
    /// assert_eq!(x_vec.len(), 4);
    /// ```
    ///
    /// *Incorrect* usage of this method:
    ///
    /// ```rust,no_run
    /// use std::mem::MaybeUninit;
    ///
    /// let mut x = MaybeUninit::<Vec<u32>>::uninit();
    /// let x_vec = unsafe { &mut *x.as_mut_ptr() };
    /// // We have created a reference to an uninitialized vector! This is undefined behavior. ⚠️
    /// ```
    ///
    /// (Notice that the rules around references to uninitialized data are not finalized yet, but
    /// until they are, it is advisable to avoid them.)
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        // `MaybeUninit` and `ManuallyDrop` are both `repr(transparent)` so we can cast the pointer.
        self as *mut _ as *mut T
    }

    /// Extracts the value from the `MaybeUninit<T>` container. This is a great way
    /// to ensure that the data will get dropped, because the resulting `T` is
    /// subject to the usual drop handling.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee that the `MaybeUninit<T>` really is in an initialized
    /// state. Calling this when the content is not yet fully initialized causes immediate undefined
    /// behavior. The [type-level documentation][inv] contains more information about
    /// this initialization invariant.
    ///
    /// [inv]: #initialization-invariant
    ///
    /// On top of that, remember that most types have additional invariants beyond merely
    /// being considered initialized at the type level. For example, a `1`-initialized [`Vec<T>`]
    /// is considered initialized (under the current implementation; this does not constitute
    /// a stable guarantee) because the only requirement the compiler knows about it
    /// is that the data pointer must be non-null. Creating such a `Vec<T>` does not cause
    /// *immediate* undefined behavior, but will cause undefined behavior with most
    /// safe operations (including dropping it).
    ///
    /// [`Vec<T>`]: ../../std/vec/struct.Vec.html
    ///
    /// # Examples
    ///
    /// Correct usage of this method:
    ///
    /// ```rust
    /// use std::mem::MaybeUninit;
    ///
    /// let mut x = MaybeUninit::<bool>::uninit();
    /// unsafe { x.as_mut_ptr().write(true); }
    /// let x_init = unsafe { x.assume_init() };
    /// assert_eq!(x_init, true);
    /// ```
    ///
    /// *Incorrect* usage of this method:
    ///
    /// ```rust,no_run
    /// use std::mem::MaybeUninit;
    ///
    /// let x = MaybeUninit::<Vec<u32>>::uninit();
    /// let x_init = unsafe { x.assume_init() };
    /// // `x` had not been initialized yet, so this last line caused undefined behavior. ⚠️
    /// ```
    #[inline(always)]
    pub unsafe fn assume_init(self) -> T {
        self.0.assume_init()
    }

    /// Reads the value from the `MaybeUninit<T>` container. The resulting `T` is subject
    /// to the usual drop handling.
    ///
    /// Whenever possible, it is preferable to use [`assume_init`] instead, which
    /// prevents duplicating the content of the `MaybeUninit<T>`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee that the `MaybeUninit<T>` really is in an initialized
    /// state. Calling this when the content is not yet fully initialized causes undefined
    /// behavior. The [type-level documentation][inv] contains more information about
    /// this initialization invariant.
    ///
    /// Moreover, this leaves a copy of the same data behind in the `MaybeUninit<T>`. When using
    /// multiple copies of the data (by calling `assume_init_read` multiple times, or first
    /// calling `assume_init_read` and then [`assume_init`]), it is your responsibility
    /// to ensure that that data may indeed be duplicated.
    ///
    /// [inv]: #initialization-invariant
    /// [`assume_init`]: MaybeUninit::assume_init
    ///
    /// # Examples
    ///
    /// Correct usage of this method:
    ///
    /// ```rust
    /// #![feature(maybe_uninit_extra)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut x = MaybeUninit::<u32>::uninit();
    /// x.write(13);
    /// let x1 = unsafe { x.assume_init_read() };
    /// // `u32` is `Copy`, so we may read multiple times.
    /// let x2 = unsafe { x.assume_init_read() };
    /// assert_eq!(x1, x2);
    ///
    /// let mut x = MaybeUninit::<Option<Vec<u32>>>::uninit();
    /// x.write(None);
    /// let x1 = unsafe { x.assume_init_read() };
    /// // Duplicating a `None` value is okay, so we may read multiple times.
    /// let x2 = unsafe { x.assume_init_read() };
    /// assert_eq!(x1, x2);
    /// ```
    ///
    /// *Incorrect* usage of this method:
    ///
    /// ```rust,no_run
    /// #![feature(maybe_uninit_extra)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut x = MaybeUninit::<Option<Vec<u32>>>::uninit();
    /// x.write(Some(vec![0, 1, 2]));
    /// let x1 = unsafe { x.assume_init_read() };
    /// let x2 = unsafe { x.assume_init_read() };
    /// // We now created two copies of the same vector, leading to a double-free ⚠️ when
    /// // they both get dropped!
    /// ```
    #[inline(always)]
    pub unsafe fn assume_init_read(&self) -> T {
        // SAFETY: the caller must guarantee that `self` is initialized.
        // Reading from `self.as_ptr()` is safe since `self` should be initialized.
        unsafe {
            //std::intrinsics::assert_inhabited::<T>();
            self.as_ptr().read()
        }
    }

    /// Drops the contained value in place.
    ///
    /// If you have ownership of the `MaybeUninit`, you can use [`assume_init`] instead.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee that the `MaybeUninit<T>` really is
    /// in an initialized state. Calling this when the content is not yet fully
    /// initialized causes undefined behavior.
    ///
    /// On top of that, all additional invariants of the type `T` must be
    /// satisfied, as the `Drop` implementation of `T` (or its members) may
    /// rely on this. For example, a `1`-initialized [`Vec<T>`] is considered
    /// initialized (under the current implementation; this does not constitute
    /// a stable guarantee) because the only requirement the compiler knows
    /// about it is that the data pointer must be non-null. Dropping such a
    /// `Vec<T>` however will cause undefined behaviour.
    ///
    /// [`assume_init`]: MaybeUninit::assume_init
    /// [`Vec<T>`]: ../../std/vec/struct.Vec.html
    pub unsafe fn assume_init_drop(&mut self) {
        // SAFETY: the caller must guarantee that `self` is initialized and
        // satisfies all invariants of `T`.
        // Dropping the value in place is safe if that is the case.
        unsafe { ptr::drop_in_place(self.as_mut_ptr()) }
    }

    /// Gets a shared reference to the contained value.
    ///
    /// This can be useful when we want to access a `MaybeUninit` that has been
    /// initialized but don't have ownership of the `MaybeUninit` (preventing the use
    /// of `.assume_init()`).
    ///
    /// # Safety
    ///
    /// Calling this when the content is not yet fully initialized causes undefined
    /// behavior: it is up to the caller to guarantee that the `MaybeUninit<T>` really
    /// is in an initialized state.
    ///
    /// # Examples
    ///
    /// ### Correct usage of this method:
    ///
    /// ```rust
    /// #![feature(maybe_uninit_ref)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut x = MaybeUninit::<Vec<u32>>::uninit();
    /// // Initialize `x`:
    /// unsafe { x.as_mut_ptr().write(vec![1, 2, 3]); }
    /// // Now that our `MaybeUninit<_>` is known to be initialized, it is okay to
    /// // create a shared reference to it:
    /// let x: &Vec<u32> = unsafe {
    ///     // SAFETY: `x` has been initialized.
    ///     x.assume_init_ref()
    /// };
    /// assert_eq!(x, &vec![1, 2, 3]);
    /// ```
    ///
    /// ### *Incorrect* usages of this method:
    ///
    /// ```rust,no_run
    /// #![feature(maybe_uninit_ref)]
    /// use std::mem::MaybeUninit;
    ///
    /// let x = MaybeUninit::<Vec<u32>>::uninit();
    /// let x_vec: &Vec<u32> = unsafe { x.assume_init_ref() };
    /// // We have created a reference to an uninitialized vector! This is undefined behavior. ⚠️
    /// ```
    ///
    /// ```rust,no_run
    /// #![feature(maybe_uninit_ref)]
    /// use std::{cell::Cell, mem::MaybeUninit};
    ///
    /// let b = MaybeUninit::<Cell<bool>>::uninit();
    /// // Initialize the `MaybeUninit` using `Cell::set`:
    /// unsafe {
    ///     b.assume_init_ref().set(true);
    ///    // ^^^^^^^^^^^^^^^
    ///    // Reference to an uninitialized `Cell<bool>`: UB!
    /// }
    /// ```
    #[inline(always)]
    pub unsafe fn assume_init_ref(&self) -> &T {
        // SAFETY: the caller must guarantee that `self` is initialized.
        // This also means that `self` must be a `value` variant.
        unsafe {
            //std::intrinsics::assert_inhabited::<T>();
            &*self.as_ptr()
        }
    }

    /// Gets a mutable (unique) reference to the contained value.
    ///
    /// This can be useful when we want to access a `MaybeUninit` that has been
    /// initialized but don't have ownership of the `MaybeUninit` (preventing the use
    /// of `.assume_init()`).
    ///
    /// # Safety
    ///
    /// Calling this when the content is not yet fully initialized causes undefined
    /// behavior: it is up to the caller to guarantee that the `MaybeUninit<T>` really
    /// is in an initialized state. For instance, `.assume_init_mut()` cannot be used to
    /// initialize a `MaybeUninit`.
    ///
    /// # Examples
    ///
    /// ### Correct usage of this method:
    ///
    /// ```rust
    /// #![feature(maybe_uninit_ref)]
    /// use std::mem::MaybeUninit;
    ///
    /// # unsafe extern "C" fn initialize_buffer(buf: *mut [u8; 2048]) { *buf = [0; 2048] }
    /// # #[cfg(FALSE)]
    /// extern "C" {
    ///     /// Initializes *all* the bytes of the input buffer.
    ///     fn initialize_buffer(buf: *mut [u8; 2048]);
    /// }
    ///
    /// let mut buf = MaybeUninit::<[u8; 2048]>::uninit();
    ///
    /// // Initialize `buf`:
    /// unsafe { initialize_buffer(buf.as_mut_ptr()); }
    /// // Now we know that `buf` has been initialized, so we could `.assume_init()` it.
    /// // However, using `.assume_init()` may trigger a `memcpy` of the 2048 bytes.
    /// // To assert our buffer has been initialized without copying it, we upgrade
    /// // the `&mut MaybeUninit<[u8; 2048]>` to a `&mut [u8; 2048]`:
    /// let buf: &mut [u8; 2048] = unsafe {
    ///     // SAFETY: `buf` has been initialized.
    ///     buf.assume_init_mut()
    /// };
    ///
    /// // Now we can use `buf` as a normal slice:
    /// buf.sort_unstable();
    /// assert!(
    ///     buf.windows(2).all(|pair| pair[0] <= pair[1]),
    ///     "buffer is sorted",
    /// );
    /// ```
    ///
    /// ### *Incorrect* usages of this method:
    ///
    /// You cannot use `.assume_init_mut()` to initialize a value:
    ///
    /// ```rust,no_run
    /// #![feature(maybe_uninit_ref)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut b = MaybeUninit::<bool>::uninit();
    /// unsafe {
    ///     *b.assume_init_mut() = true;
    ///     // We have created a (mutable) reference to an uninitialized `bool`!
    ///     // This is undefined behavior. ⚠️
    /// }
    /// ```
    ///
    /// For instance, you cannot [`Read`] into an uninitialized buffer:
    ///
    /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    ///
    /// ```rust,no_run
    /// #![feature(maybe_uninit_ref)]
    /// use std::{io, mem::MaybeUninit};
    ///
    /// fn read_chunk (reader: &'_ mut dyn io::Read) -> io::Result<[u8; 64]>
    /// {
    ///     let mut buffer = MaybeUninit::<[u8; 64]>::uninit();
    ///     reader.read_exact(unsafe { buffer.assume_init_mut() })?;
    ///                             // ^^^^^^^^^^^^^^^^^^^^^^^^
    ///                             // (mutable) reference to uninitialized memory!
    ///                             // This is undefined behavior.
    ///     Ok(unsafe { buffer.assume_init() })
    /// }
    /// ```
    ///
    /// Nor can you use direct field access to do field-by-field gradual initialization:
    ///
    /// ```rust,no_run
    /// #![feature(maybe_uninit_ref)]
    /// use std::{mem::MaybeUninit, ptr};
    ///
    /// struct Foo {
    ///     a: u32,
    ///     b: u8,
    /// }
    ///
    /// let foo: Foo = unsafe {
    ///     let mut foo = MaybeUninit::<Foo>::uninit();
    ///     ptr::write(&mut foo.assume_init_mut().a as *mut u32, 1337);
    ///                  // ^^^^^^^^^^^^^^^^^^^^^
    ///                  // (mutable) reference to uninitialized memory!
    ///                  // This is undefined behavior.
    ///     ptr::write(&mut foo.assume_init_mut().b as *mut u8, 42);
    ///                  // ^^^^^^^^^^^^^^^^^^^^^
    ///                  // (mutable) reference to uninitialized memory!
    ///                  // This is undefined behavior.
    ///     foo.assume_init()
    /// };
    /// ```
    // FIXME(#76092): We currently rely on the above being incorrect, i.e., we have references
    // to uninitialized data (e.g., in `libcore/fmt/float.rs`).  We should make
    // a final decision about the rules before stabilization.
    #[inline(always)]
    pub unsafe fn assume_init_mut(&mut self) -> &mut T {
        // SAFETY: the caller must guarantee that `self` is initialized.
        // This also means that `self` must be a `value` variant.
        unsafe {
            //std::intrinsics::assert_inhabited::<T>();
            &mut *self.as_mut_ptr()
        }
    }

    /// Extracts the values from an array of `MaybeUninit` containers.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee that all elements of the array are
    /// in an initialized state.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(maybe_uninit_uninit_array)]
    /// #![feature(maybe_uninit_array_assume_init)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut array: [MaybeUninit<i32>; 3] = MaybeUninit::uninit_array();
    /// array[0] = MaybeUninit::new(0);
    /// array[1] = MaybeUninit::new(1);
    /// array[2] = MaybeUninit::new(2);
    ///
    /// // SAFETY: Now safe as we initialised all elements
    /// let array = unsafe {
    ///     MaybeUninit::array_assume_init(array)
    /// };
    ///
    /// assert_eq!(array, [0, 1, 2]);
    /// ```
    #[inline(always)]
    pub unsafe fn array_assume_init<const N: usize>(array: [Self; N]) -> [T; N] {
        // SAFETY:
        // * The caller guarantees that all elements of the array are initialized
        // * `MaybeUninit<T>` and T are guaranteed to have the same layout
        // * MaybeUnint does not drop, so there are no double-frees
        // And thus the conversion is safe
        unsafe {
            //std::intrinsics::assert_inhabited::<[T; N]>();
            (&array as *const _ as *const [T; N]).read()
        }
    }

    /// Assuming all the elements are initialized, get a slice to them.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee that the `MaybeUninit<T>` elements
    /// really are in an initialized state.
    /// Calling this when the content is not yet fully initialized causes undefined behavior.
    ///
    /// See [`assume_init_ref`] for more details and examples.
    ///
    /// [`assume_init_ref`]: MaybeUninit::assume_init_ref
    #[inline(always)]
    pub unsafe fn slice_assume_init_ref(slice: &[Self]) -> &[T] {
        // SAFETY: casting slice to a `*const [T]` is safe since the caller guarantees that
        // `slice` is initialized, and`MaybeUninit` is guaranteed to have the same layout as `T`.
        // The pointer obtained is valid since it refers to memory owned by `slice` which is a
        // reference and thus guaranteed to be valid for reads.
        unsafe { &*(slice as *const [Self] as *const [T]) }
    }

    /// Assuming all the elements are initialized, get a mutable slice to them.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee that the `MaybeUninit<T>` elements
    /// really are in an initialized state.
    /// Calling this when the content is not yet fully initialized causes undefined behavior.
    ///
    /// See [`assume_init_mut`] for more details and examples.
    ///
    /// [`assume_init_mut`]: MaybeUninit::assume_init_mut
    #[inline(always)]
    pub unsafe fn slice_assume_init_mut(slice: &mut [Self]) -> &mut [T] {
        // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
        // mutable reference which is also guaranteed to be valid for writes.
        unsafe { &mut *(slice as *mut [Self] as *mut [T]) }
    }

    /// Gets a pointer to the first element of the array.
    #[inline(always)]
    pub const fn slice_as_ptr(this: &[MaybeUninit<T>]) -> *const T {
        this.as_ptr() as *const T
    }

    /// Gets a mutable pointer to the first element of the array.
    #[inline(always)]
    pub fn slice_as_mut_ptr(this: &mut [MaybeUninit<T>]) -> *mut T {
        this.as_mut_ptr() as *mut T
    }

    /// Copies the elements from `src` to `this`, returning a mutable reference to the now initalized contents of `this`.
    ///
    /// If `T` does not implement `Copy`, use [`write_slice_cloned`]
    ///
    /// This is similar to [`slice::copy_from_slice`].
    ///
    /// # Panics
    ///
    /// This function will panic if the two slices have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(maybe_uninit_write_slice)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut dst = [MaybeUninit::uninit(); 32];
    /// let src = [0; 32];
    ///
    /// let init = MaybeUninit::write_slice(&mut dst, &src);
    ///
    /// assert_eq!(init, src);
    /// ```
    ///
    /// ```
    /// #![feature(maybe_uninit_write_slice, vec_spare_capacity)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut vec = Vec::with_capacity(32);
    /// let src = [0; 16];
    ///
    /// MaybeUninit::write_slice(&mut vec.spare_capacity_mut()[..src.len()], &src);
    ///
    /// // SAFETY: we have just copied all the elements of len into the spare capacity
    /// // the first src.len() elements of the vec are valid now.
    /// unsafe {
    ///     vec.set_len(src.len());
    /// }
    ///
    /// assert_eq!(vec, src);
    /// ```
    ///
    /// [`write_slice_cloned`]: MaybeUninit::write_slice_cloned
    pub fn write_slice<'a>(this: &'a mut [MaybeUninit<T>], src: &[T]) -> &'a mut [T]
    where
        T: Copy,
    {
        // SAFETY: &[T] and &[MaybeUninit<T>] have the same layout
        let uninit_src: &[MaybeUninit<T>] = unsafe { mem::transmute(src) };

        this.copy_from_slice(uninit_src);

        // SAFETY: Valid elements have just been copied into `this` so it is initalized
        unsafe { MaybeUninit::slice_assume_init_mut(this) }
    }

    /// Clones the elements from `src` to `this`, returning a mutable reference to the now initalized contents of `this`.
    /// Any already initalized elements will not be dropped.
    ///
    /// If `T` implements `Copy`, use [`write_slice`]
    ///
    /// This is similar to [`slice::clone_from_slice`] but does not drop existing elements.
    ///
    /// # Panics
    ///
    /// This function will panic if the two slices have different lengths, or if the implementation of `Clone` panics.
    ///
    /// If there is a panic, the already cloned elements will be dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(maybe_uninit_write_slice)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut dst = [MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit()];
    /// let src = ["wibbly".to_string(), "wobbly".to_string(), "timey".to_string(), "wimey".to_string(), "stuff".to_string()];
    ///
    /// let init = MaybeUninit::write_slice_cloned(&mut dst, &src);
    ///
    /// assert_eq!(init, src);
    /// ```
    ///
    /// ```
    /// #![feature(maybe_uninit_write_slice, vec_spare_capacity)]
    /// use std::mem::MaybeUninit;
    ///
    /// let mut vec = Vec::with_capacity(32);
    /// let src = ["rust", "is", "a", "pretty", "cool", "language"];
    ///
    /// MaybeUninit::write_slice_cloned(&mut vec.spare_capacity_mut()[..src.len()], &src);
    ///
    /// // SAFETY: we have just cloned all the elements of len into the spare capacity
    /// // the first src.len() elements of the vec are valid now.
    /// unsafe {
    ///     vec.set_len(src.len());
    /// }
    ///
    /// assert_eq!(vec, src);
    /// ```
    ///
    /// [`write_slice`]: MaybeUninit::write_slice
    pub fn write_slice_cloned<'a>(this: &'a mut [MaybeUninit<T>], src: &[T]) -> &'a mut [T]
    where
        T: Clone,
    {
        // unlike copy_from_slice this does not call clone_from_slice on the slice
        // this is because `MaybeUninit<T: Clone>` does not implement Clone.

        struct Guard<'a, T> {
            slice: &'a mut [MaybeUninit<T>],
            initialized: usize,
        }

        impl<'a, T> Drop for Guard<'a, T> {
            fn drop(&mut self) {
                let initialized_part = &mut self.slice[..self.initialized];
                // SAFETY: this raw slice will contain only initialized objects
                // that's why, it is allowed to drop it.
                unsafe {
                    std::ptr::drop_in_place(MaybeUninit::slice_assume_init_mut(initialized_part));
                }
            }
        }

        assert_eq!(
            this.len(),
            src.len(),
            "destination and source slices have different lengths"
        );
        // NOTE: We need to explicitly slice them to the same length
        // for bounds checking to be elided, and the optimizer will
        // generate memcpy for simple cases (for example T = u8).
        let len = this.len();
        let src = &src[..len];

        // guard is needed b/c panic might happen during a clone
        let mut guard = Guard {
            slice: this,
            initialized: 0,
        };

        for i in 0..len {
            guard.slice[i].write(src[i].clone());
            guard.initialized += 1;
        }

        mem::forget(guard);

        // SAFETY: Valid elements have just been written into `this` so it is initalized
        unsafe { MaybeUninit::slice_assume_init_mut(this) }
    }
}
