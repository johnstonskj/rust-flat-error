/*!
Simple Error wrapper to ensure Clone, Debug, and PartialEq..

In general it is recommended that error types implement not just [`Error`] and it's required [`Display`], but also
[`Clone`], [`Debug`], and [`PartialEq`] although [`Eq`] is optional. Beyond these additional traits are added as
normal such as `Copy`, `PartialOrd`, `Ord`, `Hash`, etc.

However, there are a number of error types in the standard library, and many more in commonly used traits that do
not meet this requirement. This simple crate introduces a new trait, [`ExtendedError`], which provides these additional
requirements and a new concrete type, [`FlatError`] which provides a way to capture existing errors such that they
meet these requirements.

# Example

The following demonstrates a new type that meets the requirements of `ExtendedError`.

```rust
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "MyError")
    }
}

impl Error for MyError {}
```

However, the following fails because the error in `std::io` does not implement either `Clone` or `PartialEq`.

```rust,compile_fail
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

#[derive(Clone, Debug, PartialEq)]
pub enum MyError {
    Io(IoError), // <= this doesn't work.
}
```

`FlatError` allows the capture and *flattening* of these errors into `MyError`, as shown below. This does however lose
the ability to access any specific methods on the flattened error such as `last_os_error` on `std::io::Error`. 

```rust
use flat_error::FlatError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

#[derive(Clone, Debug, PartialEq)]
pub enum MyError {
    Io(FlatError),
}
```

# Features

- **std**; Uses the `std` library. This is only really relevant for implementing `From` for errors in the `std`
  crate.
- **alloc**; Uses the `alloc` and `core` libraries.

*/

#![warn(
    unknown_lints,
    // ---------- StylisticT
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]
#![cfg_attr(all(feature = "alloc", not(feature = "std")), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
/*
 * The following is no longer supported as there is no known external crate named
 * `std` after approximately edition 2018. However, it does make a nice logical
 * follow-on from the line above for the `alloc` crate.
 *
 *  ```
 * #[cfg(any(test, feature = "std"))]
 * extern crate std;
 * ```
 */

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use core::{
    any::type_name_of_val,
    clone::Clone,
    cmp::PartialEq,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait captures a minimum set of behaviors for error types that includes `Clone`, `Debug`,
/// and `PartialEq`.
///
/// There is a blanket implementation for any type that implements all of these requirements and may
/// then be used as a type constraint, as shown in the following example.
///
/// ```rust
/// use flat_error::ExtendedError;
///
/// fn clone_error(err: &impl ExtendedError) -> impl ExtendedError {
///     println!("Cloning the error {err:?}");
///     err.clone()
/// }
/// ```
///
pub trait ExtendedError: Error + Clone + Debug + PartialEq {}

///
/// A `FlatError` is used to capture an error that does not meet the requirements of the trait
/// [`ExtendedError`] and flatten it into a form that does.
///
/// To do this it captures the value of the errors `Display` implementation, used in flat errors
/// own `Display` implementation. It will also flatten any error returned by `Error::source`.
/// Finally, it captures the type name of the original error for debugging.
///
/// Note also that the blanket implementation for `ExtendedError` applies to `FlatError`.
///
#[derive(Clone, Debug, PartialEq)]
pub struct FlatError {
    original_type_name: &'static str,
    message: String,
    source: Option<Box<Self>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ ExtendedError
// ------------------------------------------------------------------------------------------------

impl<E: Error + Clone + Debug + PartialEq> ExtendedError for E {}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ FlatError
// ------------------------------------------------------------------------------------------------

impl Display for FlatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            write!(f, "{} ({}{})",
                   self.message,
                   self.source.as_ref().map(|e|format!("source: {e}, ")).unwrap_or_default(),
                   format!("original type: `{}`", self.original_type_name),
            )
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl Error for FlatError {
    #[allow(trivial_casts)]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Casts to trait objects can be a pain, this is perfectly legal but will generate
        // a warning; however, removing it generates an error.
        self.flat_source().map(|err| err as &dyn Error)
    }
}

impl FlatError {
    ///
    /// Construct a new `FlatError` by flattening the provided `error`.
    ///
    pub fn from_any<E>(error: &E) -> Self
    where
        E: Error + ?Sized,
    {
        Self {
            original_type_name: type_name_of_val(error),
            message: error.to_string(),
            source: error.source().map(|err| Box::new(FlatError::from_any(err))),
        }
    }

    ///
    /// A concrete version of `Error::source` that returns the flattened source.
    ///
    pub fn flat_source(&self) -> Option<&FlatError> {
        self.source.as_ref().map(|source| source.as_ref())
    }

    ///
    /// Return the name of the flattened error type. This uses the `type_name_of_val` function from
    /// `std::any`, with the warning:
    ///
    /// > Like type_name, this is intended for diagnostic use and the exact output is not guaranteed.
    /// > It provides a best-effort description, but the output may change between versions of the
    /// > compiler.
    ///
    pub fn original_type_name(&self) -> &'static str {
        self.original_type_name
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ core
// ------------------------------------------------------------------------------------------------

impl From<::core::array::TryFromSliceError> for FlatError {
    fn from(e: ::core::array::TryFromSliceError) -> Self {
        FlatError::from_any(&e)
    }
}

impl From<::core::cell::BorrowError> for FlatError {
    fn from(e: ::core::cell::BorrowError) -> Self {
        FlatError::from_any(&e)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ std
// ------------------------------------------------------------------------------------------------

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl From<::std::env::JoinPathsError> for FlatError {
    fn from(e: ::std::env::JoinPathsError) -> Self {
        FlatError::from_any(&e)
    }
}

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl From<::std::fs::TryLockError> for FlatError {
    fn from(e: ::std::fs::TryLockError) -> Self {
        FlatError::from_any(&e)
    }
}

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl From<::std::io::Error> for FlatError {
    fn from(e: ::std::io::Error) -> Self {
        FlatError::from_any(&e)
    }
}

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl<W> From<::std::io::IntoInnerError<W>> for FlatError
where
    W: ::std::fmt::Debug + ::std::marker::Send,
{
    fn from(e: ::std::io::IntoInnerError<W>) -> Self {
        FlatError::from_any(&e)
    }
}

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl From<::std::io::WriterPanicked> for FlatError {
    fn from(e: ::std::io::WriterPanicked) -> Self {
        FlatError::from_any(&e)
    }
}

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl From<::std::string::FromUtf16Error> for FlatError {
    fn from(e: ::std::string::FromUtf16Error) -> Self {
        FlatError::from_any(&e)
    }
}

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl<T> From<::std::sync::PoisonError<T>> for FlatError {
    fn from(e: ::std::sync::PoisonError<T>) -> Self {
        FlatError::from_any(&e)
    }
}

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl<T> From<::std::sync::TryLockError<T>> for FlatError {
    fn from(e: ::std::sync::TryLockError<T>) -> Self {
        FlatError::from_any(&e)
    }
}

#[cfg(any(not(feature = "alloc"), feature = "std"))]
impl From<::std::time::SystemTimeError> for FlatError {
    fn from(e: ::std::time::SystemTimeError) -> Self {
        FlatError::from_any(&e)
    }
}
