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
            .arg(
                Arg::with_name("preset")
                    .short("p")
                    .long("preset")
                    .help("Presets available: 'node_modules'")
                    .takes_value(true),
            )
            .group(
                ArgGroup::with_name("files")
                    .args(&vec!["ext", "fnames", "preset"])
                    .required(true),
            )
            .get_matches();

        Matcher { matches }
    }

    pub fn run(&self) -> Result<(), String> {
        let mut deleter;
        let directory = self.matches.value_of("dir").unwrap();

        let total_files_removed = &mut 0;
        let second_total_files = &mut 0;
        let third_total_files = &mut 0;

        let path = Path::new(directory);

        if self.matches.is_present("ext") {
            if let Some(extensions) = Some(self.matches.values_of("ext").unwrap().collect()) {
                deleter = Deleter::new(path, extensions, Vec::new(), total_files_removed);
                deleter.delete_files();
                deleter.show_results();
            }
        }

        if self.matches.is_present("fnames") {
            if let Some(filenames) = Some(self.matches.values_of("fnames").unwrap().collect()) {
                deleter = Deleter::new(path, Vec::new(), filenames, second_total_files);
                deleter.delete_files();
                deleter.show_results();
            }
        }

        if self.matches.is_present("preset") {
            if let Some(preset) = Some(self.matches.value_of("preset").unwrap()) {
                if preset == "node_modules" {
                    let node_preset_extensions = vec![
                        "markdown",
                        "md",
                        "mkd",
                        "ts",
                        "jst",
                        "coffee",
                        "tgz",
                        "swp",
                    ];

                    let node_preset_filenames = vec![
                        "Jenkinsfile",
                        "Makefile",
                        "Gulpfile.js",
                        "Gruntfile.js",
                        "gulpfile.js",
                        ".DS_Store",
                        ".tern-project",
                        ".gitattributes",
                        ".editorconfig",
                        ".eslintrc",
                        "eslint",
                        ".eslintrc.js",
                        ".eslintrc.json",
                        ".eslintrc.yml",
                        ".eslintignore",
                        ".stylelintrc",
                        "stylelint.config.js",
                        ".stylelintrc.json",
                        ".stylelintrc.yaml",
                        ".stylelintrc.yml",
                        ".stylelintrc.js",
                        ".htmllintrc",
                        "htmllint.js",
                        ".lint",
                        ".npmrc",
                        ".npmignore",
                        ".jshintrc",
                        ".flowconfig",
                        ".documentup.json",
                        ".yarn-metadata.json",
                        ".travis.yml",
                        "appveyor.yml",
                        ".gitlab-ci.yml",
                        "circle.yml",
                        ".coveralls.yml",
                        "CHANGES",
                        "changelog",
                        "LICENSE.txt",
                        "LICENSE",
                        "LICENSE-MIT",
                        "LICENSE.BSD",
                        "license",
                        "LICENCE.txt",
                        "LICENCE",
                        "LICENCE-MIT",
                        "LICENCE.BSD",
                        "licence",
                        "AUTHORS",
                        "CONTRIBUTORS",
                        ".yarn-integrity",
                        ".yarnclean",
                        "_config.yml",
                        ".babelrc",
                        ".yo-rc.json",
                        "jest.config.js",
                        "karma.conf.js",
                        "wallaby.js",
                        "wallaby.conf.js",
                        ".prettierrc",
                        ".prettierrc.yml",
                        ".prettierrc.toml",
                        ".prettierrc.js",
                        ".prettierrc.json",
                        "prettier.config.js",
                        ".appveyor.yml",
                        "tsconfig.json",
                        "tslint.json",
                    ];
                    
                    deleter =
                        Deleter::new(path, node_preset_extensions, node_preset_filenames, third_total_files);
                    deleter.delete_files();
                    deleter.show_results();
                }
            }
        }

        Ok(())
    }
}
