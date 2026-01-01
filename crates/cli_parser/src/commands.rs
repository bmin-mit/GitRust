use crate::args::*;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum CliCommands {
    HashObject(HashObjectArgs),
    Init(InitArgs),
}
