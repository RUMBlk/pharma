use crate::entities::{*, prelude::*};
use sea_orm::*;
use sea_query::{CaseStatement, Expr, OnConflict, MysqlQueryBuilder};
use sqlx::types::Decimal;
use serde::{Deserialize, Serialize};

use super::BasicErr;

pub fn location_cond(locations: Option<Vec<i32>>) -> Condition {
    let mut cond = Condition::any();
    if let Some(v) = locations {
        for location in v {
            cond = cond.add(stock::Column::Location.eq(location));
        };
    }
    cond
}

pub async fn get_amount(db: &DatabaseConnection, token: Option<String>, location: i32, product: i32) -> Result<i32, BasicErr> {
    if let Some(v) = token {
        super::sessions::can_manage_products(db, v).await
    } else { return Err(BasicErr::NotAuthorized) };

    let query = Stock::find()
    .filter(stock::Column::Location.eq(location))
    .filter(stock::Column::Product.eq(product))
    .select_only()
    .column(stock::Column::Amount);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Кількість товару на складі: {qstr}");


    query
    .into_tuple::<i32>()
    .one(db)
    .await
    .map(|v| v.unwrap_or(0))
    .inspect(|v| println!("{v:.?}"))
    .map_err(|e| BasicErr::from(e))
}

pub async fn get_amount_by_id(db: &DatabaseConnection, id: i32) -> Result<i32, BasicErr> {
    let query = Stock::find_by_id(id)
    .select_only()
    .column(stock::Column::Amount);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Кількість товару на складі за ID: {qstr}");

    query
    .into_tuple::<i32>()
    .one(db)
    .await
    .map(|e| e.expect("Помилка серіалізації запису"))
    .map_err(|e| e.into())
}

#[derive(FromQueryResult, Serialize)]
pub struct JoinedStockQueryRes {
    id: i32,
    location: i32,
    product: i32,
    amount: i32,
    name: String,
    description: Option<String>,
    price: Decimal,
    discount: Option<Decimal>,
    category: Option<String>,
    final_price: Decimal,
}

pub async fn get_all(
    db: &DatabaseConnection,
    token: Option<String>,
    name: Option<String>,
    price_start: Option<i16>,
    price_end: Option<i16>,
    categories: Option<Vec<i32>>,
    locations: Option<Vec<i32>>,
) -> Vec<JoinedStockQueryRes> {
    let can_manage_products = if let Some(v) = token {
        super::sessions::can_manage_products(db, v).await
    } else { false };

    let query = Stock::find()
    .left_join(Products)
    .join(JoinType::LeftJoin,<Products as sea_orm::EntityTrait>::Relation::Categories.def())
    .filter(super::products::name_cond(name))
    .filter(super::products::price_cond(price_start, price_end))
    .filter(super::products::category_cond(categories))
    .filter(location_cond(locations))
    .select_column_as(products::Column::Id, "product")
    .columns(vec!(
        products::Column::Name,
        products::Column::Description,
        products::Column::Price,
        products::Column::Discount,
        products::Column::FinalPrice,
    ))
    .order_by_asc(
        Expr::expr(CaseStatement::new()
            .case(Expr::col((Stock, stock::Column::Amount)).eq(0), true)
            .finally(false))
    )
    .order_by_asc(products::Column::Name)
    .select_column_as(categories::Column::Name, "category");

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Список товару на складі локації: {qstr}");

    let mut qres = query
    .into_model::<JoinedStockQueryRes>()
    .all(db)
    .await.expect("DbErr");

    if !can_manage_products { 
        for v in &mut qres {
            v.amount = if v.amount < 1 { 0 } else { 1 };
        };
    };

    qres
}

pub async fn save(db: &DatabaseConnection, location: i32, product: i32, amount: i32) -> Result<InsertResult<stock::ActiveModel>, BasicErr> {
    let model = stock::ActiveModel {
        location: Set(location),
        product: Set(product),
        amount: Set(amount),
        ..Default::default()
    };

    let query = Stock::insert(model).on_conflict(
        OnConflict::columns(vec![stock::Column::Location, stock::Column::Product])
        .value(stock::Column::Amount, amount)
        .to_owned()
    );

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Збереження кількості товару на складі локації: {qstr}");
    
    query.exec(db).await
    .map_err(|e| e.into())
}

#[derive(Serialize, Deserialize)]
pub struct BuyAmount {
    id: i32,
    amount: i32,
}

pub async fn buy(db: &DatabaseConnection, stocks: Vec<BuyAmount>) -> Result<(), BasicErr> {
    let txn = db.begin().await.map_err(|e| BasicErr::from(e))?;

    for stock in stocks {
        if get_amount_by_id(db, stock.id).await? < stock.amount {
            let _ = txn.rollback().await;
            return Err(BasicErr::Invalid)
        }

        let query = Stock::update_many()
        .filter(stock::Column::Id.eq(stock.id))
        .col_expr(stock::Column::Amount, Expr::col(stock::Column::Amount).sub(1));

        let qstr = query.as_query().to_string(MysqlQueryBuilder);
        println!("Оформлення замовлення: {qstr}");

        if let Err(e) = query.exec(&txn).await {
            let _ = txn.rollback().await;
            return Err(e.into())
        }
    }

    txn.commit().await.map_err(|e| e.into())
}