use crate::{db::connect, error::Result};
use account::{AccountRepo, AccountRepoImpl};
use std::sync::Arc;

pub mod account;

pub struct RepoImpls {
    pub account: AccountRepoImpl,
}

pub trait Repositories {
    type AccountRepoImpl: AccountRepo;

    fn account(&self) -> &Self::AccountRepoImpl;
}

impl Repositories for RepoImpls {
    type AccountRepoImpl = AccountRepoImpl;

    fn account(&self) -> &Self::AccountRepoImpl {
        &self.account
    }
}

pub async fn create_repositories() -> Result<Arc<RepoImpls>> {
    let conn = connect().await?;
    let account_repo = AccountRepoImpl { conn };

    let repos = RepoImpls { account: account_repo };

    Ok(Arc::new(repos))
}
