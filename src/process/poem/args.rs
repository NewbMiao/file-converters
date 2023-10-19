use anyhow::{Ok, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};
use std::default::Default;

use crate::CliArgs;

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
        let target = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Target of poem to search")
            .default("相思 王维".to_string())
            .interact_text()?;

        Ok(PoemArgs { target })
    }
}
