use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CommandLine {
    #[clap(short = 'q')]
    pub query: Option<String>,
    #[clap(short = 'f')]
    pub file: Option<PathBuf>,
}
