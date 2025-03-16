use crate::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    // swap
    pub ambient_swap_count: [u32; 2],
    pub hashflow_swap_count: [u32; 2],
    pub bean_swap_count: [u32; 2],

    // deposit
    pub apriori_deposit_count: [u32; 2],
    pub kinza_deposit_count: [u32; 2],
    pub shmonad_deposit_count: [u32; 2],

    // nft
    pub nad_domains_count: [u32; 2],

    // misc
    pub thread_delay: [u64; 2],
    pub action_delay: [u64; 2],
    pub deposit_ratio: [u32; 2],
    pub swap_ratio: [u32; 2],
    pub rpc_url: String,
}

impl Config {
    const PATH: &str = "data/config.toml";

    async fn read_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let cfg_str = tokio::fs::read_to_string(path).await?;
        Ok(toml::from_str(&cfg_str)?)
    }

    pub async fn read_default() -> Self {
        Self::read_from_file(Self::PATH).await.expect("default config to be valid")
    }
}
