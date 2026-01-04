use clap::Args;

#[derive(Args)]
pub struct HashObjectArgs {
    #[arg(short)]
    pub write: bool,

    #[arg(long)]
    pub stdin: bool,
}

#[derive(Args)]
pub struct InitArgs {}

#[derive(Args)]
pub struct CatFileArgs {
    #[arg(short, group = "output_option")]
    type_only: bool,

    #[arg(short, group = "output_option")]
    size_only: bool,

    #[arg(short, group = "output_option")]
    print: bool,
}
