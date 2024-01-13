pub use sea_orm_migration::prelude::*;

mod m20240112_000001_create_members_table;
mod m20240112_000002_unique_username;
mod m20240112_000003_create_sessions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240112_000001_create_members_table::Migration),
            Box::new(m20240112_000002_unique_username::Migration),
            Box::new(m20240112_000003_create_sessions_table::Migration),
        ]
    }
}
