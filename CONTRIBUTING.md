# How to contribute

I'm really glad you're reading this, because we need volunteer developers to
help this project continue to grow and improve.

1. file [bugs](https://github.com/johnstonskj/rust-flat-error/issues/new?assignees=&labels=bug&template=bug_report.md) and
   [enhancement requests](https://github.com/johnstonskj/rust-flat-error/issues/new?assignees=&labels=enhancement&template=feature_request.md)
2. review the project documentation know if you find are issues, or missing
   content, there
3. Fix or Add something and send us a pull request; you may like to pick up
   one of the issues marked [help wanted](https://github.com/johnstonskj/rust-flat-error/labels/help%20wanted) or [good
   first issue](https://github.com/johnstonskj/rust-flat-error/labels/good%20first%20issue) as an introduction.
   Alternatively, [documentation](https://github.com/johnstonskj/rust-flat-error/labels/documentation) issues can be a
   great way to understand the project and help improve the developer
   experience.

## Submitting changes

We love pull requests from everyone. By participating in this project, you
agree to abide by our [code of conduct](https://github.com/johnstonskj/rust-flat-error/blob/main/CODE_OF_CONDUCT.md),
and license files [Apache-2.0](https://github.com/johnstonskj/rust-flat-error/blob/main/LICENSE-APACHE)
and [MIT](https://github.com/johnstonskj/rust-flat-error/blob/main/LICENSE-MIT).

Fork, then clone the repo:

    git clone https://github.com/johnstonskj/rust-flat-error.git

Ensure you have a good Rust install, usually managed by
[Rustup](https://rustup.rs/). You can ensure the latest tools with the
following:

    rustup update

Make your change. Add tests, and documentation, for your change. Ensure not
only that tests pass, but the following all run successfully.

    cargo check --workspace --all-targets --all-features
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings

Make sure the tests pass. Note the three test passes which check for weird
cases which can creap in with features.

    cargo test --workspace --no-fail-fast --all-features
    cargo test --workspace --no-fail-fast --all-targets --all-features
    cargo test --workspace --no-fail-fast --no-default-targets --all-features
    cargo doc --workspace --all-features --no-deps

If you made changes to the book source, ensure the following runs successfully

    mdbook build

If you have made any changes to `Cargo.toml`, also check:

    cargo audit
    cargo outdated --workspace --root-deps-only

Push to your fork and [submit a pull request](https://github.com/johnstonskj/rust-flat-error/compare/) using our
[template](https://github.com/johnstonskj/rust-flat-error/blob/main/.github/pull_request_template.md).

At this point you're waiting on us. We like to at least comment on pull
requests within three business days (and, typically, one business day). We may
suggest some changes or improvements or alternatives.

Some things that will increase the chance that your pull request is accepted:

* Write unit tests.
* Write API documentation.
* Write a [good commit message](https://cbea.ms/git-commit/https://cbea.ms/git-commit/).

## Coding conventions

The primary tool for coding conventions is rustfmt, and specifically `cargo
fmt` is a part of the build process and will cause Actions to fail.

DO NOT create or update any existing `rustfmt.toml` file to change the default
formatting rules.

DO NOT alter any `warn` or `deny` library attributes.

DO NOT add any `feature` attributes that would prohibit building on the stable
channel. In some cases new crate-level features can be used to introduce an
unstable feature dependency but these MUST be clearly documented and optional.

DO NOT introduce `unsafe` code without a discussion on alternatives first.
Ideally any unsafe code will be introduced as optional features.
