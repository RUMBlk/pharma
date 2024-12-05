pub mod products;
pub mod categories;
pub mod accounts;
pub mod sessions;
pub mod stock;
pub mod locations;
pub mod positions;
pub mod empl;

use serde::{Serialize, Deserialize};
use sea_orm::DbErr;

#[derive(Serialize, Deserialize)]
pub enum BasicErr {
    Exists,
    NotFound,
    Internal,
    NotAuthorized,
    Invalid
}

impl From<DbErr> for BasicErr {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::RecordNotInserted => Self::Exists,
            DbErr::RecordNotUpdated => Self::NotFound,
            DbErr::RecordNotFound(_) => Self::NotFound,
            value => { eprintln!("{value}"); Self::Internal },
        }
    }
}