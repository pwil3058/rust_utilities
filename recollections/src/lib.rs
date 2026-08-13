// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Provide a mechanism to remember named strings
//! from one session to the next.
//! E.g. GUI widget data (size, position, etc)

mod recollect;

use std::path;

use lazy_static::lazy_static;
use mut_static::*;

use recollect::*;

lazy_static! {
    static ref RECOLLECTIONS: MutStatic<Recollections> = MutStatic::from(Recollections::default());
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
///     recollections::init(&home_dir.join(".this_apps_recollections"));
/// }
/// ```
///
/// If this initialisation is not performed then calls to `recall()`
/// will return `None`, calls to `recall_or_else()` will return the
/// default supplied and calls to `remember()` will be ignored.
/// The operation of the application will not be effected otherwise.
pub fn init<P: AsRef<path::Path>>(file_path: P) {
    let file_path: &path::Path = file_path.as_ref();
    RECOLLECTIONS.write().unwrap().set_data_file_path(file_path);
}

/// Remember the string specified by `value` and associate it with
/// the given `name` for later recall.
pub fn remember(name: &str, value: &str) {
    RECOLLECTIONS.read().unwrap().remember(name, value)
}

#[cfg(test)]
mod recollections_tests {
    use crate::{RECOLLECTIONS, init};
    use std::{fs, path};

    #[test]
    fn test_init() {
        let file_path = path::Path::new("testing/testing");
        init(file_path);
        assert!(file_path.exists());
        RECOLLECTIONS.write().unwrap().set_data_file_path(file_path);
        assert!(file_path.exists());
        assert!(fs::remove_file(file_path).is_ok());
    }
}
