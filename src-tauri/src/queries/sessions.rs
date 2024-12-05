use crate::entities::{*, prelude::*};
use sea_orm::*;
use sea_query::{Expr, ExprTrait, MysqlQueryBuilder};

pub async fn create(db: &DatabaseConnection, account_id: i32) -> sessions::Model {
    let model = sessions::ActiveModel {
        account: Set(account_id),
        ..Default::default()
    };

    let qstr = Sessions::insert(model.clone()).as_query().to_string(MysqlQueryBuilder);
    println!("Ініціалізація сесії: {qstr}");

    model.insert(db).await.expect("DbErr")
}

pub async fn invalidate(db: &DatabaseConnection, token: String) -> Result<DeleteResult, super::BasicErr> {
    let query = Sessions::delete_many()
        .filter(sessions::Column::Token.eq(token));

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Вихід з акаунту: {qstr}");

    query.exec(db)
    .await
    .map_err(|e| e.into())
}

pub async fn can(db: &DatabaseConnection,token: String, col: positions::Column) -> bool {
    let query = Sessions::find()
    .filter(sessions::Column::Token.eq(token))
    .filter(Expr::cust("NOW()").lt(sessions::Column::ExpiresAt.into_expr()))
    .inner_join(Accounts)
    .join(JoinType::LeftJoin, <Accounts as sea_orm::EntityTrait>::Relation::Employees.def())
    .join(JoinType::LeftJoin, <Employees as sea_orm::EntityTrait>::Relation::Positions.def())
    .select_only()
    .column(col);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Вибір наявності дозволу: {qstr}");

    query
    .into_tuple::<i8>()
    .one(db)
    .await
    .unwrap_or(Some(0))
    .unwrap_or(0) > 0
}

pub async fn can_manage_empl(db: &DatabaseConnection, token: String) -> bool {
    can(db, token, positions::Column::CanManageEmpl).await
}

pub async fn can_manage_products(db: &DatabaseConnection, token: String) -> bool {
    can(db, token, positions::Column::CanManageProducts).await
}

pub async fn can_manage_categories(db: &DatabaseConnection, token: String) -> bool {
    can(db, token, positions::Column::CanManageCategories).await
}

pub async fn can_sell_products(db: &DatabaseConnection, token: String) -> bool {
    can(db, token, positions::Column::CanSellProducts).await
}

pub async fn can_manage_locations(db: &DatabaseConnection, token: String) -> bool {
    can(db, token, positions::Column::CanManageLocations).await
}
