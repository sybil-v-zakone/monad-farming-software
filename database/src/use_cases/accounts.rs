use common::state::{Dex, Lending};
use sea_orm::DbErr;

use crate::{
    entity::{
        self,
        impls::{account::AccountConditions, prelude::*},
    },
    error::{Error, Result},
    repositories::{Repositories, account::AccountRepo},
};
use std::sync::Arc;

pub async fn search<R: Repositories>(
    repo: Arc<R>,
    filter: AccountConditions,
) -> Result<AccountList> {
    repo.account().find_all(filter).await
}

pub async fn search_account_by_id<R: Repositories>(repo: Arc<R>, id: i32) -> Result<AccountModel> {
    let accounts =
        repo.account().find_all(AccountConditions { id: Some(id), goal_reached: None }).await?;
    accounts.into_iter().next().ok_or(Error::Db(DbErr::RecordNotFound(format!("{id}"))))
}

pub async fn add<R: Repositories>(
    repo: Arc<R>,
    new_account: entity::account::ActiveModel,
) -> Result<i32> {
    repo.account().add(new_account).await
}

pub async fn delete_all<R: Repositories>(repo: Arc<R>) -> Result<u64> {
    repo.account().delete_all().await
}

pub async fn deactivate_account_by_id<R: Repositories>(
    repo: Arc<R>,
    id: i32,
) -> Result<AccountActiveModel> {
    let accounts =
        repo.account().find_all(AccountConditions { id: Some(id), ..Default::default() }).await?;

    let mut account = accounts.into_iter().next().ok_or_else(|| Error::NotFound)?;
    account.goal_reached = true;

    repo.account().update(account).await
}

pub async fn update_swap_count<R: Repositories>(
    repo: Arc<R>,
    dex: Dex,
    mut account: AccountModel,
) -> Result<AccountActiveModel> {
    match dex {
        Dex::Ambient => account.current_ambient_swaps_count += 1,
        Dex::Bean => account.current_bean_swaps_count += 1,
        Dex::Hashflow => account.current_hashflow_swaps_count += 1,
    };

    repo.account().update(account).await
}

pub async fn update_deposit_count<R: Repositories>(
    repo: Arc<R>,
    lending: Lending,
    mut account: AccountModel,
) -> Result<AccountActiveModel> {
    match lending {
        Lending::Apriori => account.current_apriori_deposit_count += 1,
        Lending::Kinza => account.current_kinza_deposit_count += 1,
        Lending::Shmonad => account.current_shmonad_deposit_count += 1,
    };

    repo.account().update(account).await
}
