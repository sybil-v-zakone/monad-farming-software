use common::state::{Dex, Lending, Nft};
use sea_orm::{
    ActiveValue::{self, Set},
    DbErr,
};

use crate::{
    entity::{
        account::ActiveModel,
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

pub async fn add<R: Repositories>(repo: Arc<R>, new_account: ActiveModel) -> Result<i32> {
    repo.account().add(new_account).await
}

pub async fn update<R: Repositories>(repo: Arc<R>, account: AccountActiveModel) -> Result<i32> {
    repo.account().update(account).await
}

pub async fn delete_all<R: Repositories>(repo: Arc<R>) -> Result<u64> {
    repo.account().delete_all().await
}

pub async fn deactivate_account_by_id<R: Repositories>(repo: Arc<R>, id: i32) -> Result<i32> {
    let accounts =
        repo.account().find_all(AccountConditions { id: Some(id), ..Default::default() }).await?;

    let account = accounts.into_iter().next().ok_or_else(|| Error::NotFound)?;
    let mut active_model = ActiveModel::from(account);
    active_model.goal_reached = Set(true);

    repo.account().update(active_model).await
}

pub async fn update_swap_count<R: Repositories>(
    repo: Arc<R>,
    dex: Dex,
    account: &AccountModel,
) -> Result<i32> {
    let mut active_model =
        AccountActiveModel { id: ActiveValue::set(account.id), ..Default::default() };

    match dex {
        Dex::Ambient => {
            active_model.current_ambient_swaps_count =
                ActiveValue::set(account.current_ambient_swaps_count + 1);
        }
        Dex::Bean => {
            active_model.current_bean_swaps_count =
                ActiveValue::set(account.current_bean_swaps_count + 1)
        }
        Dex::Hashflow => {
            active_model.current_hashflow_swaps_count =
                ActiveValue::set(account.current_hashflow_swaps_count + 1)
        }
    }

    update(repo, active_model).await
}

pub async fn update_deposit_count<R: Repositories>(
    repo: Arc<R>,
    lending: Lending,
    account: &AccountModel,
) -> Result<i32> {
    let mut active_model =
        AccountActiveModel { id: ActiveValue::set(account.id), ..Default::default() };

    match lending {
        Lending::Apriori => {
            active_model.current_apriori_deposit_count =
                Set(account.current_apriori_deposit_count + 1)
        }
        Lending::Kinza => {
            active_model.current_kinza_deposit_count = Set(account.current_kinza_deposit_count + 1)
        }
        Lending::Shmonad => {
            active_model.current_shmonad_deposit_count =
                Set(account.current_shmonad_deposit_count + 1)
        }
    };

    repo.account().update(active_model).await
}

pub async fn update_mint_count<R: Repositories>(
    repo: Arc<R>,
    nft: Nft,
    account: AccountModel,
) -> Result<i32> {
    let mut active_model =
        AccountActiveModel { id: ActiveValue::set(account.id), ..Default::default() };

    match nft {
        Nft::NadDomains => {
            active_model.current_nad_domains_count = Set(account.current_nad_domains_count + 1)
        }
    };

    repo.account().update(active_model).await
}

pub async fn update_bridge_goal<R: Repositories>(
    repo: Arc<R>,
    account: AccountModel,
) -> Result<i32> {
    let mut active_model =
        AccountActiveModel { id: ActiveValue::set(account.id), ..Default::default() };

    active_model.bridge_goal = Set(true);

    repo.account().update(active_model).await
}
