use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
  
pub fn delete_files(
    path: &Path,
    exts: Vec<&str>,
    total_files_removed: &mut u32,
) -> std::io::Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let extension = entry.path().extension();

        for ext in exts.iter() {
            let ex = Some(OsStr::new(ext));
            if ex == extension {
                *total_files_removed += 1;
                Some(fs::remove_file(entry.path()).expect("Error removing file!"));
            }
        }
    }

    Ok(())
}

pub fn show_completion_output(total_files_removed: &mut u32) {
    println!("Finished deleting files");
    println!("A total of: {} files deleted", total_files_removed);
}