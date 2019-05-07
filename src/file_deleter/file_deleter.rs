use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct FileDeleter<'a> {
    path: &'a Path,
    exts: Vec<&'a str>,
    total_files_removed: &'a mut u32,
}

impl<'a> FileDeleter<'a> {
    pub fn new(
        path: &'a Path,
        exts: Vec<&'a str>,
        total_files_removed: &'a mut u32,
    ) -> FileDeleter<'a> {
        FileDeleter {
            path,
            exts,
            total_files_removed,
        }
    }

    pub fn show_completion_output(&self) {
        if *self.total_files_removed > 0 {
            println!("Finished deleting files");
            println!("A total of: {} files deleted", self.total_files_removed);
        } else {
            println!("No files found!");
        }
    }

    pub fn delete_files(&mut self) {
        for entry in WalkDir::new(self.path).into_iter().filter_map(|e| e.ok()) {
            let extension = entry.path().extension();

            for ext in self.exts.iter() {
                let ex = Some(OsStr::new(ext));
                if ex == extension {
                    *self.total_files_removed += 1;
                    fs::remove_file(entry.path()).expect("Error removing file!");
                }
            }
        }
    }
}
