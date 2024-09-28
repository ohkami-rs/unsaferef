#[doc(hidden)]
#[doc = include_str!("../README.md")]
mod test_readme {}

mod unsafe_ref;
pub use unsafe_ref::UnsafeRef;

#[cfg(feature="alloc")]
mod unsafe_cow;
#[cfg(feature="alloc")]
pub use unsafe_cow::UnsafeCow;
