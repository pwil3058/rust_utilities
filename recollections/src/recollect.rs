// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::collections::HashMap;
use std::io::Seek;
use std::{fs, io, path};

use path_utilities::*;

use crate::RecollectError;

type RecollectionDb = HashMap<String, String>;

#[derive(Debug, Default, Clone)]
pub struct Recollections {
    pub file_path: Option<path::PathBuf>,
}

impl Recollections {
    pub fn set_data_file_path(
        &mut self,
        file_path: impl AsRef<path::Path>,
    ) -> Result<(), RecollectError> {
        let file_path = file_path.as_ref().absolute_path_buf()?;
        if !file_path.exists() {
            if let Some(dir_path) = file_path.parent()
                && !dir_path.exists()
            {
                fs::create_dir_all(dir_path)?;
            }
            let mut file = fs::File::create(&file_path)?;
            serde_json::to_writer(&mut file, &RecollectionDb::new())?;
        };
        self.file_path = Some(file_path);
        Ok(())
    }

    pub fn remember(&self, name: &str, value: &str) {
        if let Some(ref file_path) = self.file_path {
            debug_assert!(
                file_path.exists(),
                "{}",
                format!(
                    "Recollections file {:?} seems to have gone away",
                    file_path.display()
                )
            );
            let mut file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(file_path)
                .expect("Could not open recollections data file");
            file.lock().expect("Could not lock recollections data file");
            let mut hash_map: RecollectionDb =
                serde_json::from_reader(&file).expect("Could not read recollections data file");
            hash_map.insert(name.to_string(), value.to_string());
            file.seek(io::SeekFrom::Start(0))
                .expect("Could not seek recollections data file");
            file.set_len(0)
                .expect("Could not seek recollections data file");
            serde_json::to_writer(&mut file, &hash_map)
                .expect("Could not write recollections data file");
            file.unlock()
                .expect("Could not unlock recollections data file");
        }
    }

    pub fn recall(&self, name: &str) -> Option<String> {
        if let Some(ref file_path) = self.file_path {
            let file = fs::File::open(file_path).expect("Could not open recollections data file");
            file.lock_shared()
                .expect("Could not lock recollections data file");
            let hash_map: RecollectionDb =
                serde_json::from_reader(&file).expect("Could not read recollections data file");
            file.unlock()
                .expect("Could not unlock recollections data file");
            hash_map.get(name).map(|s| s.to_string())
        } else {
            None
        }
    }

    pub fn recall_or_else(&self, name: &str, default: &str) -> String {
        match self.recall(name) {
            Some(string) => string,
            None => default.to_string(),
        }
    }
}
