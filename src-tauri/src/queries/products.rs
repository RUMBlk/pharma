use crate::entities::{*, prelude::*};
use sea_orm::*;
use sea_query::{Expr, IntoCondition, MysqlQueryBuilder};
use sqlx::types::Decimal;
use serde::Serialize;
use num_traits::cast::FromPrimitive;
use super::BasicErr;

pub fn name_cond(name: Option<String>) -> Condition {
    if let Some(v) = name {
        products::Column::Name.contains(v).into_condition()
    } else {
        Condition::all()
    }
}

pub fn category_cond(mut categories: Option<Vec<i32>>) -> Condition {
    let mut cond = Condition::any();
    if let Some(v) = &mut categories {
        if v.contains(&(-1)) {
            cond = cond.add(products::Column::Category.is_null());
            v.retain(|&x| x != -1);
        }
        cond = cond.add(products::Column::Category.is_in(v.to_owned()))
    }
    cond
}

pub fn price_cond(mut price_start: Option<i16>, mut price_end: Option<i16>) -> Condition {
    if price_start.is_some() && price_start == price_end {
        price_start = price_start.and_then(|v| Some(v-1));
        price_end = price_end.and_then(|v| Some(v+1))
    }
    
    let mut cond = Condition::all();
    if let Some(v) = price_start { cond = cond.add(products::Column::FinalPrice.gte(v)); };
    if let Some(v) = price_end { cond = cond.add(products::Column::FinalPrice.lte(v));};
    cond
}

pub async fn get_price_range(
    db: &DatabaseConnection,
    name: Option<String>,
    categories: Option<Vec<i32>>,
) -> (Decimal, Decimal) {
    let query = Products::find()
    .filter(name_cond(name))
    .filter(category_cond(categories))
    .select_only()
    .column_as(products::Column::FinalPrice.min(), "min")
    .column_as(products::Column::FinalPrice.max(), "max");

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Діапазон цін: {qstr}");

    query
    .into_tuple::<(Decimal, Decimal)>()
    .one(db)
    .await
    .unwrap_or(Some((Decimal::ZERO, Decimal::ZERO)))
    .expect("Помилка серіалізації запису")
}

#[derive(Serialize, FromQueryResult)]
pub struct ProdWithCatQueryRes {
    id: i32,
    name: String,
    description: Option<String>,
    price: Decimal,
    discount: Option<Decimal>,
    category: Option<String>,
    final_price: Decimal,
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<products::Model, BasicErr> {
    let query = Products::find_by_id(id);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Вибір товару: {qstr}");
    
    query
    .one(db)
    .await
    .map(|v| v.expect("Помилка десеріалізації моделі"))
    .map_err(|e| e.into())
}

pub async fn get_joined_all(
    db: &DatabaseConnection,
    name: Option<String>,
    price_start: Option<i16>,
    price_end: Option<i16>,
    categories: Option<Vec<i32>>,
) -> Vec<ProdWithCatQueryRes> {    
    let query = Products::find()
    .filter(name_cond(name))
    .filter(price_cond(price_start, price_end))
    .filter(category_cond(categories))
    .left_join(Categories)
    .select_only()
    .columns(vec![
        products::Column::Id,
        products::Column::Name,
        products::Column::Description,
        products::Column::Price,
        products::Column::Discount,
        products::Column::FinalPrice,
    ])
    .column_as(categories::Column::Name, "category")
    .order_by_asc(products::Column::Name);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Список товарів: {qstr}");

    query
    .into_model::<ProdWithCatQueryRes>()
    .all(db)
    .await.expect("DbErr")
}

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    description: Option<String>,
    price: f64,
    discount: Option<f64>,
    category: Option<i32>
) -> Result<products::Model, BasicErr> { 
    let Some(price) = Decimal::from_f64(price) else { return Err(BasicErr::Invalid); };
    let discount = match discount.map(|v| Decimal::from_f64(v)) {
        Some(None) => return Err(BasicErr::Invalid),
        Some(v) => v,
        None => None,
    };
    let mut model = products::ActiveModel {
        name: Set(name),
        description: Set(description),
        price: Set(price),
        ..Default::default()
    };

    discount.inspect(|v| { model.set(products::Column::Discount, (*v).into()); });
    category.inspect(|v| { model.set(products::Column::Category, (*v).into()); });

    let query = Products::insert(model.clone());
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Додавання товару: {qstr}");

    model.insert(db).await.map_err(|e| e.into())
}

pub async fn edit(
    db: &DatabaseConnection,
    id: i32,
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    discount: Option<Option<f64>>,
    category: Option<i32>
) -> Result<UpdateResult, BasicErr> { 
    let mut stmt = Products::update_many()
    .filter(products::Column::Id.eq(id));

    let price = match price.map(|v| Decimal::from_f64(v)) {
        Some(None) => return Err(BasicErr::Invalid),
        Some(v) => v,
        None => None,
    };
    let discount = if let Some(v) = discount {
        match v.map(|v| Decimal::from_f64(v)) {
            Some(None) => return Err(BasicErr::Invalid),
            Some(v) => v,
            None => None,
        }
    } else { None };

    if let Some(v) = name { stmt = stmt.col_expr(products::Column::Name, v.into()) };
    if let Some(v) = description { stmt = stmt.col_expr(products::Column::Description, v.into()) };
    if let Some(v) = price { stmt = stmt.col_expr(products::Column::Price, v.into()) };
    if let Some(v) = discount { stmt = stmt.col_expr(products::Column::Discount, v.into()) };
    if let Some(v) = category { 
        if v == -1 {
            stmt = stmt.col_expr(products::Column::Category, Expr::cust("NULL"));
        } else {
            stmt = stmt.col_expr(products::Column::Category, v.into())
        }
    };

    let qstr = stmt.as_query().to_string(MysqlQueryBuilder);
    println!("Оновлення товару: {qstr}");

    stmt.exec(db).await.map_err(|e| e.into())
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, BasicErr> {
    let query = Products::delete_by_id(id);
    
    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Видалення товару: {qstr}");

    query.exec(db).await.map_err(|e| e.into())
}