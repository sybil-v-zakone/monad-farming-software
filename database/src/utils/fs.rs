use crate::error::Result;
use std::path::Path;
use tokio::io::AsyncBufReadExt;

pub async fn read_lines(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = tokio::fs::read(path).await?;
    let mut lines = file.lines();

    let mut contents = vec![];
    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            contents.push(trimmed.to_string());
        }
    }

    Ok(contents)
}
