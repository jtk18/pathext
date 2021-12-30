use std::path::{Path, PathBuf};

pub trait PathExt {
    fn contains_or_ends_with(&self, pattern: &str) -> bool;
    fn contains_or_starts_with(&self, pattern: &str) -> bool;
    fn starts_or_ends_with(&self, pattern: &str) -> bool;
    fn has_component(&self, component: &str) -> bool;
}

impl<T: AsRef<Path>> PathExt for T {
    fn contains_or_ends_with(&self, pattern: &str) -> bool {
        self.as_ref().to_str().map(|s| s.contains(pattern) || s.ends_with(pattern)) == Some(true)
    }

    fn contains_or_starts_with(&self, pattern: &str) -> bool {
        self.as_ref().to_str().map(|s| s.contains(pattern) || s.starts_with(pattern)) == Some(true)
    }

    fn starts_or_ends_with(&self, pattern: &str) -> bool {
        self.as_ref().to_str().map(|s| s.starts_with(pattern) || s.ends_with(pattern)) == Some(true)
    }

    fn has_component(&self, component: &str) -> bool {
        self.as_ref().components().any(|c| c.as_os_str().eq(component))
    }
}