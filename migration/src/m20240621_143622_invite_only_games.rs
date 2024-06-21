use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Game::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Game::Pending)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Game::Table)
                    .drop_column(Game::Pending)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Pending,
}
