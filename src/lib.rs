use std::path::Path;

pub trait PathExt {
    fn starts_or_ends_with(&self, pattern: &str) -> bool;
    fn has_component(&self, component: &str) -> bool;
}

impl<T: AsRef<Path>> PathExt for T {
    fn starts_or_ends_with(&self, pattern: &str) -> bool {
        self.as_ref()
            .to_str()
            .map(|s| s.starts_with(pattern) || s.ends_with(pattern))
            == Some(true)
    }

    fn has_component(&self, component: &str) -> bool {
        self.as_ref()
            .components()
            .any(|c| c.as_os_str().eq(component))
    }
}

#[cfg(test)]
mod tests {
    use super::PathExt;
    use std::path::{Path, PathBuf};

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
