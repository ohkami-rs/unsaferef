use crate::UnsafeRef;

pub enum UnsafeCow<T: ?Sized + ToOwned> {
    Ref(UnsafeRef<T>),
    Own(<T as ToOwned>::Owned)
}

impl<T: ?Sized + ToOwned> UnsafeCow<T> {
    #[inline]
    pub fn to_mut(&mut self) -> &mut <T as ToOwned>::Owned {
        if let Self::Ref(r) = self {
            *self = Self::Own((&**r).to_owned())
        }
        let Self::Own(o) = self else {unreachable!()};
        o
    }
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

impl<T: ?Sized + ToOwned + std::hash::Hash> std::hash::Hash for UnsafeCow<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (&**self).hash(state);
    }
}

impl<T: ?Sized + ToOwned + std::fmt::Debug> std::fmt::Debug for UnsafeCow<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, f)
    }
}
impl<T: ?Sized + ToOwned + std::fmt::Display> std::fmt::Display for UnsafeCow<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized + ToOwned + PartialEq> PartialEq for UnsafeCow<T>
where
    <T as ToOwned>::Owned: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ref(r1), Self::Ref(r2)) => r1 == r2,
            (Self::Own(o1), Self::Own(o2)) => o1 == o2,
            _ => false
        }
    }
}
impl<T: ?Sized + ToOwned + PartialEq> PartialEq<T> for UnsafeCow<T>
where
    <T as ToOwned>::Owned: PartialEq<T>
{
    fn eq(&self, other: &T) -> bool {
        match self {
            Self::Ref(r) => r == other,
            Self::Own(o) => o == other,
        }
    }
}
impl<'t, T: ?Sized + ToOwned + PartialEq> PartialEq<&'t T> for UnsafeCow<T>
where
    <T as ToOwned>::Owned: PartialEq<&'t T>
{
    fn eq(&self, other: &&'t T) -> bool {
        match self {
            Self::Ref(r) => r == other,
            Self::Own(o) => o == other,
        }
    }
}
impl<T: ?Sized + ToOwned + PartialEq> Eq for UnsafeCow<T>
where <T as ToOwned>::Owned: PartialEq {}
