use clap::{App, Arg};

pub fn build() -> App<'static, 'static> {
    App::new("FSD")
    .version("1.0")
    .about("Trashes files within a directory recursivesly")
    .author("Tyler K.")
    .arg(
        Arg::with_name("directory")
            .short("d")
            .long("directory")
            .help("Directory to delete files in")
            .required(true)
            .takes_value(true),
    )
    .arg(
        Arg::with_name("extensions")
            .short("e")
            .long("extensions")
            .help("Extensions to delete from directory")
            .multiple(true)
            .takes_value(true),
    )
    .arg(
        Arg::with_name("filenames")
            .short("f")
            .long("filenames")
            .help("Filenames to delete from directory")
            .multiple(true)
            .takes_value(true),
    )
    .arg(
        Arg::with_name("folders")
            .short("F")
            .long("folders")
            .help("Folders to delete from directory")
            .multiple(true)
            .takes_value(true),
    )
    .arg(
        Arg::with_name("preset")
            .short("p")
            .long("preset")
            .help("Presets available: 'node_modules'")
            .takes_value(true),
    )
}