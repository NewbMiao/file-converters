use clap::Subcommand;

use self::args::WordArgs;

pub use super::args;

#[derive(Subcommand, Debug)]
pub enum WordsAction {
    Run(WordArgs),
    Parse,
}
