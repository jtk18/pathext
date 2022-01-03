//! # pathext
//!
//! A simple extension trait that includes some convenience methods I have found useful.
//!

use std::path::Path;

/// ```rust
/// use pathext::PathExt;
///
/// assert!("/some/path".has_component("path"));
/// assert!("/some/path".contains("some"));
/// assert!("/this/and/that/".starts_or_ends_with("/"));
/// ```
pub trait PathExt {
    /// Checks if the contained pattern is in the stringified version of the AsRef<Path>
    fn contains(&self, pattern: &str) -> bool;
    /// Checks if the supplied component is present in total in the path
    fn has_component(&self, component: &str) -> bool;
    /// Checks if the supplied pattern is at the beginning or end of the stringified version of the AsRef<Path>
    fn starts_or_ends_with(&self, pattern: &str) -> bool;
}

/// I think this is the only implementation needed since there is a lot that implements AsRef<Path> in std.
impl<T: AsRef<Path>> PathExt for T {
    fn contains(&self, pattern: &str) -> bool {
        self.as_ref().to_str().map(|s| s.contains(pattern)) == Some(true)
    }

    fn has_component(&self, component: &str) -> bool {
        self.as_ref()
            .components()
            .any(|c| c.as_os_str().eq(component))
    }

    fn starts_or_ends_with(&self, pattern: &str) -> bool {
        self.as_ref()
            .to_str()
            .map(|s| s.starts_with(pattern) || s.ends_with(pattern))
            == Some(true)
    }
}

#[cfg(test)]
mod tests {
    use super::PathExt;
    use std::path::{Path, PathBuf};

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
}
