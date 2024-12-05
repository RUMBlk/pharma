use crate::entities::{*, prelude::*};
use sea_orm::*;
use sea_query::{ IntoCondition, MysqlQueryBuilder };
use super::BasicErr;

pub fn name_cond(name: Option<String>) -> Condition {
    if let Some(v) = name {
        categories::Column::Name.contains(v).into_condition()
    } else {
        Condition::all()
    }
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<categories::Model, BasicErr> {
    let query = Categories::find_by_id(id);
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Вибір категорії: {qstr}");
    
    query.one(db).await
    .map(|e| e.expect("Помилка десеріалізації моделі"))
    .map_err(|e| e.into())
}

pub async fn get_all(db: &DatabaseConnection, name: Option<String>) -> Vec<categories::Model> {
    let query = Categories::find().filter(name_cond(name)).order_by_asc(categories::Column::Name);
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Список категорій: {qstr}");

    query.all(db).await.expect("DbErr")
}

pub async fn create(db: &DatabaseConnection, name: String) -> Result<categories::Model, BasicErr> {
    let model = categories::ActiveModel {
        name: Set(name),
        ..Default::default()
    };

    let qstr = Categories::insert(model.clone()).as_query().to_string(MysqlQueryBuilder);
    println!("Додавання категорії: {qstr}");

    model.insert(db).await.map_err(|e| e.into())
}

pub async fn edit(db: &DatabaseConnection, id: i32, name: String) -> Result<categories::Model, BasicErr> {
    let model = categories::ActiveModel {
        id: Set(id),
        name: Set(name),
    };

    let qstr = Categories::update(model.clone()).as_query().to_string(MysqlQueryBuilder);
    println!("Оновлення категорії: {qstr}");

    model.update(db).await.map_err(|e| e.into())
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, BasicErr> {
    let query = Categories::delete_by_id(id);
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Видалення категорії: {qstr}");
    
    query.exec(db).await.map_err(|e| e.into())
}