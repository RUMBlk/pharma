use sea_orm::{prelude::Decimal, DatabaseConnection};
use tauri::State;

mod entities;
mod queries;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn register(
    db: State<'_, DatabaseConnection>,
    token: String,
    login: String,
    password: String,
    employee: Option<i32>
) -> Result<(), queries::accounts::CreateErr> {
    if !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::accounts::CreateErr::NotAuthorized) }
    queries::accounts::create(&db, login, password, employee).await.map(|_| ())
}

#[tauri::command]
async fn login(
    db: State<'_, DatabaseConnection>,
    login: String,
    password: String
) -> Result<queries::accounts::AccountInfo, queries::accounts::LoginErr> {
    queries::accounts::login(&db, login, password).await
}

#[tauri::command]
async fn categories(db: State<'_, DatabaseConnection>, name: Option<String>) -> Result<Vec<entities::categories::Model>, ()> {
    Ok(queries::categories::get_all(&db, name).await.into())
}

#[tauri::command]
async fn locations(db: State<'_, DatabaseConnection>, name: Option<String>) -> Result<Vec<entities::locations::Model>, ()> {
    Ok(queries::locations::get_all(&db, name).await.into())
}

#[tauri::command]
async fn products(
    db: State<'_, DatabaseConnection>,
    name: Option<String>,
    price_start: Option<i16>,
    price_end: Option<i16>,
    categories: Option<Vec<i32>>
) -> Result<Vec<queries::products::ProdWithCatQueryRes>, ()> {
    Ok(queries::products::get_joined_all(&db, name, price_start, price_end, categories).await.into())
}

#[tauri::command]
async fn stocks(
    db: State<'_, DatabaseConnection>,
    token: Option<String>,
    name: Option<String>,
    price_start: Option<i16>,
    price_end: Option<i16>,
    categories: Option<Vec<i32>>,
    locations: Option<Vec<i32>>,
) -> Result<Vec<queries::stock::JoinedStockQueryRes>, ()> {
    Ok(queries::stock::get_all(&db, token, name, price_start, price_end, categories, locations).await.into())
}

#[tauri::command]
async fn stock(
    db: State<'_, DatabaseConnection>,
    token: Option<String>,
    location: i32,
    product: i32,
) -> Result<i32, queries::BasicErr> {
    queries::stock::get_amount(&db, token, location, product).await.into()
}


#[tauri::command]
async fn products_price_range(db: State<'_, DatabaseConnection>, name: Option<String>, categories: Option<Vec<i32>>) -> Result<(Decimal, Decimal), ()> {
    Ok(queries::products::get_price_range(&db, name, categories).await.into())
}

#[tauri::command]
async fn create_category(db: State<'_, DatabaseConnection>, token: String, name: String) -> Result<entities::categories::Model, queries::BasicErr> {
    if !queries::sessions::can_manage_categories(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::categories::create(&db, name).await
}

#[tauri::command]
async fn edit_category(db: State<'_, DatabaseConnection>, token: String, id: i32, name: String) -> Result<entities::categories::Model, queries::BasicErr> {
    if !queries::sessions::can_manage_categories(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::categories::edit(&db, id, name).await
}

#[tauri::command]
async fn delete_category(db: State<'_, DatabaseConnection>, token: String, id: i32) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_categories(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::categories::delete(&db, id).await
    .map(|_| ())
}

#[tauri::command]
async fn category(db: State<'_, DatabaseConnection>, id: i32) -> Result<entities::categories::Model, queries::BasicErr> {
    queries::categories::get_by_id(&db, id).await
}

#[tauri::command]
async fn create_product(
    db: State<'_, DatabaseConnection>,
    token: String,
    name: String,
    description: Option<String>,
    price: f64,
    discount: Option<f64>,
    category: Option<i32>
) -> Result<entities::products::Model, queries::BasicErr> {
    if !queries::sessions::can_manage_products(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::products::create(&db, name, description, price, discount, category).await
}

#[tauri::command]
async fn edit_product(    
    db: State<'_, DatabaseConnection>,
    token: String,
    id: i32,
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    discount: Option<Option<f64>>,
    category: Option<i32>
) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_products(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::products::edit(&db, id, name, description, price, discount, category).await.map(|_| ())
}

#[tauri::command]
async fn delete_product(db: State<'_, DatabaseConnection>, token: String, id: i32) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_products(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::products::delete(&db, id).await.map(|_| ())
}

#[tauri::command]
async fn product(db: State<'_, DatabaseConnection>, id: i32) -> Result<entities::products::Model, queries::BasicErr> {
    queries::products::get_by_id(&db, id).await
}

#[tauri::command]
async fn save_stock(db: State<'_, DatabaseConnection>, location: i32, product: i32, amount: i32) -> Result<(), queries::BasicErr> {
    queries::stock::save(&db, location, product, amount).await.map(|_| ())
}

#[tauri::command]
async fn location(db: State<'_, DatabaseConnection>, id: i32) -> Result<entities::locations::Model, queries::BasicErr> {
    queries::locations::get_by_id(&db, id).await
}

#[tauri::command]
async fn create_location(db: State<'_, DatabaseConnection>, token: String, name: String, address: String) -> Result<entities::locations::Model, queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::locations::create(&db, name, address).await
}

#[tauri::command]
async fn edit_location(db: State<'_, DatabaseConnection>, token: String, id: i32, name: Option<String>, address: Option<String>) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::locations::edit(&db, id, name, address).await.map(|_| ())
}

#[tauri::command]
async fn delete_location(db: State<'_, DatabaseConnection>, token: String, id: i32) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::locations::delete(&db, id).await
    .map(|_| ())
}

#[tauri::command]
async fn positions(
    db: State<'_, DatabaseConnection>,
    token: String,
    name: Option<String>,
    salary_start: Option<i32>,
    salary_end: Option<i32>,
    locations: Option<Vec<i32>>,

) -> Result<Vec<entities::positions::Model>, queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token.clone()).await 
        && !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    Ok(queries::positions::get_all(&db, name, salary_start, salary_end, locations).await.into())
}

#[tauri::command]
async fn position(db: State<'_, DatabaseConnection>, token: String, id: i32) -> Result<entities::positions::Model, queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token.clone()).await 
        && !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::positions::get_by_id(&db, id).await
}

#[tauri::command]
async fn positions_salary_range(db: State<'_, DatabaseConnection>,
    token: String,
    name: Option<String>,
    locations: Option<Vec<i32>>,
) -> Result<(Decimal, Decimal), queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token.clone()).await 
        && !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    Ok(queries::positions::get_salary_range(&db, name, locations).await.into())
}

