// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Provide a mechanism to remember named strings
//! from one session to the next.
//! E.g. GUI widget data (size, position, etc)

mod recollect;

use std::path;

use lazy_static::lazy_static;
use mut_static::MutStatic;
use thiserror::Error;

use recollect::*;

lazy_static! {
    static ref RECOLLECTIONS: MutStatic<Recollections> = MutStatic::from(Recollections::default());
}

#[derive(Error, Debug)]
pub enum RecollectError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("File path error: {0}")]
    FilePathError(#[from] path_utilities::PathExtError),
    #[error("JSON error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Initialise the mechanism by providing the path of the file
/// where the data should be stored.  This would normally be a
/// hidden file in the user's home directory or a hidden configuration
/// directory for the application.
///
/// This function should normally be called early in the application's
/// `main()` function e.g.
///
/// ```no_run
/// fn main_() {
///     use dirs;
///     use recollections;
///
///     let home_dir = dirs::home_dir().expect("badly designed OS");
///     recollections::init(&home_dir.join(".this_apps_recollections")).unwrap();
/// }
/// ```
///
/// If this initialisation is not performed then calls to `recall()`
/// will return `None`, calls to `recall_or_else()` will return the
/// default supplied and calls to `remember()` will be ignored.
/// The operation of the application will not be effected otherwise.
pub fn init<P: AsRef<path::Path>>(file_path: P) -> Result<(), RecollectError> {
    let file_path: &path::Path = file_path.as_ref();
    RECOLLECTIONS
        .write()
        .unwrap()
        .set_data_file_path(file_path)?;
    Ok(())
}

/// Remember the string specified by `value` and associate it with
/// the given `name` for later recall.
pub fn remember(name: &str, value: &str) {
    RECOLLECTIONS.read().unwrap().remember(name, value)
}

/// Return the `String` value associated with the given `name` or
/// `None` if `recollections` has not been initialised or
/// asked remember data associated with the given `name`.
pub fn recall(name: &str) -> Option<String> {
    RECOLLECTIONS.read().unwrap().recall(name)
}

/// Return the `String` value associated with the given `name` or
/// `default` if `recollections` has not been initialised or
/// asked remember data associated with the given `name`.
pub fn recall_or_else(name: &str, default: &str) -> String {
    RECOLLECTIONS.read().unwrap().recall_or_else(name, default)
}

#[cfg(test)]
mod recollections_tests {
    use crate::{RECOLLECTIONS, init, recall, recall_or_else, remember};
    use std::{fs, path};

    #[test]
    fn test_init() {
        let file_path = path::Path::new("testing/testing");
        init(file_path).unwrap();
        assert!(file_path.exists());
        RECOLLECTIONS
            .write()
            .unwrap()
            .set_data_file_path(file_path)
            .unwrap();
        assert!(file_path.exists());
        assert!(fs::remove_file(file_path).is_ok());
    }

    #[test]
    fn recollect_test() {
        let recollection_file = path::Path::new("recollection_test");
        init(recollection_file).unwrap();
        assert_eq!(recall("anything"), None);
        assert_eq!(recall_or_else("anything", "but"), "but");
        remember("anything", "whatever");
        assert_eq!(recall("anything"), Some("whatever".to_string()));
        assert_eq!(recall_or_else("anything", "but"), "whatever");
        assert!(fs::remove_file(recollection_file).is_ok());
    }
}
