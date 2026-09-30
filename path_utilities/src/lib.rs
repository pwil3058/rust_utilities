// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs::{DirEntry, FileType, Metadata, ReadDir};
use std::path::{self, Component, Path, PathBuf};
use std::{env, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Current directory not found")]
    CurrDirNotFound(#[from] std::io::Error),
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("Could not find current directory's parent.")]
    ParentDirNotFound,
    #[error("Failed to strip path prefix")]
    StripPrefixError(#[from] path::StripPrefixError),
    #[error("Unexpected prefix for this operation.")]
    UnexpectedPrefix,
}

#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::CurrDirNotFound(_) => matches!(other, Self::CurrDirNotFound(_)),
            Self::HomeDirNotFound => matches!(other, Self::HomeDirNotFound),
            Self::ParentDirNotFound => matches!(other, Self::ParentDirNotFound),
            Self::StripPrefixError(_) => matches!(other, Self::StripPrefixError(_)),
            Self::UnexpectedPrefix => matches!(other, Self::UnexpectedPrefix),
        }
    }
}

pub trait ComponentIs {
    fn is_prefix(&self) -> bool;
    fn is_normal(&self) -> bool;
}

impl<'a> ComponentIs for Component<'a> {
    fn is_prefix(&self) -> bool {
        match self {
            Component::Prefix(_) => true,
            _ => false,
        }
    }