#[tauri::command]
async fn create_position(db: State<'_, DatabaseConnection>, token: String, name: String, salary: f64, location: i32,
    can_manage_empl: Option<bool>, can_manage_products: Option<bool>, can_manage_categories: Option<bool>, can_sell_products: Option<bool>
) -> Result<entities::positions::Model, queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::positions::create(&db, name, salary, location,
        can_manage_empl, can_manage_products, can_manage_categories, can_sell_products, Some(false)).await
}

#[tauri::command]
async fn edit_position(db: State<'_, DatabaseConnection>, token: String, id: i32, name: Option<String>, salary: Option<f64>, location: Option<i32>,
    can_manage_empl: Option<bool>, can_manage_products: Option<bool>, can_manage_categories: Option<bool>, can_sell_products: Option<bool>
) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::positions::edit(&db, id, name, salary, location,
        can_manage_empl, can_manage_products, can_manage_categories, can_sell_products, Some(false)).await
        .map(|_| ())
}

#[tauri::command]
async fn delete_position(db: State<'_, DatabaseConnection>, token: String, id: i32) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::positions::delete(&db, id).await
    .map(|_| ())
}

#[tauri::command]
async fn empl_amount(db: State<'_, DatabaseConnection>, token: String, position: i32) -> Result<i32, queries::BasicErr> {
    if !queries::sessions::can_manage_locations(&db, token.clone()).await 
        && !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    Ok(queries::empl::empl_amount(&db, position).await)
}


#[tauri::command]
async fn employees(
    db: State<'_, DatabaseConnection>,
    token: String,
    name: Option<String>,
    position: i32,

) -> Result<Vec<entities::employees::Model>, queries::BasicErr> {
    if !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::empl::get_all(&db, name, position).await
}

#[tauri::command]
async fn employee(db: State<'_, DatabaseConnection>, token: String, id: i32) -> Result<entities::employees::Model, queries::BasicErr> {
    if !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::empl::get_by_id(&db, id).await
}


#[tauri::command]
async fn create_empl(
    db: State<'_, DatabaseConnection>,
    token: String,
    surname: String,
    name: String,
    patronim: String,
    position: i32,
    salary_bonus: Option<f64>,
) -> Result<entities::employees::Model, queries::BasicErr> {
    if !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::empl::create(&db, surname, name, patronim, position, salary_bonus).await
}

#[tauri::command]
async fn edit_empl(
    db: State<'_, DatabaseConnection>,
    token: String,
    id: i32,
    surname: Option<String>,
    name: Option<String>,
    patronim: Option<String>,
    position: Option<i32>,
    salary_bonus: Option<f64>,
) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::empl::edit(&db, id, surname, name, patronim, position, salary_bonus).await
        .map(|_| ())
}

#[tauri::command]
async fn delete_empl(db: State<'_, DatabaseConnection>, token: String, id: i32) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_manage_empl(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::empl::delete(&db, id).await
    .map(|_| ())
}

#[tauri::command]
async fn buy(db: State<'_, DatabaseConnection>, token: String, stocks: Vec<queries::stock::BuyAmount>) -> Result<(), queries::BasicErr> {
    if !queries::sessions::can_sell_products(&db, token).await { return Err(queries::BasicErr::NotAuthorized) }
    queries::stock::buy(&db, stocks).await.map(|_| ())
}

#[tauri::command]
async fn exit(db: State<'_, DatabaseConnection>, token: String) -> Result<(), queries::BasicErr> {
    queries::sessions::invalidate(&db, token).await.map(|_| ())
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(db: DatabaseConnection) {
    tauri::Builder::default()
        .manage(db)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            login,
            register,
            products,
            categories,
            products_price_range,
            stock,
            stocks,
            locations,
            create_category,
            edit_category,
            delete_category,
            category,
            create_product,
            edit_product,
            delete_product,
            product,
            save_stock,
            location,
            create_location,
            edit_location,
            delete_location,
            positions,
            position,
            positions_salary_range,
            create_position,
            edit_position,
            delete_position,
            empl_amount,
            employees,
            employee,
            create_empl,
            edit_empl,
            delete_empl,
            buy,
            exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
