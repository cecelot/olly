pub use sea_orm_migration::prelude::*;

mod m20240112_000001_create_members_table;
mod m20240112_000002_unique_username;
mod m20240112_000003_create_sessions_table;
mod m20240113_000004_create_games_table;
mod m20240113_000005_null_guest;
mod m20240113_000006_non_null_guest;
mod m20240527_185255_create_friends;
mod m20240527_191255_create_friend_requests;
mod m20240621_143622_invite_only_games;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240112_000001_create_members_table::Migration),
            Box::new(m20240112_000002_unique_username::Migration),
            Box::new(m20240112_000003_create_sessions_table::Migration),
            Box::new(m20240113_000004_create_games_table::Migration),
            Box::new(m20240113_000005_null_guest::Migration),
            Box::new(m20240113_000006_non_null_guest::Migration),
            Box::new(m20240527_185255_create_friends::Migration),
            Box::new(m20240527_191255_create_friend_requests::Migration),
            Box::new(m20240621_143622_invite_only_games::Migration),
        ]
    }
}
