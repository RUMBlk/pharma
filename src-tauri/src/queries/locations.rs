use crate::entities::{*, prelude::*};
use sea_orm::*;
use super::BasicErr;
use sea_query::MysqlQueryBuilder;

pub fn name_cond(name: Option<String>) -> Condition {
    if let Some(v) = name {
        Condition::any()    
        .add(locations::Column::Name.contains(&v))
        .add(locations::Column::Address.contains(&v))
    } else {
        Condition::all()
    }
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<locations::Model, BasicErr> {
    let query = Locations::find_by_id(id);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Вибір локації: {qstr}");

    query
    .one(db).await
    .map(|e| e.expect("Помилка десеріалізації моделі"))
    .map_err(|e| e.into())
}

pub async fn get_all(db: &DatabaseConnection, name: Option<String>) -> Vec<locations::Model> {
    let query = Locations::find().filter(name_cond(name));
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Список локацій: {qstr}");

    query.all(db).await.expect("DbErr")
}

pub async fn create(db: &DatabaseConnection, name: String, address: String) -> Result<locations::Model, BasicErr> {
    let model = locations::ActiveModel {
        name: Set(name),
        address: Set(address),
        ..Default::default()
    };

    let query = Locations::insert(model.clone());
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Додавання локації: {qstr}");

    model.insert(db).await.map_err(|e| e.into())
}

pub async fn edit(
    db: &DatabaseConnection,
    id: i32,
    name: Option<String>,
    address: Option<String>,
) -> Result<UpdateResult, BasicErr> { 
    let mut stmt = Locations::update_many()
    .filter(locations::Column::Id.eq(id));

    if let Some(v) = name { stmt = stmt.col_expr(locations::Column::Name, v.into()) };
    if let Some(v) = address { stmt = stmt.col_expr(locations::Column::Address, v.into()) };

    let qstr = stmt.as_query().to_string(MysqlQueryBuilder);
    println!("Оновлення локації: {qstr}");

    stmt.exec(db).await.map_err(|e| e.into())
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, BasicErr> {
    let query = Locations::delete_by_id(id);
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Видалення локації: {qstr}");

    query.exec(db).await.map_err(|e| e.into())
}