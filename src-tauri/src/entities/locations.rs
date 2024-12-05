//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "locations")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i32,
    pub name: String,
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::positions::Entity")]
    Positions,
    #[sea_orm(has_many = "super::stock::Entity")]
    Stock,
}

impl Related<super::positions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Positions.def()
    }
}

impl Related<super::stock::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
