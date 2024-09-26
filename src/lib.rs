#[cfg(test)]
#[doc(hidden)]
mod test_readme {
    #![doc = include_str!("../README.md")]
}

mod unsafe_ref;
mod unsafe_cow;

pub use unsafe_ref::UnsafeRef;
pub use unsafe_cow::UnsafeCow;
