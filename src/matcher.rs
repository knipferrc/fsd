use clap::{App, Arg, ArgMatches};
use std::path::Path;

use crate::deleter::Deleter;

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
                    .short("o")
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
            .get_matches();

        Matcher { matches }
    }

    pub fn run(&self) -> Result<(), String> {
        let directory = self.matches.value_of("directory").unwrap();
        let mut file_extensions = Vec::new();
        let mut file_names = Vec::new();
        let mut folders = Vec::new();
        let total_files_removed = &mut 0;
        let total_folders_removed = &mut 0;

        let path = Path::new(directory);

        if self.matches.is_present("ext") {
            if let Some(extensions) = Some(self.matches.values_of("extensions").unwrap().collect()) {
                file_extensions = extensions;
            }
        }

        if self.matches.is_present("fnames") {
            if let Some(filenames) = Some(self.matches.values_of("filenames").unwrap().collect()) {
                file_names = filenames;
            }
        }

        if self.matches.is_present("folders") {
            if let Some(fldrs) = Some(self.matches.values_of("folders").unwrap().collect()) {
                folders = fldrs;
            }
        }

        if self.matches.is_present("preset") {
            if let Some(preset) = Some(self.matches.value_of("preset").unwrap()) {
                if preset == "node_modules" {
                    file_extensions =
                        vec!["markdown", "md", "mkd", "ts", "jst", "coffee", "tgz", "swp"];

                    folders = vec![
                        "__tests__",
                        "test",
                        "tests",
                        "powered-test",
                        "docs",
                        "doc",
                        ".idea",
                        ".vscode",
                        "website",
                        "images",
                        "assets",
                        "example",
                        "examples",
                        "coverage",
                        ".nyc_output",
                        ".circleci",
                        ".github",
                    ];

                    file_names = vec![
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
                }
            }
        }

        let mut deleter = Deleter::new(
            path,
            file_extensions,
            file_names,
            folders,
            total_files_removed,
            total_folders_removed
        );
        deleter.calculate_size();
        deleter.delete_files();
        deleter.show_results();
        deleter.calculate_size();

        Ok(())
    }
}
