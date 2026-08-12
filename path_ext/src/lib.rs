// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::ffi::OsString;
use std::fs::{DirEntry, FileType, Metadata, ReadDir};
use std::path::{self, Component, Path, PathBuf};
use std::{env, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathExtError {
    #[error("Current directory not found")]
    CurrDirNotFound(#[from] std::io::Error),
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("Failed to strip path prefix")]
    StripPrefixError(#[from] path::StripPrefixError),
}

#[cfg(test)]
impl PartialEq for PathExtError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::CurrDirNotFound(_) => matches!(other, Self::CurrDirNotFound(_)),
            Self::HomeDirNotFound => matches!(other, Self::HomeDirNotFound),
            Self::StripPrefixError(_) => matches!(other, Self::StripPrefixError(_)),
        }
    }
}

pub fn absolute_path_buf(path: impl AsRef<Path>) -> Result<PathBuf, PathExtError> {
    if path.as_ref().is_absolute() {
        Ok(path.as_ref().to_path_buf())
    } else if path.as_ref().starts_with("~/") {
        let home_dir_path = dirs::home_dir().ok_or(PathExtError::HomeDirNotFound)?;
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

pub fn relative_path_buf(path: impl AsRef<Path>) -> Result<PathBuf, PathExtError> {
    let absolute_path = absolute_path_buf(&path)?;
    let mut cur_dir = env::current_dir()?;
    if absolute_path.starts_with(&cur_dir) {
        Ok(absolute_path.strip_prefix(&cur_dir)?.to_path_buf())
    } else {
        let mut path_buf = PathBuf::new();
        loop {
            path_buf.push("../");
            if cur_dir.pop() {
                if absolute_path.starts_with(&cur_dir) {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(path_buf.join(path.as_ref().strip_prefix(&cur_dir)?))
    }
}

#[derive(Debug)]
pub struct UsableDirEntry {
    dir_entry: DirEntry,
    file_type: FileType,
}

#[cfg(test)]
impl PartialEq for UsableDirEntry {
    fn eq(&self, other: &Self) -> bool {
        self.dir_entry.file_name() == other.dir_entry.file_name()
            && self.file_type == other.file_type
    }
}

impl UsableDirEntry {
    pub fn path(&self) -> PathBuf {
        self.dir_entry.path()
    }

    pub fn file_name(&self) -> OsString {
        self.dir_entry.file_name()
    }

    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.file_type.is_file()
    }

    pub fn is_symlink(&self) -> bool {
        self.file_type.is_symlink()
    }

    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    pub fn metadata(&self) -> io::Result<Metadata> {
        self.dir_entry.metadata()
    }
}

pub struct UsableDirEntryIter {
    read_dir: ReadDir,
}

impl Iterator for UsableDirEntryIter {
    type Item = UsableDirEntry;

    #[allow(clippy::while_let_on_iterator)]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.read_dir.next() {
            match result {
                Ok(dir_entry) => match dir_entry.metadata() {
                    Ok(metadata) => {
                        let file_type = metadata.file_type();
                        return Some(UsableDirEntry {
                            dir_entry,
                            file_type,
                        });
                    }
                    Err(err) => match err.kind() {
                        io::ErrorKind::NotFound => {
                            // We assume that "not found" is due to race condition and ignore it
                        }
                        io::ErrorKind::PermissionDenied => {
                            // benign so just log it in case someone cares
                            log::info!(
                                "{:?}: permission denied accessing metadata",
                                dir_entry.path()
                            )
                        }
                        _ => log::warn!(
                            "{:?}: unexpected error \"{err}\" accessing metadata",
                            dir_entry.path()
                        ),
                    },
                },
                Err(err) => match err.kind() {
                    io::ErrorKind::NotFound => {
                        // We assume that "not found" is due to race condition and ignore it
                    }
                    io::ErrorKind::PermissionDenied => {
                        // benign so just log it in case someone cares
                        log::info!("Permission denied for ReadDir::next()")
                    }
                    _ => log::warn!("Unexpected error \"{err}\"  for ReadDir::next()"),
                },
            }
        }
        None
    }
}

pub fn usable_dir_entries(dir_path: &Path) -> Result<UsableDirEntryIter, io::Error> {
    let read_dir = dir_path.read_dir()?;
    Ok(UsableDirEntryIter { read_dir })
}

pub fn filtered_dir_entries(
    dir_path: &Path,
) -> Result<impl Iterator<Item = UsableDirEntry>, io::Error> {
    let read_dir = dir_path.read_dir()?;
    Ok(read_dir.filter_map(|dir_entry| {
        match dir_entry {
            Ok(dir_entry) => match dir_entry.metadata() {
                Ok(metadata) => {
                    let file_type = metadata.file_type();
                    Some(UsableDirEntry {
                        dir_entry,
                        file_type,
                    })
                }
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::NotFound => {
                            // We assume that "not found" is due to race condition and ignore it
                        }
                        io::ErrorKind::PermissionDenied => {
                            // benign so just log it in case someone cares
                            log::info!(
                                "{:?}: permission denied accessing metadata",
                                dir_entry.path()
                            )
                        }
                        _ => log::warn!(
                            "{:?}: unexpected error \"{err}\" accessing metadata",
                            dir_entry.path()
                        ),
                    }
                    None
                }
            },
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::NotFound => {
                        // assume race condition and ignore
                    }
                    io::ErrorKind::PermissionDenied => {
                        // benign so just log it in case someone cares
                        log::info!("Permission denied for ReadDir::next()");
                    }
                    _ => log::warn!("Unexpected error \"{err}\"  for ReadDir::next()"),
                };
                None
            }
        }
    }))
}

