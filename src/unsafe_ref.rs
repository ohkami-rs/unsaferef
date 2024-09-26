use std::ptr::NonNull;

pub struct UnsafeRef<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> UnsafeRef<T> {
    /// SAFETY: `reference` must be valid for the entire lifetime of the `UnsafeRef`.
    #[inline]
    pub const unsafe fn new(reference: &T) -> Self {
        Self(NonNull::new_unchecked(reference as *const T as *mut T))
    }
}

unsafe impl<T: ?Sized> Send for UnsafeRef<T> {}
unsafe impl<T: ?Sized> Sync for UnsafeRef<T> {}

impl<T: ?Sized> Copy for UnsafeRef<T> {}
impl<T: ?Sized> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: ?Sized> std::ops::Deref for UnsafeRef<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: guaranteed by the SAFETY condition of `UnsafeRef::new`
        unsafe {self.0.as_ref()}
    }
}

impl<T: ?Sized + std::fmt::Debug> std::fmt::Debug for UnsafeRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UnsafeRef")
            .field(&&**self)
            .finish()
    }
}
impl<T: ?Sized + std::fmt::Display> std::fmt::Display for UnsafeRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized + PartialEq> PartialEq for UnsafeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        &**self == &**other
    }
}
impl<T: ?Sized + PartialEq> PartialEq<T> for UnsafeRef<T> {
    fn eq(&self, other: &T) -> bool {
        &**self == other
    }
}
impl<T: ?Sized + PartialEq> PartialEq<&T> for UnsafeRef<T> {
    fn eq(&self, other: &&T) -> bool {
        &**self == *other
    }
}
