use std::collections::HashMap;

use crate::file::load_excel_data;

use super::{
    analyzer::{analyze_words, merge_words_per_id, WordCounter},
    args::WordArgs,
    processor::ContentRow,
};
use anyhow::{Ok, Result};
use rayon::prelude::*;
impl WordArgs {
    pub async fn get_top_ten_words_via_rayon(&self) -> Result<()> {
        let results = load_excel_data::<ContentRow>(&self.file)?;
        let words_counts: HashMap<u32, WordCounter> = results
            .par_iter()
            .fold(HashMap::new, |mut acc, v| {
                let words = analyze_words(v.content.as_str());
                merge_words_per_id(v.id, &mut acc, words);
                acc
            })
            .reduce(HashMap::new, |mut acc, batch| {
                batch
                    .into_iter()
                    .for_each(|(id, words)| merge_words_per_id(id, &mut acc, words));
                acc
            });
        self.write_to_file(words_counts).await?;

        Ok(())
    }
}
