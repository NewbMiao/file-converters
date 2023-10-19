use anyhow::Result;
use clap::Parser;
use file_converters::{
    process::poem::{args::PoemArgs, cli::PoemAction},
    AsyncProcessor, Cli, CliArgs,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Cli::<PoemAction>::parse();

    match cfg.action {
        PoemAction::Run(mut args) => {
            args.parse_args()?.run().await?;
        }
        PoemAction::Parse => {
            PoemArgs::default()
                .parse_args_interactively()?
                .run()
                .await?
        }
    }
    Ok(())
}
