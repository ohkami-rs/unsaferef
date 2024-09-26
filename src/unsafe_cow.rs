use crate::UnsafeRef;

pub enum UnsafeCow<T: ?Sized + ToOwned> {
    Ref(UnsafeRef<T>),
    Own(<T as ToOwned>::Owned)
}

impl<T: ?Sized + ToOwned> Clone for UnsafeCow<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Ref(r) => Self::Ref(*r),
            Self::Own(o) => Self::Own(std::borrow::Borrow::borrow(o).to_owned())
        }
    }
}

impl<T: ?Sized + ToOwned> std::ops::Deref for UnsafeCow<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(r) => &**r,
            Self::Own(o) => std::borrow::Borrow::borrow(o)
        }
    }
}

impl<T: ?Sized + ToOwned> std::ops::DerefMut for UnsafeCow<T>
where
    <T as ToOwned>::Owned: std::borrow::BorrowMut<T>
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Self::Ref(r) = self {
            *self = Self::Own((&**r).to_owned())
        }
        let Self::Own(o) = self else {unsafe {std::hint::unreachable_unchecked()}};
        std::borrow::BorrowMut::borrow_mut(o)
    }
}
