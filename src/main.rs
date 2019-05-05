use clap::{App, Arg, ArgMatches};
use std::path::Path;
use std::process;

mod files;

fn run(matches: ArgMatches) -> Result<(), String> {
    let directory = matches.value_of("dir").unwrap();
    let extensions: Vec<&str> = matches.values_of("ext").unwrap().collect();
    let total_files_removed = &mut 0;

    let path = Path::new(directory);
    Some(files::delete_files(path, extensions, total_files_removed));

    files::show_completion_output(total_files_removed);

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
