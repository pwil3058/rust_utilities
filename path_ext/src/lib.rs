// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std;
use std::env;
use std::path::{self, Component, Path, PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Current directory not found")]
    CurrDirNotFound(#[from] std::io::Error),
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("Failed to strip path prefix")]
    StripPrefixError(#[from] path::StripPrefixError),
}

#[cfg(test)]
impl PartialEq for PathError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::CurrDirNotFound(_) => matches!(other, Self::CurrDirNotFound(_)),
            Self::HomeDirNotFound => matches!(other, Self::HomeDirNotFound),
            Self::StripPrefixError(_) => matches!(other, Self::StripPrefixError(_)),
        }
    }
}

pub fn absolute_path_buf(path: impl AsRef<Path>) -> Result<PathBuf, PathError> {
    if path.as_ref().is_absolute() {
        Ok(path.as_ref().to_path_buf())
    } else if path.as_ref().starts_with("~/") {
        let home_dir_path = dirs::home_dir().ok_or(PathError::HomeDirNotFound)?;
        let tail = path.as_ref().strip_prefix("~/")?;
        Ok(home_dir_path.join(tail))
    } else {
        let mut current_dir = env::current_dir()?;
        let mut components = path.as_ref().components();
        if let Some(mut first_component) = components.next() {
            match first_component {
                Component::Prefix(_) | Component::RootDir => {
                    unreachable!()
                }
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
                    Ok(current_dir.join(components.as_path()))
                }
                Component::CurDir => Ok(current_dir.join(components.as_path())),
                Component::Normal(_) => Ok(current_dir.join(path.as_ref())),
            }
        } else {
            Ok(current_dir.to_path_buf())
        }
    }
}

pub fn relative_path_buf(path: impl AsRef<Path>) -> Result<PathBuf, PathError> {
    let absolute_path = absolute_path_buf(path)?;
    let cur_dir = env::current_dir()?;
    Ok(absolute_path.strip_prefix(&cur_dir)?.to_path_buf())
}

pub trait PathExt {
    fn absolute_path_buf(&self) -> Result<PathBuf, PathError>;
    fn relative_path_buf(&self) -> Result<PathBuf, PathError>;
}

impl PathExt for Path {
    fn absolute_path_buf(&self) -> Result<PathBuf, PathError> {
        absolute_path_buf(self)
    }

    fn relative_path_buf(&self) -> Result<PathBuf, PathError> {
        relative_path_buf(self)
    }
}

impl PathExt for PathBuf {
    fn absolute_path_buf(&self) -> Result<PathBuf, PathError> {
        absolute_path_buf(self)
    }

    fn relative_path_buf(&self) -> Result<PathBuf, PathError> {
        relative_path_buf(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_absolute_path_buf_works() {
        let path = Path::new("/foo/bar");
        assert_eq!(path.absolute_path_buf(), Ok(PathBuf::from("/foo/bar")));
    }

    #[test]
    fn current_dir_absolute_path_buf_works() {
        let path = Path::new("./foo/bar");
        let current_dir = env::current_dir().unwrap();
        let expected = current_dir.join(path);
        assert_eq!(path.absolute_path_buf(), Ok(expected));
        let path = Path::new("foo/bar");
        let current_dir = env::current_dir().unwrap();
        let expected = current_dir.join(path);
        assert_eq!(path.absolute_path_buf(), Ok(expected));
    }

    #[test]
    fn parent_dir_absolute_path_buf_works() {
        let path = Path::new("../foo/bar");
        let current_dir = env::current_dir().unwrap();
        let parent_dir = current_dir.parent().unwrap();
        let expected = parent_dir.join(Path::new("foo/bar"));
        assert_eq!(path.absolute_path_buf(), Ok(expected));
    }

    #[test]
    fn parent_dirs_absolute_path_buf_works() {
        let path = Path::new("../../foo/bar");
        let current_dir = env::current_dir().unwrap();
        let parent_dir = current_dir.parent().unwrap();
        let parent_dir = parent_dir.parent().unwrap();
        let expected = parent_dir.join(Path::new("foo/bar"));
        assert_eq!(path.absolute_path_buf(), Ok(expected));
    }

    #[test]
    fn home_dir_absolute_path_buf_works() {
        let path = Path::new("~/foo/bar");
        let home_dir = env::home_dir().unwrap();
        let expected = home_dir.join(Path::new("foo/bar"));
        assert_eq!(path.absolute_path_buf(), Ok(expected));
    }

    #[test]
    fn simple_relative_path_buf_works() {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(Path::new("foo/bar"));
        assert_eq!(path.relative_path_buf(), Ok(PathBuf::from("foo/bar")));
        assert!(path.relative_path_buf().unwrap().is_relative());
        let path = Path::new("./foo/bar");
        assert_eq!(path.relative_path_buf(), Ok(PathBuf::from("foo/bar")));
        let path = Path::new("foo/bar");
        assert_eq!(path.relative_path_buf(), Ok(PathBuf::from("foo/bar")));
        let path = Path::new("/foo/bar");
        assert!(path.relative_path_buf().is_err());
        let path = Path::new("~/foo/bar");
        assert!(path.relative_path_buf().is_err());
    }
}
