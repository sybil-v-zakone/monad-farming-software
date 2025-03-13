use super::entity::impls::prelude::AccountActiveModel;
use crate::{
    error::{Error, Result},
    repositories::RepoImpls,
    use_cases::accounts,
    utils::{fs::read_lines, random::random_in_range},
};
use alloy::signers::local::PrivateKeySigner;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbConn};
use std::{str::FromStr, sync::Arc};

pub async fn connect() -> Result<DbConn> {
    const DB_URL: &str = "sqlite://data/database.sqlite?mode=rwc";

    let mut options = ConnectOptions::new(DB_URL);
    options.sqlx_logging(false);

    let db = Database::connect(options).await?;

    Migrator::up(&db, None).await?;

    Ok(db)
}

pub async fn generate(repo: Arc<RepoImpls>, random_count_range: [u32; 2]) -> Result<()> {
    const PRIVATE_KEYS_PATH: &str = "data/private_keys.txt";
    const PROXIES_PATH: &str = "data/proxies.txt";

    let (pks, proxies) = tokio::try_join!(read_lines(PRIVATE_KEYS_PATH), read_lines(PROXIES_PATH))?;

    let mut proxies_iter = proxies.into_iter();

    for pk in pks {
        let address = match PrivateKeySigner::from_str(&pk) {
            Ok(signer) => signer.address(),
            Err(e) => {
                tracing::error!("Private key `{pk}` is invalid: {e}");
                continue;
            }
        };

        let random_count = random_in_range(random_count_range);

        let account = AccountActiveModel::new(
            pk.to_string(),
            proxies_iter.next(),
            address.to_string(),
            random_count as i32,
            random_count as i32,
            random_count as i32,
            random_count as i32,
            random_count as i32,
            random_count as i32,
        );

        if let Err(Error::Db(sea_orm::DbErr::Exec(rt_e))) =
            accounts::add(repo.clone(), account).await
        {
            if rt_e.to_string().contains("UNIQUE constraint failed") {
                tracing::warn!("An attempt to insert a duplicate entry failed, private key: `{pk}`")
            } else {
                tracing::error!("{}", rt_e.to_string());
            }
        }
    }

    Ok(())
}

pub async fn clear(repo: Arc<RepoImpls>) -> Result<()> {
    let _ = accounts::delete_all(repo.clone()).await?;

    Ok(())
}
