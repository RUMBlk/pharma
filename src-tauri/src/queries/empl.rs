use sea_orm::*;
use crate::entities::{*, prelude::*};
use sqlx::types::Decimal;
use num_traits::FromPrimitive;
use super::BasicErr;
use sea_query::{ IntoCondition, MysqlQueryBuilder };

pub fn name_cond(name: Option<String>) -> Condition {
    if let Some(v) = name {
        employees::Column::Name.contains(v).into_condition()
    } else {
        Condition::all()
    }
}

pub async fn empl_amount(db: &DatabaseConnection, position: i32) -> i32 {
    let query = Employees::find()
    .filter(employees::Column::Position.eq(position))
    .select_only()
    .column_as(employees::Column::Id.count(), "amount");

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Кількість працівників: {qstr}");

    query
    .into_tuple::<i32>()
    .one(db)
    .await
    .expect("DbErr")
    .expect("Помилка серіалізації запису")
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<employees::Model, BasicErr> {
    let query = Employees::find_by_id(id);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Вибір працівника: {qstr}");

    query
    .one(db)
    .await
    .map(|v| v.expect("Помилка серіалізації запису"))
    .map_err(|e| e.into())
}

pub async fn get_all(db: &DatabaseConnection, name: Option<String>, position: i32) -> Result<Vec<employees::Model>, BasicErr> {
    let query = Employees::find()
    .filter(name_cond(name))
    .filter(employees::Column::Position.eq(position));

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Список працівників: {qstr}");

    query
    .all(db)
    .await
    .map_err(|e| e.into())
}

pub async fn create(
    db: &DatabaseConnection,
    surname: String,
    name: String,
    patronim: String,
    position: i32,
    salary_bonus: Option<f64>,
) -> Result<employees::Model, BasicErr> {
    let salary_bonus = match salary_bonus.map(|v| Decimal::from_f64(v)) {
        Some(None) => return Err(BasicErr::Invalid),
        Some(v) => v,
        None => None,
    };

    let mut model = employees::ActiveModel {
        surname: Set(surname),
        name: Set(name),
        patronim: Set(patronim),
        position: Set(position),
        ..Default::default()
    };

    salary_bonus.inspect(|v| { model.set(employees::Column::SalaryBonus, (*v).into()); });
    
    let query = Employees::insert(model.clone());
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Додавання працівника: {qstr}");
    
    model.insert(db).await.map_err(|e| e.into())
}

pub async fn edit(
    db: &DatabaseConnection,
    id: i32,
    surname: Option<String>,
    name: Option<String>,
    patronim: Option<String>,
    position: Option<i32>,
    salary_bonus: Option<f64>,
) -> Result<UpdateResult, BasicErr> { 
    let mut stmt = Employees::update_many()
    .filter(employees::Column::Id.eq(id));

    if let Some(v) = surname { stmt = stmt.col_expr(employees::Column::Surname, v.into()) };
    if let Some(v) = name { stmt = stmt.col_expr(employees::Column::Name, v.into()) };
    if let Some(v) = patronim { stmt = stmt.col_expr(employees::Column::Patronim, v.into()) };
    if let Some(v) = position { stmt = stmt.col_expr(employees::Column::Position, v.into()) };
    if let Some(v) = salary_bonus { stmt = stmt.col_expr(employees::Column::SalaryBonus, v.into()) };

    let qstr = stmt.as_query().to_string(MysqlQueryBuilder);
    println!("Оновлення працівника: {qstr}");

    stmt.exec(db).await.map_err(|e| e.into())
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, BasicErr> {
    let query = Employees::delete_by_id(id);
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Видалення працівника: {qstr}");

    query.exec(db).await.map_err(|e| e.into())
}