    fn is_normal(&self) -> bool {
        match self {
            Component::Normal(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PathType {
    Absolute,
    RelativeCurDir,
    RelativeCurDirImplicit,
    RelativeParentDirs,
    RelativeHomeDir,
    Empty,
}

impl PathType {
    pub fn of<P: AsRef<Path>>(path_arg: P) -> Self {
        let path = path_arg.as_ref();
        match path.components().next() {
            None => PathType::Empty,
            Some(component) => match component {
                Component::RootDir | Component::Prefix(_) => PathType::Absolute,
                Component::CurDir => PathType::RelativeCurDir,
                Component::ParentDir => PathType::RelativeParentDirs,
                Component::Normal(os_string) => {
                    if os_string == "~" {
                        PathType::RelativeHomeDir
                    } else {
                        PathType::RelativeCurDirImplicit
                    }
                }
            },
        }
    }
}

pub fn expand_current_dir<P: AsRef<Path>>(path_arg: P) -> Result<PathBuf, Error> {
    let path = path_arg.as_ref();
    if path.starts_with(Component::CurDir) {
        let cur_dir = env::current_dir()?;
        let path_tail = path.strip_prefix(Component::CurDir)?;
        Ok(cur_dir.join(path_tail))
    } else {
        Err(Error::UnexpectedPrefix)
    }
}

pub fn expand_parent_dirs<P: AsRef<Path>>(path_arg: P) -> Result<PathBuf, Error> {
    let path = path_arg.as_ref();
    let mut components = path.components();
    let mut parent_dir = env::current_dir()?;

    // Use a loop to consume sequential '..' components from the front
    while let Some(component) = components.next() {
        match component {
            Component::ParentDir => {
                parent_dir = match parent_dir.parent() {
                    Some(p) => p.to_path_buf(),
                    None => return Err(Error::ParentDirNotFound),
                };
            }
            _ => {
                // We hit a normal component or root! Reconstruct the rest of the path
                // by joining the component we just checked with everything left in the iterator.
                return Ok(parent_dir.join(component).join(components.as_path()));
            }
        }
    }

    // If the loop finished and only contained '..', just return the updated parent path
    Ok(parent_dir)
}

pub fn expand_home_dir<P: AsRef<Path>>(path_arg: P) -> Result<PathBuf, Error> {
    let path = path_arg.as_ref();
    let mut components = path.components();

    if let Some(Component::Normal(os_str)) = components.next() {
        if os_str == "~" {
            let home_dir = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
            return Ok(home_dir.join(components.as_path()));
        }
    }
    Err(Error::UnexpectedPrefix)
}

pub fn expand_home_dir_or_mine<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    expand_home_dir(path).unwrap_or(path.to_path_buf())
}

pub fn prepend_current_dir<P: AsRef<Path>>(path_arg: P) -> Result<PathBuf, Error> {
    let path = path_arg.as_ref();
    match path.components().next() {
        None => Ok(env::current_dir()?),
        Some(component) => match component {
            Component::Normal(os_string) => {
                if os_string == "~" {
                    Err(Error::UnexpectedPrefix)
                } else {
                    let cur_dir = env::current_dir()?;
                    Ok(cur_dir.join(path))
                }
            }
            _ => Err(Error::UnexpectedPrefix),
        },
    }
}

pub fn absolute_path_buf(path: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let path = path.as_ref();
    match PathType::of(path) {
        PathType::Absolute => Ok(path.to_path_buf()),
        PathType::RelativeCurDir => expand_current_dir(path),
        PathType::RelativeParentDirs => expand_parent_dirs(path),
        PathType::RelativeHomeDir => expand_home_dir(path),
        PathType::RelativeCurDirImplicit => prepend_current_dir(path),
        PathType::Empty => Ok(env::current_dir()?),
    }
}

pub fn relative_path_buf(path: impl AsRef<Path>) -> Result<PathBuf, Error> {
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

pub fn relative_path_buf_or_mine(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    relative_path_buf(path).unwrap_or(path.to_path_buf())
}

pub fn path_to_string(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    if let Some(path_str) = path.to_str() {
        path_str.to_string()
    } else {
        let string = path.to_string_lossy();
        log::warn!("Non UniCode file path: {string}");
        string.to_string()
    }
}

pub struct FilteredDirEntries(ReadDir, String);

impl Iterator for FilteredDirEntries {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(result) = self.0.next() {
                match result {
                    Ok(entry) => return Some(entry),
                    Err(err) => match err.kind() {
                        io::ErrorKind::NotFound => {
                            // Assume race condition amd ignore
                        }
                        io::ErrorKind::PermissionDenied => {
                            // benign so just log it in case someone cares
                            log::info!("{}: Permission denied for ReadDir::next()", self.1);
                        }
                        _ => log::warn!(
                            "{}: Unexpected error \"{err}\"  for ReadDir::next()",
                            self.1
                        ),
                    },
                }
            } else {
                return None;
            }
        }
    }
}

pub fn filtered_dir_entries(dir_path: impl AsRef<Path>) -> io::Result<FilteredDirEntries> {
    let dir_path_str = dir_path.as_ref().display().to_string();
    let read_dir = dir_path.as_ref().read_dir()?;
    Ok(FilteredDirEntries(read_dir, dir_path_str))
}

#[derive(Debug)]
pub struct UsableDirEntry {
    pub dir_entry: DirEntry,
    pub metadata: Metadata,
}

impl UsableDirEntry {
    pub fn path(&self) -> PathBuf {
        self.dir_entry.path()
    }

    pub fn file_name(&self) -> OsString {
        self.dir_entry.file_name()
    }

    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.metadata.is_file()
    }

    pub fn is_symlink(&self) -> bool {
        self.metadata.is_symlink()
    }

    pub fn file_type(&self) -> FileType {
        self.metadata.file_type()
    }
}

pub struct UsableDirEntries(FilteredDirEntries);

impl Iterator for UsableDirEntries {
    type Item = UsableDirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(dir_entry) = self.0.next() {
                match dir_entry.metadata() {
                    Ok(metadata) => {
                        return Some(UsableDirEntry {
                            dir_entry,
                            metadata,
                        });
                    }
                    Err(err) => match err.kind() {
                        io::ErrorKind::NotFound => {
                            //   We assume that "not found" is due to race condition and ignore it
                        }
                        io::ErrorKind::PermissionDenied => {
                            //  benign so just log it in case someone cares
                            log::info!(
                                "{}: {:?}: permission denied accessing metadata",
                                self.0.1,
                                dir_entry.path()
                            )
                        }
                        _ => log::warn!(
                            "{}: {:?}: unexpected error \"{err}\" accessing metadata",
                            self.0.1,
                            dir_entry.path()
                        ),
                    },
                }
            } else {
                return None;
            }
        }
    }
}

pub fn usable_dir_entries(dir_path: impl AsRef<Path>) -> io::Result<UsableDirEntries> {
    Ok(UsableDirEntries(filtered_dir_entries(dir_path)?))
}

/// Recursively gathers all paths within a target workspace directory,
/// sorting and isolating them as strictly relative paths.
pub fn collect_relative_files(
    root_path: impl AsRef<Path>,
    excludes: &[String],
) -> io::Result<BTreeSet<PathBuf>> {
    let root = root_path.as_ref();
    let mut file_set = BTreeSet::new();
    let mut dirs_to_visit = vec![root.to_path_buf()];

    while let Some(current_dir) = dirs_to_visit.pop() {
        for entry in current_dir.usable_dir_entries()? {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if excludes.iter().any(|pattern| file_name_str == *pattern) {
                continue;
            }

            if entry.is_dir() {
                dirs_to_visit.push(entry.path());
            } else if entry.is_file()
                && let Ok(relative_path) = entry.path().strip_prefix(root)
            {
                file_set.insert(relative_path.to_path_buf());
            }
        }
    }

    Ok(file_set)
}

