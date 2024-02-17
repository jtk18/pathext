//! # pathext
//!
//! A simple extension trait that includes some convenience methods I have found useful.
//!

use std::path::Path;

/// ```rust
/// use pathext::PathExt;
/// use std::ops::Not;
/// use std::path::Path;
///
/// assert!("/some/path".has_component("path"));
/// assert!(Path::new("/some/path").has_component("path"));
/// assert!("/some/path".has_component("some/path").not());
/// assert!("/some/path".has_component("so").not());
///
/// assert!("/some/path".contains("some/pa"));
/// assert!(Path::new("/some/path").contains("some/pa"));
///
/// assert!("/this/and/that/".starts_or_ends_with("/"));
/// assert!(Path::new("/this/and/that/").starts_or_ends_with("/"));
///
/// assert!("multiple-extensions.tar.gz".strip_extensions() == Some("multiple-extensions"));
/// assert!(Path::new("multiple-extensions.tar.gz").strip_extensions() == Some("multiple-extensions"));
///
/// assert!("archive.tar.gz".ends_with_extensions(".tar.gz"));
/// assert!(Path::new("archive.tar.gz").ends_with_extensions("tar.gz"));
/// assert!(Path::new("archive.tar.gz").ends_with_extensions("z")); // it will match any part of the end of the extension
///
/// // Unlike std
/// assert!("archive.tar.gz".ends_with(".tar.gz"));
/// assert!(Path::new("archive.tar.gz").ends_with(".tar.gz").not());
/// assert!(Path::new("archive.tar.gz").ends_with("z").not());
/// ```
pub trait PathExt {
    /// Checks if the contained pattern is in the stringified version of the AsRef<Path>
    fn contains<S: AsRef<str>>(&self, pattern: S) -> bool;
    /// This function was created due to the following expectation breaking pattern in std:
    /// `assert!("archive.tar.gz".ends_with(".tar.gz"));`
    /// `assert!(Path::new("archive.tar.gz").ends_with(".tar.gz").not());`
    ///
    /// So, instead you can:
    /// `assert!("archive.tar.gz".ends_with_extensions(".tar.gz"));`
    /// `assert!(Path::new("archive.tar.gz").ends_with_extensions(".tar.gz"));`
    ///
    /// Note that the pattern can match a parital extension as long as it ENDS the path.
    /// `assert!(Path::new("archive.tar.gz").ends_with_extensions("z"));` is valid.
    fn ends_with_extensions<S: AsRef<str>>(&self, pattern: S) -> bool;
    /// Checks if the supplied component is present in total in the path
    fn has_component<S: AsRef<str>>(&self, component: S) -> bool;
    /// Checks if the supplied pattern is at the beginning or end of the stringified version of the AsRef<Path>
    fn starts_or_ends_with<S: AsRef<str>>(&self, pattern: S) -> bool;
    /// Strips all extensions from a pathref. If the path isn't able to be converted to a `str` return `None` instead
    fn strip_extensions(&self) -> Option<&str>;
    /// Strip the prefix if it's there
    fn strip_prefix_if_needed<'a, S: AsRef<str>>(&'a self, prefix: S) -> &'a Path;
}

/// I think this is the only implementation needed since there is a lot that implements AsRef<Path> in std.
impl<T: AsRef<Path>> PathExt for T {
    fn contains<S: AsRef<str>>(&self, pattern: S) -> bool {
        self.as_ref()
            .to_str()
            .map_or(false, |s| s.contains(pattern.as_ref()))
    }

    fn has_component<S: AsRef<str>>(&self, component: S) -> bool {
        self.as_ref()
            .components()
            .any(|c| c.as_os_str().eq(component.as_ref()))
    }

    fn starts_or_ends_with<S: AsRef<str>>(&self, pattern: S) -> bool {
        self.as_ref().to_str().map_or(false, |s| {
            s.starts_with(pattern.as_ref()) || s.ends_with(pattern.as_ref())
        })
    }

    fn ends_with_extensions<S: AsRef<str>>(&self, pattern: S) -> bool {
        self.as_ref()
            .to_str()
            .map_or(false, |s| s.ends_with(pattern.as_ref()))
    }

    fn strip_extensions(&self) -> Option<&str> {
        if let Some(path) = self.as_ref().to_str() {
            if let Some((base, ..)) = path.split_once('.') {
                Some(base)
            } else {
                Some(path)
            }
        } else {
            None
        }
    }

