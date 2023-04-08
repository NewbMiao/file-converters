use super::{
    analyzer::{get_top_ten_words, WordCounter},
    args::{WType, WordArgs},
};
use crate::{file_writer::AsyncFileWriter, AsyncProcessor};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContentRow {
    pub id: u32,
    pub content: String,
}

impl WordArgs {
    pub async fn write_to_file(&self, results: HashMap<u32, WordCounter>) -> Result<()> {
        let path = "words_output.csv";
        let mut file = AsyncFileWriter::new(path).await?;
        for (id, words) in results {
            file.write_line(&format!(
                "{},{}",
                id,
                serde_json::to_string(&get_top_ten_words(words)).unwrap()
            ))
            .await?;
        }

        file.flush().await?;
        println!("Generated word counts to file {}", path);
        Ok(())
    }
}
#[async_trait]
impl AsyncProcessor for WordArgs {
    type Item = ContentRow;

    async fn run(&self) -> Result<()> {
        match self.w_type {
            WType::Mpsc => self.get_top_ten_words_via_mpsc().await?,
            WType::Rayon => self.get_top_ten_words_via_rayon().await?,
        };
        Ok(())
    }
}
