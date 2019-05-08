use std::process;

use matcher::matcher::Matcher;

mod deleter;
mod matcher;

fn main() {
    let matcher = Matcher::new();

    if let Err(e) = matcher.run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
