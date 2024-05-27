use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Friend::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Friend::A).uuid().not_null())
                    .col(ColumnDef::new(Friend::B).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Friend::Table, Friend::A)
                            .to(Member::Table, Member::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Friend::Table, Friend::B)
                            .to(Member::Table, Member::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .table(Friend::Table)
                            .col(Friend::A)
                            .col(Friend::B),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Friend::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Friend {
    Table,
    A,
    B,
}

#[derive(DeriveIden)]
enum Member {
    Table,
    Id,
}
