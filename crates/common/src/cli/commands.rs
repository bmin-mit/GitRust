use clap::Subcommand;
use crate::cli::args;

#[derive(Subcommand)]
pub enum CliCommands {
    HashObject(args::HashObjectArgs),
}