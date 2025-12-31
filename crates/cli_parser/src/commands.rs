use clap::Subcommand;
use crate::args::*;

#[derive(Subcommand)]
pub enum CliCommands {
    HashObject(HashObjectArgs),
    Init(InitArgs),
}