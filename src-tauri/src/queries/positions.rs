use crate::entities::{*, prelude::*};
use sea_orm::*;
use sea_query::{ IntoCondition, MysqlQueryBuilder };
use sqlx::types::Decimal;
use super::BasicErr;
use num_traits::cast::FromPrimitive;

pub fn name_cond(name: Option<String>) -> Condition {
    if let Some(v) = name {
        positions::Column::Name.contains(v).into_condition()
    } else {
        Condition::all()
    }
}

pub fn salary_cond(mut start: Option<i32>, mut end: Option<i32>) -> Condition {
    if start.is_some() && start == end {
        start = start.and_then(|v| Some(v-1));
        end = end.and_then(|v| Some(v+1))
    }
    
    let mut cond = Condition::all();
    if let Some(v) = start { cond = cond.add(positions::Column::Salary.gte(v)); };
    if let Some(v) = end { cond = cond.add(positions::Column::Salary.lte(v));};
    cond
}

pub fn location_cond(locations: Option<Vec<i32>>) -> Condition {
    let mut cond = Condition::any();
    if let Some(v) = locations {
        for location in v {
            cond = cond.add(positions::Column::Location.eq(location));
        };
    }
    cond
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<positions::Model, BasicErr> {
    let query = Positions::find_by_id(id);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Вибір посади: {qstr}");

    query.one(db).await
    .map(|e| e.expect("Помилка десеріалізації моделі"))
    .map_err(|e| e.into())
}

pub async fn get_all(
    db: &DatabaseConnection,
    name: Option<String>,
    salary_start: Option<i32>,
    salary_end: Option<i32>,
    locations: Option<Vec<i32>>,
) -> Vec<positions::Model> {
    println!("{salary_start:.?} {salary_end:.?}");
    let query = Positions::find()
    .filter(name_cond(name))
    .filter(salary_cond(salary_start, salary_end))
    .filter(location_cond(locations))
    .order_by_asc(positions::Column::Name);
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Список посад: {qstr}");

    query.all(db).await.expect("DbErr")
}

pub async fn get_salary_range(
    db: &DatabaseConnection,
    name: Option<String>,
    locations: Option<Vec<i32>>
) -> (Decimal, Decimal) {
    let query = Positions::find()
    .filter(name_cond(name))
    .filter(location_cond(locations))
    .select_only()
    .column_as(positions::Column::Salary.min(), "min")
    .column_as(positions::Column::Salary.max(), "max");

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Вибір діапазону зарплати: {qstr}");

    query
    .into_tuple::<(Decimal, Decimal)>()
    .one(db)
    .await
    .unwrap_or(Some((Decimal::ZERO, Decimal::ZERO)))
    .expect("Помилка серіалізації запису")
}

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    salary: f64,
    location: i32,
    can_manage_empl: Option<bool>,
    can_manage_products: Option<bool>,
    can_manage_categories: Option<bool>,
    can_sell_products: Option<bool>,
    can_manage_locations: Option<bool>
) -> Result<positions::Model, BasicErr> {
    let Some(salary) = Decimal::from_f64(salary) else { return Err(BasicErr::Invalid); };

    let mut model = positions::ActiveModel {
        name: Set(name),
        salary: Set(salary),
        location: Set(location.into()),
        ..Default::default()
    };

    can_manage_empl.inspect(|v| { model.set(positions::Column::CanManageEmpl, (*v as i8).into()); });
    can_manage_products.inspect(|v| { model.set(positions::Column::CanManageProducts, (*v as i8).into()); });
    can_manage_categories.inspect(|v| { model.set(positions::Column::CanManageCategories, (*v as i8).into()); });
    can_sell_products.inspect(|v| { model.set(positions::Column::CanSellProducts, (*v as i8).into()); });
    can_manage_locations.inspect(|v| { model.set(positions::Column::CanManageLocations, (*v as i8).into()); });

    let query = Positions::insert(model.clone());
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Додавання посади: {qstr}");

    model.insert(db).await.map_err(|e| e.into())
}

pub async fn edit(
    db: &DatabaseConnection,
    id: i32,
    name: Option<String>,
    salary: Option<f64>,
    location: Option<i32>,
    can_manage_empl: Option<bool>,
    can_manage_products: Option<bool>,
    can_manage_categories: Option<bool>,
    can_sell_products: Option<bool>,
    can_manage_locations: Option<bool>
) -> Result<UpdateResult, BasicErr> { 
    let mut stmt = Positions::update_many()
    .filter(positions::Column::Id.eq(id));

    let salary = match salary.map(|v| Decimal::from_f64(v)) {
        Some(None) => return Err(BasicErr::Invalid),
        Some(v) => v,
        None => None,
    };

    if let Some(v) = name { stmt = stmt.col_expr(positions::Column::Name, v.into()) };
    if let Some(v) = salary { stmt = stmt.col_expr(positions::Column::Salary, v.into()) };
    if let Some(v) = location { stmt = stmt.col_expr(positions::Column::Location, v.into()) };
    if let Some(v) = can_manage_empl { stmt = stmt.col_expr(positions::Column::CanManageEmpl, (v as i8).into()) };
    if let Some(v) = can_manage_products { stmt = stmt.col_expr(positions::Column::CanManageProducts, (v as i8).into()) };
    if let Some(v) = can_manage_categories { stmt = stmt.col_expr(positions::Column::CanManageCategories, (v as i8).into()) };
    if let Some(v) = can_sell_products { stmt = stmt.col_expr(positions::Column::CanSellProducts, (v as i8).into()) };
    if let Some(v) = can_manage_locations { stmt = stmt.col_expr(positions::Column::CanManageLocations, (v as i8).into()) };

    let qstr = stmt.as_query().to_string(MysqlQueryBuilder);
    println!("Оновлення посади: {qstr}");

    stmt.exec(db).await.map_err(|e| e.into())
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, BasicErr> {
    let query = Positions::delete_by_id(id);
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Видалення посади: {qstr}");

    query.exec(db).await.map_err(|e| e.into())
}