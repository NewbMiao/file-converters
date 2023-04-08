use std::path::PathBuf;

use anyhow::{Ok, Result};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub struct AsyncFileWriter {
    file: tokio::fs::File,
}

impl AsyncFileWriter {
    pub async fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .await?;
        Ok(Self { file })
    }

    pub async fn write_line(&mut self, line: &str) -> Result<()> {
        self.file.write_all(line.as_bytes()).await?;
        self.file.write_all(b"\n").await?;
        Ok(())
    }
    pub async fn flush(&mut self) -> Result<()> {
        self.file.flush().await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_file_write_should_work() -> Result<()> {
    use std::fs;

    let path = "test.txt";
    let line = "hello from test";
    fs::remove_file(path)?;
    let mut writer = AsyncFileWriter::new(path).await?;
    writer.write_line(line).await?;
    writer.flush().await?;
    assert_eq!(fs::read_to_string(path)?, format!("{}\n", line));

    Ok(())
}
