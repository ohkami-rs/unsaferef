use core::ptr::NonNull;

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

impl<T: ?Sized> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<T: ?Sized> Copy for UnsafeRef<T> {}

impl<T: ?Sized> core::ops::Deref for UnsafeRef<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: guaranteed by the SAFETY condition of `UnsafeRef::new`
        unsafe {self.0.as_ref()}
    }
}

impl<T: ?Sized + core::hash::Hash> core::hash::Hash for UnsafeRef<T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (&**self).hash(state);
    }
}

impl<T: ?Sized + core::fmt::Debug> core::fmt::Debug for UnsafeRef<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UnsafeRef")
            .field(&&**self)
            .finish()
    }
}
impl<T: ?Sized + core::fmt::Display> core::fmt::Display for UnsafeRef<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&**self, f)
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

impl<T: ?Sized + PartialEq> Eq for UnsafeRef<T> {}