pub trait PathExt {
    fn absolute_path_buf(&self) -> Result<PathBuf, PathExtError>;
    fn relative_path_buf(&self) -> Result<PathBuf, PathExtError>;
    fn usable_dir_entries(&self) -> Result<UsableDirEntryIter, io::Error>;
}

impl PathExt for Path {
    fn absolute_path_buf(&self) -> Result<PathBuf, PathExtError> {
        absolute_path_buf(self)
    }

    fn relative_path_buf(&self) -> Result<PathBuf, PathExtError> {
        relative_path_buf(self)
    }

    fn usable_dir_entries(&self) -> Result<UsableDirEntryIter, io::Error> {
        usable_dir_entries(self)
    }
}

impl PathExt for PathBuf {
    fn absolute_path_buf(&self) -> Result<PathBuf, PathExtError> {
        absolute_path_buf(self)
    }

    fn relative_path_buf(&self) -> Result<PathBuf, PathExtError> {
        relative_path_buf(self)
    }

    fn usable_dir_entries(&self) -> Result<UsableDirEntryIter, io::Error> {
        usable_dir_entries(self)
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
    }

    #[test]
    fn complex_relative_path_buf_works() {
        let mut current_dir = env::current_dir().unwrap();
        let path = current_dir.parent().unwrap().join(Path::new("foo/bar"));
        assert_eq!(path.relative_path_buf(), Ok(PathBuf::from("../foo/bar")));
        let mut expected_prefix = PathBuf::new();
        loop {
            if let Some(parent) = current_dir.parent() {
                let path = parent.join(Path::new("foo/bar"));
                expected_prefix.push("../");
                let expected = expected_prefix.join(PathBuf::from("foo/bar"));
                assert_eq!(path.relative_path_buf(), Ok(expected));
                assert!(path.relative_path_buf().unwrap().is_relative());
                assert_eq!(
                    path.relative_path_buf()
                        .unwrap()
                        .absolute_path_buf()
                        .unwrap(),
                    path
                );
                current_dir = parent.to_path_buf();
            } else {
                break;
            }
        }
    }

    #[test]
    fn test_usable_dir_entries_agree() {
        let current_dir = env::current_dir().unwrap();
        let usable_dir_entries = current_dir.usable_dir_entries().unwrap();
        let filtered_dir_entries = filtered_dir_entries(&current_dir).unwrap();
        assert!(
            usable_dir_entries
                .zip(filtered_dir_entries)
                .all(|(l, r)| l == r)
        );
    }
}
