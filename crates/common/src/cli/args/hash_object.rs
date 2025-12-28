use clap::{Args};

#[derive(Args)]
pub struct HashObjectArgs {
    #[arg(short)]
    pub write: bool,

    #[arg(long)]
    pub stdin: bool,
}