# PathExt

A small collection of utilities on [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) and [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html), technically [`AsRef<Path>`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) which includes [`&str`](https://doc.rust-lang.org/std/primitive.str.html).

## Simple usage

```rust
use pathext::PathExt;
use std::ops::Not;
use std::path::Path;

// Because of this expectation breaking difference:
assert!("archive.tar.gz").ends_with(".tar.gz"));
assert!(Path::new("archive.tar.gz").ends_with(".tar.gz").not());
// Instead use:
assert!("archive.tar.gz").ends_with_extentions(".tar.gz"));
assert!(Path::new()"archive.tar.gz")).ends_with_extentions(".tar.gz"));

// Plus some more utility
assert!("/some/path".has_component("path"));
assert!("multiple-extensions.tar.gz".strip_extensions(), Some("multiple-extensions"));
```
