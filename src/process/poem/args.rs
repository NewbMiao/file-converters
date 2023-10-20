use anyhow::{Ok, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};
use std::default::Default;

use crate::CliArgs;

use super::lib::get_next_target;

#[derive(Parser, Debug, Clone, Default)]
pub struct PoemArgs {
    #[clap(short, long, default_value = "")]
    pub target: String,
}

impl CliArgs for PoemArgs {
    fn parse_args(&mut self) -> Result<Self> {
        Ok(self.clone())
    }
    fn parse_args_interactively(&mut self) -> Result<Self> {
        let next_target = get_next_target();
        let target = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Target of poem to search")
            .default(next_target)
            .interact_text()?;

        Ok(PoemArgs { target })
    }
}
