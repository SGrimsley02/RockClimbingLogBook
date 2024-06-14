use sea_orm_migration::prelude::*;
use super::m20220101_000001_create_routes_table::Routes;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_sends_table.rs"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sends::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sends::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sends::Date).string().not_null())
                    .col(ColumnDef::new(Sends::Partner).string())
                    .col(ColumnDef::new(Sends::Type).string().not_null())
                    .col(ColumnDef::new(Sends::Attempts).integer().not_null())
                    .col(ColumnDef::new(Sends::Notes).string())
                    .col(ColumnDef::new(Sends::Route).integer().not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk-routes-sends_id")
                        .from(Sends::Table, Sends::Route)
                        .to(Routes::Table, Routes::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sends::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sends {
    Table,
    Id,
    Date,
    Partner,
    Type,
    Attempts,
    Notes,
    Route,
}