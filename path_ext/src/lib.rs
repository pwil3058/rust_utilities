// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::env;
use std::path::{Component, Path, PathBuf};

pub trait PathExt {
    fn absolute_path_buf(&self) -> Option<PathBuf>;
}

impl PathExt for Path {
    fn absolute_path_buf(&self) -> Option<PathBuf> {
        if self.is_absolute() {
            Some(self.to_path_buf())
        } else if self.starts_with("~/") {
            let home_dir_path = dirs::home_dir()?;
            if let Ok(tail) = self.strip_prefix("~/") {
                Some(home_dir_path.join(tail))
            } else {
                None
            }
        } else if let Ok(mut current_dir) = env::current_dir() {
            let mut components = self.components();
            if let Some(mut first_component) = components.next() {
                match first_component {
                    Component::RootDir | Component::Prefix(_) => Some(self.to_path_buf()),
                    Component::ParentDir => {
                        while first_component == Component::ParentDir {
                            debug_assert!(current_dir.pop());
                            if let Some(component) = components.next() {
                                first_component = component;
                            } else {
                                break;
                            }
                        }
                        current_dir.push(first_component);
                        Some(current_dir.join(components.as_path()))
                    }
                    Component::CurDir => Some(current_dir.join(components.as_path())),
                    Component::Normal(_) => Some(current_dir.join(self)),
                }
            } else {
                Some(current_dir)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_absolute_path_buf_works() {
        let path = Path::new("/foo/bar");
        assert_eq!(path.absolute_path_buf(), Some(PathBuf::from("/foo/bar")));
    }

    #[test]
    fn current_dir_absolute_path_buf_works() {
        let path = Path::new("./foo/bar");
        let current_dir = env::current_dir().unwrap();
        let expected = current_dir.join(path);
        assert_eq!(path.absolute_path_buf(), Some(expected));
        let path = Path::new("foo/bar");
        let current_dir = env::current_dir().unwrap();
        let expected = current_dir.join(path);
        assert_eq!(path.absolute_path_buf(), Some(expected));
    }

    #[test]
    fn parent_dir_absolute_path_buf_works() {
        let path = Path::new("../foo/bar");
        let current_dir = env::current_dir().unwrap();
        let parent_dir = current_dir.parent().unwrap();
        let expected = parent_dir.join(Path::new("foo/bar"));
        assert_eq!(path.absolute_path_buf(), Some(expected));
    }

    #[test]
    fn parent_dirs_absolute_path_buf_works() {
        let path = Path::new("../../foo/bar");
        let current_dir = env::current_dir().unwrap();
        let parent_dir = current_dir.parent().unwrap();
        let parent_dir = parent_dir.parent().unwrap();
        let expected = parent_dir.join(Path::new("foo/bar"));
        assert_eq!(path.absolute_path_buf(), Some(expected));
    }

    #[test]
    fn home_dir_absolute_path_buf_works() {
        let path = Path::new("~/foo/bar");
        let home_dir = env::home_dir().unwrap();
        let expected = home_dir.join(Path::new("foo/bar"));
        assert_eq!(path.absolute_path_buf(), Some(expected));
    }
}
