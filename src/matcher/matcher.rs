use clap::{App, Arg, ArgGroup, ArgMatches};
use std::path::Path;

use crate::deleter::deleter::Deleter;

pub struct Matcher {
    matches: ArgMatches<'static>,
}

impl Matcher {
    pub fn new() -> Matcher {
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
                    .help("Extensions to delete from directory")
                    .multiple(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("fnames")
                    .short("f")
                    .long("fnames")
                    .help("Filenames to delete from directory")
                    .multiple(true)
                    .takes_value(true),
            )
            .group(
                ArgGroup::with_name("files")
                    .args(&vec!["ext", "fnames"])
                    .required(true),
            )
            .get_matches();

        Matcher { matches }
    }

    pub fn run(&self) -> Result<(), String> {
        let directory = self.matches.value_of("dir").unwrap();

        if let Some(extensions) = Some(self.matches.values_of("ext").unwrap().collect()) {
            println!("Worked");
        }

        let filenames: Vec<&str> = self.matches.values_of("fnames").unwrap().collect();
        let total_files_removed = &mut 0;
        let path = Path::new(directory);

        let mut deleter = Deleter::new(path, extensions, filenames, total_files_removed);

        deleter.delete_files();
        deleter.show_results();

        Ok(())
    }
}
