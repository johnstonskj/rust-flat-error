# Package `flat_error`

Simple Error wrapper to ensure Clone, Debug, and PartialEq.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![Rust Workflow](https://github.com/johnstonskj/rust-flat-error/actions/workflows/rust.yml/badge.svg)](<https://github.com/johnstonskj/rust-flat-error/actions/workflows/rust.yml>)
[![Security Audit Workflow](https://github.com/johnstonskj/rust-flat-error/actions/workflows/security-audit.yml/badge.svg)](<https://github.com/johnstonskj/rust-flat-error/actions/workflows/security-audit.yml>)
[![Coverage Status](https://codecov.io/github/johnstonskj/rust-flat-error/graph/badge.svg?token=1HGN6M4KIT)](<https://codecov.io/github/johnstonskj/rust-flat-error>)
[![crates.io](https://img.shields.io/crates/v/flat_error.svg)](https://crates.io/crates/flat_error)
[![docs.rs](https://docs.rs/xml_dom/badge.svg)](https://docs.rs/flat_error)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-flat-error.svg)](<https://github.com/johnstonskj/rust-flat-error/stargazers>)

In general it is recommended that error types implement not just [`Error`] and it's required [`Display`], but also
[`Clone`], [`Debug`], and [`PartialEq`] although [`Eq`] is optional. Beyond these additional traits are added as
normal such as `Copy`, `PartialOrd`, `Ord`, `Hash`, etc.

However, there are a number of error types in the standard library, and many more in commonly used traits that do
not meet this requirement. This simple crate introduces a new trait, [`ExtendedError`], which provides these additional
requirements and a new concrete type, [`FlatError`] which provides a way to capture existing errors such that they
meet these requirements.

## Example

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

## Features

| Name    | Dependencies | Description                                                            |
|---------|--------------|------------------------------------------------------------------------|
| `std`   | `alloc`      | Enables the `std` library crate, the most common default.              |
| `alloc` |              | Enables the `alloc` library crate, required in a `no_std` environment. |

## License(s)

The contents of this repository are made available under the following
licenses:

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-flat-error/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-flat-error/blob/main/LICENSE-MIT).

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

For information on contributing to this project, see the following.

1. Project [Code of Conduct](https://github.com/johnstonskj/rust-flat-error/blob/main/CODE_OF_CONDUCT.md).
1. Project [Contribution Guidelines](https://github.com/johnstonskj/rust-flat-error/blob/main/CONTRIBUTING.md).
1. Project [TODO Items](<https://github.com/johnstonskj/rust-flat-error/issues>) in Issues.
1. Repository [Change Log](https://github.com/johnstonskj/rust-flat-error/blob/main/CHANGELOG.md).
