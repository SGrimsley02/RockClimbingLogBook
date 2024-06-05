use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_grades_table::Grades;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_routes_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to create routes table
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        

        manager
            .create_table(
                Table::create()
                    .table(Routes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Routes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Routes::Name).string().not_null())
                    .col(ColumnDef::new(Routes::Length).integer().not_null())
                    .col(ColumnDef::new(Routes::Pitches).integer().not_null())
                    .col(ColumnDef::new(Routes::Style).string().not_null())
                    .col(ColumnDef::new(Routes::GradeId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-grades-routes_id")
                            .from(Routes::Table, Routes::GradeId)
                            .to(Grades::Table, Grades::Id),
                    )
                    .to_owned(),
            )
            .await
    }
    // Define how to drop routes table
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Routes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Routes {
    Table,
    Id,
    Name,
    Length,
    Pitches,
    Style,
    GradeId,
}
