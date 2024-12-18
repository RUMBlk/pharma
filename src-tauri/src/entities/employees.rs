//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "employees")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i32,
    pub surname: String,
    pub name: String,
    pub patronim: String,
    pub hired_at: Date,
    pub position: i32,
    #[sea_orm(column_type = "Decimal(Some((8, 2)))")]
    pub salary_bonus: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::accounts::Entity")]
    Accounts,
    #[sea_orm(
        belongs_to = "super::positions::Entity",
        from = "Column::Position",
        to = "super::positions::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Positions,
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Accounts.def()
    }
}

impl Related<super::positions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Positions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