pub trait UsefulPathMethods {
    fn absolute_path_buf(&self) -> Result<PathBuf, Error>;
    fn relative_path_buf(&self) -> Result<PathBuf, Error>;
    fn usable_dir_entries(&self) -> io::Result<UsableDirEntries>;
    fn filtered_dir_entries(&self) -> io::Result<FilteredDirEntries>;
    fn collect_relative_files(&self, excludes: &[String]) -> io::Result<BTreeSet<PathBuf>>;
}

impl UsefulPathMethods for Path {
    fn absolute_path_buf(&self) -> Result<PathBuf, Error> {
        absolute_path_buf(self)
    }

    fn relative_path_buf(&self) -> Result<PathBuf, Error> {
        relative_path_buf(self)
    }

    fn usable_dir_entries(&self) -> io::Result<UsableDirEntries> {
        usable_dir_entries(self)
    }

    fn filtered_dir_entries(&self) -> io::Result<FilteredDirEntries> {
        filtered_dir_entries(self)
    }

    fn collect_relative_files(&self, excludes: &[String]) -> io::Result<BTreeSet<PathBuf>> {
        collect_relative_files(self, excludes)
    }
}

impl UsefulPathMethods for PathBuf {
    fn absolute_path_buf(&self) -> Result<PathBuf, Error> {
        absolute_path_buf(self)
    }

    fn relative_path_buf(&self) -> Result<PathBuf, Error> {
        relative_path_buf(self)
    }

    fn usable_dir_entries(&self) -> io::Result<UsableDirEntries> {
        usable_dir_entries(self)
    }

    fn filtered_dir_entries(&self) -> io::Result<FilteredDirEntries> {
        filtered_dir_entries(self)
    }

    fn collect_relative_files(&self, excludes: &[String]) -> io::Result<BTreeSet<PathBuf>> {
        collect_relative_files(self, excludes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
    fn test_collect_relative_files_recursively() {
        use std::fs::{self, File};
        use tempfile::TempDir;

        // 1. Setup a temporary workspace directory tree
        let tmp_dir = TempDir::new().unwrap();
        let root = tmp_dir.path();

        // 2. Build structured files at varying directory nesting depths
        let file1 = "src/main.rs";
        let file2 = "src/utils/math.rs";
        let file3 = "docs/readme.md";
        let file4 = "root_file.txt";

        let paths_to_create = [file1, file2, file3, file4];
        for rel_path in &paths_to_create {
            let full_path = root.join(rel_path);
            // Ensure parent subdirectories are dynamically spun up
            fs::create_dir_all(full_path.parent().unwrap()).unwrap();
            File::create(full_path).unwrap();
        }

        // Create an empty directory variant to ensure it is cleanly bypassed
        fs::create_dir_all(root.join("empty_dir_to_ignore")).unwrap();

        // 3. Execute your path_utilities trait method
        let relative_files = root.collect_relative_files(&[]).unwrap();

        // 4. Assertions
        assert_eq!(relative_files.len(), 4);

        // BTreeSet enforces total alphabetical order; verify index sequencing
        let files_vec: Vec<PathBuf> = relative_files.into_iter().collect();
        assert_eq!(files_vec[0], PathBuf::from("docs/readme.md"));
        assert_eq!(files_vec[1], PathBuf::from("root_file.txt"));
        assert_eq!(files_vec[2], PathBuf::from("src/main.rs"));
        assert_eq!(files_vec[3], PathBuf::from("src/utils/math.rs"));

        // Confirm all output records are clean, un-prefixed relative paths
        for path in &files_vec {
            assert!(path.is_relative());
            assert!(!path.starts_with(root));
        }
    }

    #[test]
    fn test_usable_dir_entries_agree() {
        let current_dir = env::current_dir().unwrap();

        let usable_names: HashSet<OsString> = usable_dir_entries(&current_dir)
            .unwrap()
            .map(|e| e.file_name())
            .collect();

        let filtered_names: HashSet<OsString> = filtered_dir_entries(&current_dir)
            .unwrap()
            .map(|e| e.file_name())
            .collect();

        assert_eq!(usable_names, filtered_names);
    }
}
