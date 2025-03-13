use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_auto(Account::Id))
                    .col(string(Account::Address).unique_key())
                    .col(string(Account::PrivateKey).unique_key())
                    .col(ColumnDef::new(Account::Proxy).string())
                    .col(integer(Account::CurrentAmbientSwapsCount).default(0))
                    .col(integer(Account::TargetAmbientSwapsCount))
                    .col(integer(Account::CurrentAprioriDepositCount).default(0))
                    .col(integer(Account::TargetAprioriDepositCount))
                    .col(integer(Account::CurrentBeanSwapsCount).default(0))
                    .col(integer(Account::TargetBeanSwapsCount))
                    .col(integer(Account::CurrentHashflowSwapsCount).default(0))
                    .col(integer(Account::TargetHashflowSwapsCount))
                    .col(integer(Account::CurrentKinzaDepositCount).default(0))
                    .col(integer(Account::TargetKinzaDepositCount))
                    .col(integer(Account::CurrentShmonadDepositCount).default(0))
                    .col(integer(Account::TargetShmonadDepositCount))
                    .col(ColumnDef::new(Account::GoalReached).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Account::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Address,
    PrivateKey,
    Proxy,
    CurrentAmbientSwapsCount,
    TargetAmbientSwapsCount,
    CurrentAprioriDepositCount,
    TargetAprioriDepositCount,
    CurrentBeanSwapsCount,
    TargetBeanSwapsCount,
    CurrentHashflowSwapsCount,
    TargetHashflowSwapsCount,
    CurrentKinzaDepositCount,
    TargetKinzaDepositCount,
    CurrentShmonadDepositCount,
    TargetShmonadDepositCount,
    GoalReached,
}
