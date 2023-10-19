use anyhow::Result;
use clap::Parser;
use file_converters::{
    process::words::{args::WordArgs, cli::WordsAction},
    AsyncProcessor, Cli, CliArgs,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Cli::<WordsAction>::parse();

    match cfg.action {
        WordsAction::Run(mut args) => {
            args.parse_args()?.run().await?;
        }
        WordsAction::Parse => {
            WordArgs::default()
                .parse_args_interactively()?
                .run()
                .await?
        }
    }
    Ok(())
}
