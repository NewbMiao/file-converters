use std::collections::HashMap;

use crate::file::load_csv_line_stream;

use super::{
    analyzer::{analyze_words, get_top_ten_words, merge_words},
    args::WordArgs,
    processor::ContentRow,
};
use anyhow::{Ok, Result};
use tokio::{
    sync::mpsc::{self, Sender},
    task,
};

impl WordArgs {
    pub async fn get_top_ten_words_via_mpsc(&self) -> Result<()> {
        let lines = load_csv_line_stream::<ContentRow>(&self.file);

        let mut handles = vec![];
        let mut senders: HashMap<u32, Sender<String>> = HashMap::new();
        let mut prev_id = 0;
        for line in lines {
            let line = line?;

            let entry = senders.entry(line.id).or_insert_with(|| {
                let (tx, mut rx) = mpsc::channel::<String>(10);
                let handle = task::spawn(async move {
                    let mut result = HashMap::new();
                    while let Some(content) = rx.recv().await {
                        merge_words(&mut result, analyze_words(content.as_str()));
                    }
                    (line.id, get_top_ten_words(result))
                });
                handles.push(handle);
                tx
            });
            entry.send(line.content).await?;

            if prev_id > 0 && prev_id != line.id {
                // drop previous entry
                // N.B. assume the lines are ordered by id
                if let Some(sender) = senders.remove(&prev_id) {
                    drop(sender);
                }
                prev_id = line.id;
            }
        }
        for (_, sender) in senders {
            drop(sender);
        }
        let mut results = HashMap::new();
        for handle in handles {
            let (id, words) = handle.await?;
            results.insert(id, words);
        }
        self.write_to_file(results).await?;
        Ok(())
    }
}
