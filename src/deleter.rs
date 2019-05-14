use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct Deleter<'a> {
    path: &'a Path,
    exts: Vec<&'a str>,
    filenames: Vec<&'a str>,
    folders: Vec<&'a str>,
    total_files_removed: &'a mut u32,
}

impl<'a> Deleter<'a> {
    pub fn new(
        path: &'a Path,
        exts: Vec<&'a str>,
        filenames: Vec<&'a str>,
        folders: Vec<&'a str>,
        total_files_removed: &'a mut u32,
    ) -> Deleter<'a> {
        Deleter {
            path,
            exts,
            filenames,
            folders,
            total_files_removed,
        }
    }

    pub fn show_results(&self) {
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

            if !self.folders.is_empty() {
                for folder in self.folders.iter() {
                    let path_string = String::from(entry.path().to_string_lossy());
                    let paths: Vec<&str> = path_string.split("/").collect();
                    if paths[paths.len() - 1] == *folder {
                        fs::remove_dir_all(entry.path()).expect("Error removing folder");
                    }
                }
            }

            if !self.filenames.is_empty() {
                for fname in self.filenames.iter() {
                    let filename = OsStr::new(fname);
                    if entry.file_name() == filename {
                        *self.total_files_removed += 1;
                        fs::remove_file(entry.path()).expect("Error removing file!");
                    }
                }
            }

            if !self.exts.is_empty() {
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
}