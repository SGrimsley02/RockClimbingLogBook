use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_routes_table::Routes;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_grades_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to create grades table
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Grades::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Grades::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Grades::Yosemite).string().not_null())
                    .col(ColumnDef::new(Grades::Hueco).string().not_null())
                    .col(ColumnDef::new(Grades::Font).string().not_null())
                    .col(ColumnDef::new(Grades::French).string().not_null())
                    .col((ColumnDef::new(Grades::Uiaa).string().not_null()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_grades_routes_id")
                            .from(Routes::Table, Routes::Id)
                            .to(Grades::Table, Grades::Id),
                    )
                    .to_owned(),
            )
            .await
    }
    // Define how to drop grades table
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Routes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Grades {
    Table,
    Id,
    Yosemite,
    Font,
    French,
    Hueco,
    Uiaa,
    RouteId,
}