    fn strip_prefix_if_needed<'a, S: AsRef<str>>(&'a self, prefix: S) -> &'a Path {
        if let Ok(stripped) = self.as_ref().strip_prefix(prefix.as_ref()) {
            stripped
        } else {
            self.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PathExt;

    use std::ffi::OsStr;
    use std::ops::Not;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_ends_with_extensions() {
        let archive_path = Path::new("archive.tar.gz");
        assert!(archive_path.ends_with_extensions(".tar.gz"));
        assert!(archive_path.ends_with_extensions("tar.gz"));
        assert!(archive_path.ends_with_extensions(".gz"));
        assert!(archive_path.ends_with_extensions("z"));
        assert!(archive_path.ends_with_extensions("archive.tar.gz"));
    }

    // from a playground link I made https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c3d8a15324eeb911795bf5ac40bd2429
    #[test]
    fn is_ends_with_still_ignoring_extensions() {
        // This doesn't make sense to me. Hence the crate.
        let archive_path = Path::new("archive.tar.gz");
        assert!(archive_path.ends_with(".tar.gz").not());
        assert!(archive_path.ends_with(OsStr::new(".tar.gz")).not());
        assert!(archive_path.extension() == Some(OsStr::new("gz")));
        assert!(archive_path.extension() != Some(OsStr::new("tar.gz")));
    }

    #[test]
    fn test_strip_extensions() {
        let tests = &[
            (".stuff", Some("")),
            ("something.tar.gz", Some("something")),
            ("areally-cool.attempt.js", Some("areally-cool")),
            ("lastdot.", Some("lastdot")),
        ];

        for test_case in tests {
            assert_eq!(test_case.0.strip_extensions(), test_case.1);
        }
    }

    #[test]
    fn test_contains() {
        let tests = &[(
            "/opt/somewhere/someplace/somehow/",
            vec![
                ("opt", true),
                ("/opt", true),
                ("somewhere", true),
                ("/someplace/somehow", true),
                ("someplace/somehow/", true),
                ("root", false),
            ],
        )];

        for test_case in tests {
            for test in test_case.1.iter() {
                assert_eq!(test_case.0.contains(test.0), test.1);
                let p = Path::new(test_case.0);
                assert_eq!(p.contains(test.0), test.1);
                let pb = PathBuf::from(test_case.0);
                assert_eq!(pb.contains(test.0), test.1);
            }
        }
    }

    #[test]
    fn test_has_component() {
        let tests = &[(
            "/opt/somewhere/someplace/",
            vec![
                ("opt", true),
                ("somewhere", true),
                ("someplace/", false),
                ("root", false),
            ],
        )];

        for test_case in tests {
            for test in test_case.1.iter() {
                assert_eq!(test_case.0.has_component(test.0), test.1);
                let p = Path::new(test_case.0);
                assert_eq!(p.has_component(test.0), test.1);
                let pb = PathBuf::from(test_case.0);
                assert_eq!(pb.has_component(test.0), test.1);
            }
        }
    }

    #[test]
    fn test_starts_or_ends_with() {
        let tests = &[(
            "/opt/somewhere/someplace/somehow/",
            vec![
                ("opt", false),
                ("/opt", true),
                ("somewhere", false),
                ("someplace/somehow", false),
                ("someplace/somehow/", true),
                ("root", false),
            ],
        )];

        for test_case in tests {
            for test in test_case.1.iter() {
                assert_eq!(test_case.0.starts_or_ends_with(test.0), test.1);
                let p = Path::new(test_case.0);
                assert_eq!(p.starts_or_ends_with(test.0), test.1);
                let pb = PathBuf::from(test_case.0);
                assert_eq!(pb.starts_or_ends_with(test.0), test.1);
            }
        }
    }

    #[test]
    fn test_strip_prefix_if_needed() {
        let tests = &[(
            "/usr/local/aardvark",
            vec![
                ("/usr/local", "aardvark"),
                ("/usr/", "local/aardvark"),
                ("/repos/local/", "/usr/local/aardvark"),
            ],
        )];

        for test_case in tests {
            for test in test_case.1.iter() {
                let expected = Path::new(test.1);
                assert_eq!(test_case.0.strip_prefix_if_needed(test.0), expected);
                let p = Path::new(test_case.0);
                assert_eq!(p.strip_prefix_if_needed(test.0), expected);
                let pb = PathBuf::from(test_case.0);
                assert_eq!(pb.strip_prefix_if_needed(test.0), expected);
            }
        }
    }
}
