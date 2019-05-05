use clap::{App, Arg, ArgMatches};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

fn delete_files(path: &Path, exts: Vec<&str>) -> std::io::Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let extension = entry.path().extension();

        for ext in exts.iter() {
            let ex = Some(OsStr::new(ext));
            if ex == extension {
                Some(fs::remove_file(entry.path()));
            }
        }
    }

    Ok(())
}

fn run(matches: ArgMatches) -> Result<(), String> {
    let directory = matches.value_of("dir").unwrap();
    let extensions: Vec<&str> = matches.values_of("ext").unwrap().collect();

    let path = Path::new(directory);
    Some(delete_files(path, extensions));

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
        .arg(
            Arg::with_name("ext")
                .short("e")
                .long("ext")
                .help("Extensions to delete from dir")
                .required(true)
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
