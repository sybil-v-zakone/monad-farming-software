use crate::{
    entity,
    entity::{
        impls::{account::AccountConditions, prelude::*},
        prelude::Account,
    },
    error::Result,
};
use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, DbConn, EntityTrait, QueryFilter};

pub struct AccountRepoImpl {
    pub conn: DbConn,
}

#[async_trait]
pub trait AccountRepo {
    async fn find_all(&self, filter: AccountConditions) -> Result<AccountList>;
    async fn add(&self, account: AccountActiveModel) -> Result<i32>;
    async fn delete_all(&self) -> Result<u64>;
    async fn update(&self, account: AccountActiveModel) -> Result<i32>;
}

#[async_trait]
impl AccountRepo for AccountRepoImpl {
    async fn find_all(&self, filter: AccountConditions) -> Result<AccountList> {
        let mut query = Account::find();
        let mut conditions = Condition::all();

        if let Some(goal_reached) = filter.goal_reached {
            conditions = conditions.add(entity::account::Column::GoalReached.eq(goal_reached));
        }

        query = query.filter(conditions);
        let accounts = query.all(&self.conn).await?;
        Ok(accounts)
    }

    async fn add(&self, account: AccountActiveModel) -> Result<i32> {
        let id = Account::insert(account).exec(&self.conn).await?.last_insert_id;

        Ok(id)
    }

    async fn delete_all(&self) -> Result<u64> {
        let result = Account::delete_many().exec(&self.conn).await?;
        Ok(result.rows_affected)
    }

    async fn update(&self, account: AccountActiveModel) -> Result<i32> {
        let result = Account::update(account.clone())
            .filter(entity::account::Column::Id.eq(account.id.unwrap()))
            .exec(&self.conn)
            .await?;

        Ok(result.id)
    }
}
