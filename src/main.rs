mod bundle;
mod regex;
mod utils;

use bundle::{bundle};
use utils::{pbcopy};

use argh::FromArgs;
use std::path::PathBuf;

#[derive(FromArgs)]
#[argh(description = "bndl")]
struct Args {
    #[argh(positional, description = "entry")]
    entry: String,

    #[argh(option, short = 'i', default = "4", description = "indent")]
    indent: usize,
}

fn main() -> std::io::Result<()> {
    let args = argh::from_env::<Args>();
    let Args { entry, indent } = args;

    let mut path = PathBuf::from(&entry);
    let bndl = bundle(&mut path, 0, indent)?;

    pbcopy(&bndl)?;
    Ok(())
}
