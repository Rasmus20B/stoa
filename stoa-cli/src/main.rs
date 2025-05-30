mod commands;
mod error;

use error::{Error, Result};

use clap::Parser;
use commands::CommandLine;

fn main() -> Result<()> {
    let args = commands::CommandLine::parse();
    let query = args.query.ok_or(Error::NoQuerySpecified)?;
    let file = args.file.ok_or(Error::NoFileSpecified)?;

    println!("file: {query:}");
    println!("file: {file:?}");

    Ok(())
}
