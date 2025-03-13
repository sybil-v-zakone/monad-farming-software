use crate::Result;
use database::{db::generate, repositories::create_repositories};
use dialoguer::{Select, theme::ColorfulTheme};
use std::sync::Arc;

pub async fn menu() -> Result<()> {
    let repo = create_repositories().await?;
    let repo = Arc::new(repo);

    let random_count_range: [u32; 2] = [1, 5];

    loop {
        let options = vec!["DB: Generate", "Exit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choice:")
            .default(0)
            .items(&options)
            .interact()?;

        match selection {
            0 => generate(Arc::clone(&repo), random_count_range).await?,
            1 => return Ok(()),
            _ => tracing::error!("Invalid selection"),
        }
    }
}
