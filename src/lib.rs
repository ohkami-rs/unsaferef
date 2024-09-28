#[doc(hidden)]
#[doc = include_str!("../README.md")]
mod test_readme {}

mod unsafe_ref;
mod unsafe_cow;

pub use unsafe_ref::UnsafeRef;
pub use unsafe_cow::UnsafeCow;
