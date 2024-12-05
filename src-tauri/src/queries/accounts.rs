use crate::entities::{prelude::*, *};
use sea_orm::*;
use sea_query::{IntoCondition, MysqlQueryBuilder};
use serde::Serialize;
use sha256;

#[derive(Serialize)]
pub enum CreateErr {
    Exists,
    InvalidPassword,
    NotAuthorized,
}

pub async fn create(db: &DatabaseConnection, login: String, password: String, employee: Option<i32>) -> Result<accounts::ActiveModel, CreateErr> {
    println!("{password}");
    if password.len() < 6 { return Err(CreateErr::InvalidPassword) };

    let password = sha256::digest(password).to_ascii_uppercase();
    let model = accounts::ActiveModel {
        login: Set(login),
        password: Set(password),
        employee: Set(employee),
        ..Default::default()
    };

    let qstr = Accounts::insert(model.clone()).as_query().to_string(MysqlQueryBuilder);
    println!("Реєстрація: {qstr}");

    model.save(db).await.map_err(|_| CreateErr::Exists)
}

#[derive(FromQueryResult, Serialize)]
struct AccountInfoQueryRes {
    id: i32,
    login: String,
    #[serde(skip_deserializing)]
    password: String,
    employee_id: Option<i32>,
    surname: Option<String>,
    name: Option<String>,
    patronim: Option<String>,
    location_id: Option<i32>,
    location_name: Option<String>,
    can_manage_empl: Option<i8>,
    can_manage_products: Option<i8>,
    can_manage_categories: Option<i8>,
    can_sell_products: Option<i8>,
    can_manage_locations: Option<i8>,
}


#[derive(Serialize)]
pub struct Permissions {
    pub manage_empl: bool,
    pub manage_products: bool,
    pub manage_categories: bool,
    pub sell_products: bool,
    pub manage_locations: bool,
}

impl Permissions {
    pub fn new(manage_empl: bool, manage_products: bool, manage_categories: bool, sell_products: bool, manage_locations: bool) -> Permissions {
        Self { manage_empl, manage_products, manage_categories, sell_products, manage_locations }
    }

    pub fn from_i8(manage_empl: i8, manage_products: i8, manage_categories: i8, sell_products: i8, manage_locations: i8) -> Self {
        Self::new(manage_empl > 0, manage_products > 0, manage_categories > 0, sell_products > 0, manage_locations > 0)
    }
}

#[derive(Serialize)]
pub struct EmployeeInfo {
    id: i32,
    surname: String,
    name: String,
    patronim: String,
    location_id: i32,
    location_name: String,
    permissions: Permissions,
}

impl EmployeeInfo {
    pub fn new(id: i32, surname: String, name: String, patronim: String, location_id: i32, location_name: String, permissions: Permissions) -> Self {
        Self { id, surname, name, patronim, location_id, location_name, permissions }
    }
}

#[derive(Serialize)]
pub struct AccountInfo {
    id: i32,
    login: String,
    employee: Option<EmployeeInfo>,
    token: Option<String>,
}

impl AccountInfo {
    pub fn new(id: i32, login: String, employee: Option<EmployeeInfo>) -> Self {
        Self {id, login, employee, token: None}
    }

    pub async fn gen_token(&mut self, db: &DatabaseConnection) {
        let session = super::sessions::create(db, self.id).await;
        self.token = Some(session.token);
    }
}

impl From<AccountInfoQueryRes> for AccountInfo {
    fn from(value: AccountInfoQueryRes) -> Self {
        let employee = if let (
            Some(id),
            Some(surname),
            Some(name),
            Some(patronim),
            Some(location_id),
            Some(location_name),
            Some(manage_empl),
            Some(manage_products),
            Some(manage_categories),
            Some(sell_products),
            Some(manage_locations),
        ) = (
            value.employee_id,
            value.surname,
            value.name,
            value.patronim,
            value.location_id,
            value.location_name,
            value.can_manage_empl,
            value.can_manage_products,
            value.can_manage_categories,
            value.can_sell_products,
            value.can_manage_locations,
        ) {
            let permissions = Permissions::from_i8(manage_empl, manage_products, manage_categories, sell_products, manage_locations);
            Some(EmployeeInfo::new(id, surname, name, patronim, location_id, location_name, permissions))
        } else {
            None
        };

        Self::new(value.id, value.login, employee)
    }
}

#[derive(Debug, Serialize)]
pub enum LoginErr {
    NotFound,
    InvalidPassword,
    Internal
}

async fn login_query(db: &DatabaseConnection, filter: Condition) -> Result<AccountInfoQueryRes, LoginErr> {
    let query = Accounts::find()
    .filter(filter)
    .left_join(Employees)
    .join(JoinType::LeftJoin, <Employees as sea_orm::EntityTrait>::Relation::Positions.def())
    .join(JoinType::LeftJoin, <Positions as sea_orm::EntityTrait>::Relation::Locations.def())
    .select_only()
    .columns(vec![
        accounts::Column::Id,
        accounts::Column::Login,
        accounts::Column::Password,
    ])
    .column_as(employees::Column::Id, "employee_id")
    .columns(vec![
        employees::Column::Surname,
        employees::Column::Name,
        employees::Column::Patronim,
    ])
    .column_as(positions::Column::Location, "location_id")
    .column_as(locations::Column::Name, "location_name")
    .columns(vec![
        positions::Column::CanManageEmpl,
        positions::Column::CanManageProducts,
        positions::Column::CanManageCategories,
        positions::Column::CanSellProducts,
        positions::Column::CanManageLocations,
    ]);

    let qstr = query.as_query().to_string(MysqlQueryBuilder);
    println!("Авторизація: {qstr}");

    query
    .into_model::<AccountInfoQueryRes>()
    .one(db)
    .await
    .map(|v| v.expect("Помилка десеріалізації моделі"))
    .map_err(|e| {
        if let DbErr::RecordNotFound(_) = e { LoginErr::NotFound } else { eprintln!("{e}"); LoginErr::Internal }
    })
}

pub async fn login(db: &DatabaseConnection, login: String, password: String) -> Result<AccountInfo, LoginErr> {
    let account = login_query(db, accounts::Column::Login.eq(&login).into_condition()).await?;

    let password = sha256::digest(password).to_ascii_uppercase();
    if account.password != password { return Err(LoginErr::InvalidPassword); }

    let mut account_info: AccountInfo = account.into();
    account_info.gen_token(db).await;

    Ok(account_info)
}