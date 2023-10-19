use clap::Subcommand;

use self::args::PoemArgs;

pub use super::args;

#[derive(Subcommand, Debug)]
pub enum PoemAction {
    Parse,
    Run(PoemArgs),
}
