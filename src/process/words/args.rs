use anyhow::{anyhow, Ok, Result};
use clap::{Parser, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::{default::Default, fmt::Display, path::PathBuf};

use crate::{file::get_file_full_path, CliArgs};

#[derive(ValueEnum, Clone, Copy, Debug, Default)]
pub enum WType {
    #[default]
    Mpsc,
    Rayon,
}
impl Display for WType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WType::Mpsc => f.write_str("mpsc"),
            WType::Rayon => f.write_str("rayon"),
        }
    }
}

#[derive(Parser, Debug, Clone, Default)]
pub struct WordArgs {
    #[clap(short, long, value_parser(get_file_full_path))]
    pub file: PathBuf,

    #[clap(short, long, value_enum, default_value = "mpsc")]
    pub w_type: WType,
}

impl CliArgs for WordArgs {
    fn parse_args(&mut self) -> Result<Self> {
        Ok(self.clone())
    }
    fn parse_args_interactively(&mut self) -> Result<Self> {
        let file: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Path of words file (csv)")
            .default("fixtures/words.csv".to_string())
            .interact_text()?;
        let path = get_file_full_path(&file)?;
        if !path.exists() {
            return Err(anyhow!("Path invalid"));
        }
        let range_items = &vec![WType::Mpsc, WType::Rayon];
        let picked_range = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an analyze type")
            .items(range_items)
            .default(0)
            .interact()?;
        let w_type = range_items[picked_range];

        Ok(WordArgs { file: path, w_type })
    }
}
