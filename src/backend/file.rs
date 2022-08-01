use std::{error, fs};
use std::io::Error;
use std::path::Path;

use crate::backend::base::{Backend, EXTENSIONS};

#[derive(Clone)]
pub struct FileBackend {
    path: String,
}

impl Backend for FileBackend {
    fn list_files(&self) -> Result<Vec<String>, std::io::Error> {
        // Read all file entries from the pictures path.
        let all_entries = fs::read_dir(self.path.as_str());
        if all_entries.is_err() {
            return Err(all_entries.err().unwrap());
        }
        let all_entries = all_entries.unwrap();

        // Filter to contain only files with extensions contained in EXTENSIONS.
        let fm_entries = all_entries.filter_map(|p| {
            let entry = p.as_ref().unwrap();
            let path = entry.path();
            let ext = path.extension();
            if ext.is_none() {
                return None;
            }
            let ext_str = ext.unwrap().to_str().unwrap();
            let is_valid_ext = EXTENSIONS.contains(&ext_str);
            let allow = p.is_ok() && entry.file_type().unwrap().is_file() && is_valid_ext;

            return if allow {
                // TODO: Better error handling.
                let path = p.unwrap().path();
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                Some(file_name)
            } else {
                None
            };
        });

        // Collect all of the files.
        let entries: Vec<String> = fm_entries.collect();
        return Ok(entries);
    }

    fn get_file_contents(&self, file: &str) -> Result<Vec<u8>, Error> {
        let base = Path::new(&self.path);
        let file = Path::new(file);
        let path = base.join(file);
        return fs::read(path);
    }
}

pub fn create(path: String) -> Result<Box<dyn Backend>, Box<dyn error::Error>> {
    return Ok(Box::new(FileBackend {
        path
    }));
}
