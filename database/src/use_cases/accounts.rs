use sea_orm::Set;

use crate::{
    entity::{
        self,
        account::Model as AccountModel,
        impls::{account::AccountConditions, prelude::AccountList},
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
    accounts.into_iter().next().ok_or(Error::NotFound)
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

pub async fn deactivate_account_by_id<R: Repositories>(repo: Arc<R>, id: i32) -> Result<i32> {
    let accounts =
        repo.account().find_all(AccountConditions { id: Some(id), goal_reached: None }).await?;

    let account = accounts.into_iter().next().ok_or_else(|| Error::NotFound)?;

    let mut active_model = entity::account::ActiveModel::from(account);
    active_model.goal_reached = Set(true);

    repo.account().update(active_model).await
}
