mod matcher;
use std::process;
use crate::matcher::{Matcher};

fn main() {
    let matches = Matcher::new();

    if let Err(e) = matches.run(matches) {
            println!("Application error: {}", e);
            process::exit(1);
        }
}
