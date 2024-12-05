const { invoke } = window.__TAURI__.core;

export async function login(login, password) {
    return await invoke("login", {
        login: login,
        password: password
      })
}

export async function register(token, login, password, employee) {
    return await invoke("register", {
        token: token,
        login: login,
        password: password,
        employee: employee
      })
}

export async function locations(name=null) {
    return await invoke("locations", {name: name});
}

export async function categories(name=null) {
    return await invoke("categories", {name: name});
}

export async function category(id) {
    return await invoke("category", {id: id})
}

export async function product(id) {
    return await invoke("product", {id: id})
}

export async function create_category(token, name) {
    return await invoke("create_category", {token: token, name: name})
}

export async function edit_category(token, id, name) {
    return await invoke("edit_category", {token: token, id: id, name: name})
}

export async function delete_category(token, id) {
    return await invoke("delete_category", {token: token, id: id})
}

export async function products(name, priceStart, priceEnd, categories, locations) {
    const args = {name: name, priceStart: priceStart, priceEnd: priceEnd, categories: categories, locations: locations};
    return await invoke("products", args)
}

export async function stock(token, location, product) {
    return await invoke("stock", {token: token, location: location, product: product})
}

export async function stocks(token, name, priceStart, priceEnd, categories, locations) {
    const args = {token: token, name: name, priceStart: priceStart, priceEnd: priceEnd, categories: categories, locations: locations};
    return await invoke("stocks", args)
}

export async function products_price_range(name, categories) {
    return await invoke("products_price_range", {name: name, categories: categories});
}

export async function create_product(token, name, description, price, discount, category) {
    return await invoke("create_product", {token:token, name: name, description: description, price: price, discount: discount, category: category})
}

export async function edit_product(token, id, name, description, price, discount, category) {
    return await invoke("edit_product", {token:token, id: id, name: name, description: description, price: price, discount: discount, category: category})
}

export async function delete_product(token, id) {
    return await invoke("delete_product", {token:token, id: id})
}

export async function save_stock(token, location, product, amount) {
    return await invoke("save_stock", {token:token, location: location, product: product, amount: amount})
}

export async function location(id) {
    return await invoke("location", {id: id});
}

export async function create_location(token, name, addr) {
    return await invoke("create_location", {token: token, name: name, address: addr})
}

export async function edit_location(token, id, name, addr) {
    return await invoke("edit_location", {token: token, id: id, name: name, address: addr})
}

export async function delete_location(token, id) {
    return await invoke("delete_location", {token: token, id: id})
}

export async function position(token, id) {
    return await invoke("position", {token: token, id: id});
}

export async function positions(token, name, salary_start, salary_end, locations) {
    return await invoke("positions", {token: token, name: name, salaryStart: salary_start, salaryEnd: salary_end, locations: locations})
}

export async function positions_salary_range(token, name, locations) {
    return await invoke("positions_salary_range", {token: token, name: name, locations: locations})
}

export async function create_position(token, name, salary, location, can_manage_empl, can_manage_products, can_manage_categories, can_sell_products) {
    return await invoke("create_position", {token: token, name: name, salary: salary, location: location,
        canManageEmpl: can_manage_empl, canManageProducts: can_manage_products, canManageCategories: can_manage_categories, canSellProducts: can_sell_products})
}

export async function edit_position(token, id, name, salary, location, can_manage_empl, can_manage_products, can_manage_categories, can_sell_products) {
    return await invoke("edit_position", {token: token, id: id, name: name, salary: salary, location: location,
        canManageEmpl: can_manage_empl, canManageProducts: can_manage_products, canManageCategories: can_manage_categories, canSellProducts: can_sell_products
    })
}

export async function delete_position(token, id) {
    return await invoke("delete_position", {token: token, id: id})
}

export async function empl_amount(token, position) {
    return await invoke("empl_amount", {token: token, position: position})
}

export async function employees(token, name, position) {
    return await invoke("employees", {token: token, name: name, position: position})
}

export async function employee(token, id) {
    return await invoke("employee", {token: token, id: id})
}

export async function create_empl(token, surname, name, patronim, position, salary_bonus) {
    return await invoke("create_empl", {token: token, surname: surname, name: name, patronim: patronim, position: position, salary_bonus: salary_bonus})
}

export async function edit_empl(token, id, surname, name, patronim, position, salary_bonus) {
    return await invoke("edit_empl", {token: token, id: id, surname: surname, name: name, patronim: patronim, position: position, salaryBonus: salary_bonus})
}

export async function delete_empl(token, id) {
    return await invoke("delete_empl", {token: token, id: id})
}

export async function sell(token, stocks) {
    return await invoke("buy", {token: token, stocks: stocks})
}

export async function exit(token) {
    return await invoke("exit", {token: token})
}
