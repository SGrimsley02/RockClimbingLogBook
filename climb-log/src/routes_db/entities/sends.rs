//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sends")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub date: String,
    pub partner: Option<String>,
    pub r#type: String,
    pub attempts: i32,
    pub notes: Option<String>,
    pub route: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::routes::Entity",
        from = "Column::Route",
        to = "super::routes::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Routes,
}

impl Related<super::routes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Routes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
