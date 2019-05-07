mod file_deleter;

use clap::{App, Arg, ArgMatches};
use std::path::Path;
use crate::file_deleter::{FileDeleter};

pub struct Matcher {}

impl Matcher {
    pub fn new() -> ArgMatches 'static {
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

        return matches;
    }

    fn run(matches: ArgMatches) -> Result<(), String> {
        let directory = matches.value_of("dir").unwrap();
        let extensions: Vec<&str> = matches.values_of("ext").unwrap().collect();
        let total_files_removed = &mut 0;
        let path = Path::new(directory);

        let mut file_deleter = FileDeleter::new(path, extensions, total_files_removed);

        file_deleter.delete_files();
        file_deleter.show_completion_output();

        Ok(())
    }
}
