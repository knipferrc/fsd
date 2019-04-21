use clap::{App, Arg, ArgMatches};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

fn delete_files(path: &Path) -> std::io::Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(OsStr::to_str) == Some("css") {
            fs::remove_file(entry.path())?;
        }
    }
    
    Ok(())
}

fn run(matches: ArgMatches) -> Result<(), String> {
    let directory = matches.value_of("dir").unwrap();

    let path = Path::new(directory);
    Some(delete_files(path));

    Ok(())
}

fn main() {
    let matches = App::new("FSD")
        .version("1.0")
        .about("Trashes files within a directory recursivesly")
        .author("Tyler K.")
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .help("Directory to delete files in")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
