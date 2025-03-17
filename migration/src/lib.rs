pub use sea_orm_migration::prelude::*;

mod account_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(account_create_table::Migration)]
    }
}
