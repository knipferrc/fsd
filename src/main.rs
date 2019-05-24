mod deleter;
mod app;
mod flags;

use crate::flags::Flags;

fn main() {
    let matches = app::build().get_matches();

    let _flags = Flags::from_matches(&matches);
}
