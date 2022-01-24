# PathExt

A small collection of utilities on Paths and PathBufs.

## Simple usage

```rust
use pathext::PathExt;
assert!("/some/path".has_component("path"));
assert!("multiple-extensions.tar.gz".strip_extensions(), Some("multiple-extensions"));
```
