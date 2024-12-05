import { showSection } from './main.js'
import * as api from './api.js'
import * as auth from './auth.js';
import { createCheckbox, createRadio } from './utils.js';

let search = null
let work_id = null;

const locationsTitleDiv = document.getElementById("shop-locations-title") 
const locationsDiv = document.getElementById("shop-locations");
const categoriesDiv = document.getElementById("categories");
const minPriceDiv = document.getElementById("inp_price_min");
const maxPriceDiv = document.getElementById("inp_price_max");
const productsDiv = document.getElementById("products");
const prodedit_title = document.getElementById("prodedit-title");
const prodeditNameDiv = document.getElementById("inp-prodedit-name");
const prodeditDescDiv = document.getElementById("inp-prodedit-desc");
const prodeditPriceDiv = document.getElementById("inp-prodedit-price");
const prodeditDiscountDiv = document.getElementById("inp-prodedit-discount");
const prodeditCategoriesDiv = document.getElementById("prodedit-categories");
const productsControlsDiv = document.getElementById("products-controls");
const stockeditTitleDiv = document.getElementById("stockedit-title")
const stockeditDiv = document.getElementById("inp-stockedit")
const searchbox = document.getElementById("inp-search")
const cashregDiv = document.getElementById("cashreg")
const cashregTableDiv = document.getElementById("cashreg-products");
const cashregSumDiv = document.getElementById("cashreg-sum");

async function loadPrices() {
    const range = await api.products_price_range(search, getCheckedCategories());
  
    const mi = Math.floor(parseFloat(range[0]));
    minPriceDiv.placeholder = mi;
  
    const ma = Math.ceil(parseFloat(range[1]));
    maxPriceDiv.placeholder = ma;
  }

  async function loadLocations() {
    let locations = await api.locations()
    locationsDiv.innerHTML = "";
    let firstChecked = false;
    locations.forEach(location => {
      let empl_location_id = auth.accountInfo?.employee?.location_id;
      let can_manage_locations = auth.accountInfo?.employee?.can_manage_locations;
      let box = createRadio("shop-location", location.id, `${location.name} - ${location.address}`,
        !firstChecked || empl_location_id === location.id,
        empl_location_id !== undefined && can_manage_locations === false
      )
      firstChecked = true;
      if (empl_location_id !== null) {  }
      box.addEventListener("change", () => load())
      locationsDiv.append(box)
    })
  }

  async function loadCategories() {
    let categories = await api.categories()
    categoriesDiv.innerHTML = "";
    let chkbox = createCheckbox("category", -1, "Без категорії")
    chkbox.addEventListener("change", () => load())
    categoriesDiv.append(chkbox)
    categories.forEach(category => {
      let chkbox = createCheckbox("category", category.id, category.name)
      chkbox.addEventListener("change", () => load())
      categoriesDiv.append(chkbox)
    })
  }
  
  
  function getCheckedLocations() {
    const checked = document.querySelectorAll('input[name="shop-location"]:checked');
    return Array.from(checked).map(v => parseInt(v.value));
  }
  
  function getCheckedProdeditCategory() {
    return parseInt(document.querySelectorAll('input[name="prodedit-category"]:checked')[0].value);
  }
  
  
  function getCheckedCategories() {
    const checked = document.querySelectorAll('input[name="category"]:checked');
    return Array.from(checked).map(checkbox => parseInt(checkbox.value));
  }
  
  function createProductDiv(name, category, description, price, discount, final_price) {
    let productDiv = document.createElement("div");
  
    let nameDiv = document.createElement("h1");
    nameDiv.textContent = name;
    productDiv.appendChild(nameDiv);
  
    let hasDiscount = discount !== null && discount !== undefined && discount != 0.0;
  
    let priceDiv = document.createElement("div");
  
    let finalPrice = document.createElement("strong");
    finalPrice.classList.add("final-price")
    finalPrice.textContent = `${final_price} грн.`;
    priceDiv.appendChild(finalPrice);
    
    if (hasDiscount) {
      let discountInfo = document.createElement("span");
      discountInfo.textContent = ` (Знижка: ${discount}%)`;
      priceDiv.appendChild(discountInfo);
    
      let basePrice = document.createElement("div");
      basePrice.textContent = `Базова ціна: ${price} грн.`;
      basePrice.className = "base-price";
      priceDiv.appendChild(basePrice);
    }
    
    productDiv.appendChild(priceDiv);
    
  
    let categoryDiv = document.createElement("div");
    categoryDiv.classList.add("prod-category");
    categoryDiv.textContent = category || "Без категорії";
    productDiv.appendChild(categoryDiv);
  
    let desc = document.createElement("span");
    desc.textContent = description;
    productDiv.appendChild(desc);
  
    return productDiv;
  }

  function addToCart(id, name, price, discount, final_price, amount) {
    let div = document.createElement("div");
  
    let nameDiv = document.createElement("span");
    nameDiv.textContent = name;
    nameDiv.classList.add("cashreg-title")
    div.appendChild(nameDiv);
  
    let hasDiscount = discount !== null && discount !== undefined && discount != 0.0;
  
    let priceDiv = document.createElement("div");
  
    let finalPrice = document.createElement("strong");
    finalPrice.classList.add("final-price")
    finalPrice.textContent = `${final_price} грн.`;
    priceDiv.appendChild(finalPrice);
    
    if (hasDiscount) {
      let discountInfo = document.createElement("span");
      discountInfo.textContent = ` (Знижка: ${discount}%)`;
      priceDiv.appendChild(discountInfo);
    
      let basePrice = document.createElement("div");
      basePrice.textContent = `Базова ціна: ${price} грн.`;
      basePrice.className = "base-price";
      priceDiv.appendChild(basePrice);
    }
    
    div.appendChild(priceDiv);

    let controls = document.createElement("div");
    controls.classList.add("inline");

    let amountDiv = document.createElement("input");
    amountDiv.type = "number"
    amountDiv.max = amount
    amountDiv.min = 1
    amountDiv.value = 1
    amountDiv.addEventListener("change", () => recalculateSum())

    let delBtn = document.createElement("button");
    delBtn.innerText = "Прибрати";
    delBtn.addEventListener("click", () => {
      div.remove()
      recalculateSum()
    })

    controls.appendChild(amountDiv)
    controls.appendChild(delBtn)

    div.appendChild(controls)

    div.id = `cart-prod-${id}`
    div.dataset.id = id
    div.dataset.price = final_price
  
    cashregTableDiv.appendChild(div)
    recalculateSum()
  }
  
  export async function loadProducts() {
    cashregDiv.style.display = "none";
    showSection("shop")
  
    productsDiv.innerHTML = "";
    productsControlsDiv.innerHTML = "";
    locationsTitleDiv.style.display = "none";
    locationsDiv.style.display = "none";
  
    let btnNew = document.createElement("button");
    btnNew.classList.add("full-width-button")
    btnNew.addEventListener("click", () => showEditor());
    btnNew.innerText = "Додати новий продукт"
    productsControlsDiv.appendChild(btnNew)
  
    const price_start = parseInt(minPriceDiv.value);
    const price_end = parseInt(maxPriceDiv.value);
  
  
    if (price_start > price_end) {
      alert("Мінімальна ціна не може бути більшою за максимальну!")
      await loadPrices()
      await loadStock()
      return;
    }
  
  
    let products = await api.products(search, price_start, price_end, getCheckedCategories(), getCheckedLocations())
    
    products.forEach(element => {
      let productDiv = createProductDiv(element.name, element.category, element.description, element.price, element.discount, element.final_price);
  
      let editBtn = document.createElement("button");
      editBtn.innerText = "Редагувати"
      editBtn.addEventListener("click", () => showEditor(element.id))
  
      let locBtn = document.createElement("button");
      locBtn.innerText = "Ред. кількість на складі"
      locBtn.addEventListener("click", () => showStockEditor(element.id, element.name))
  
      let delBtn = document.createElement("button");
      delBtn.innerText = "Видалити"
      delBtn.addEventListener("click", async () => {
        await del(element.id)
        await loadProducts()
      })
  
      productDiv.appendChild(editBtn);
      productDiv.appendChild(locBtn);
      productDiv.appendChild(delBtn);
      productsDiv.appendChild(productDiv);
    });
  
    prodedit_is_on = true
  }
  
  async function loadStock() {
    cashregDiv.style.display = "flex";
    showSection("shop")
  
    productsDiv.innerHTML = "";
    productsControlsDiv.innerHTML = "";
    locationsTitleDiv.style.display = "flex";
    locationsDiv.style.display = "flex";

    const price_start = parseInt(minPriceDiv.value);
    const price_end = parseInt(maxPriceDiv.value);
  
  
    if (price_start > price_end) {
      alert("Мінімальна ціна не може бути більшою за максимальну!")
      await loadPrices()
      await loadStock()
      return;
    }
  
    let products = await api.stocks(auth.accountInfo?.token, search, price_start, price_end, getCheckedCategories(), getCheckedLocations())
    
    products.forEach(element => {
      let productDiv = createProductDiv(element.name, element.category, element.description, element.price, element.discount, element.final_price);
  
      if (element.amount == 0) { productDiv.classList.add("product-item-off") } else {
        let btn_add = document.createElement("button");
        btn_add.textContent = "Додати в кошик";
        btn_add.addEventListener("click", async () =>  {
          const cartDiv = document.getElementById(`cart-prod-${element.id}`);
          if (cartDiv === null || cartDiv === undefined) {
            await addToCart(element.id, element.name, element.price, element.discount, element.final_price, element.amount)
          } else {
            const cartAmount = cartDiv.getElementsByTagName("input")[0];
            cartAmount.value = parseInt(cartAmount.value) + 1;
            recalculateSum()
          }
        });
        productDiv.appendChild(btn_add); 
      }
  
      productsDiv.appendChild(productDiv);
    });
    prodedit_is_on = false
  }
  
  async function loadShop() {
    await loadLocations()
    await loadCategories()
    await loadPrices()
    await loadStock()
  }
  
  export async function loadProductsEditor() {
    await loadCategories()
    await loadPrices()
    await loadProducts()
  }
  
  let firstLoad = true;
  let prodedit_is_on = false;
  export async function load(_search = "", reset) {
    if (reset) { searchbox.value = ""; search = ""; firstLoad = true; prodedit_is_on = false; }
    search = _search;
    if (firstLoad) {
        if (prodedit_is_on) {
            loadProductsEditor()
        } else {
            loadShop()
        }
        firstLoad = false;
    } else {
        if (prodedit_is_on) {
            loadProducts()
        } else {
            loadStock()
        }
    }
  }


  //Editor

  async function showEditor(id = null) {
    prodeditNameDiv.value = ""
    prodeditDescDiv.value = ""
    prodeditPriceDiv.value = ""
    prodeditDiscountDiv.value = ""
  
    if (id !== null) {
      work_id = id;
      const product = await api.product(id);
      loadEditorCategories(product.category);
      prodeditNameDiv.placeholder = product.name
      prodeditDescDiv.placeholder = product.description
      prodeditPriceDiv.placeholder = product.price
      prodeditDiscountDiv.placeholder = product.discount
      prodedit_title.innerText = `Редагування продукту: ${product.name}`
    } else {
      work_id = null;
      loadEditorCategories();
      prodeditNameDiv.placeholder = ""
      prodeditDescDiv.placeholder = ""
      prodeditPriceDiv.placeholder = ""
      prodeditDiscountDiv.placeholder = ""
      prodedit_title.innerText = `Створення нового продукту`
    }
    showSection("wdv-product-form")
  }

  async function showStockEditor(id = null, name) {
    stockeditDiv.value = ""
  
    if (id !== null) {
      work_id = id;
      const stock = await api.stock(auth.accountInfo.token, auth.accountInfo.employee.location_id, id);
      //loadEditorCategories(stock.category);
      stockeditDiv.placeholder = stock
      stockeditTitleDiv.innerText = name ? `Редагування кількості продукту ${name} на складі` : `Редактор кількості продукту на складі`
      showSection("wdv-stock-form")
    }
  }

  async function save(id=null, name=null, desc=null, price=null, discount=null, category=null) {
    if (name === "") { name = null };
    if (desc === "") { desc = null } else if (desc === " ") { desc = "" };
    try {
      if (id === null) {
        await api.create_product(auth.accountInfo.token, name, desc, price, discount, category)
      } else {
        await api.edit_product(auth.accountInfo.token, id, name, desc, price, discount, category)
      }
      loadProducts()
    }  
    catch(e) {
      if (e === "Exists") {
        alert("Такий продукт вже існує!")
      } else if (e === "Not Found") {
        alert("Такого продукта вже не існує! Можливо вона була видалена кимось іншим під час редагування.")
      } else {
        alert("Щось пішло не так. Можливо така категорія вже існує або ви не заповнили всі поля у випадку додавання.")
      }
    }
  }

  async function del(id = null) {
    try {
      await api.delete_product(auth.accountInfo.token, id)
    }  
    catch(e) {
      if (e === "NotFound") {
        alert("Такий продукт не існує")
      } else {
        alert("Щось пішло не так.")
      }
    }
  }
  
  async function loadEditorCategories(checked_id = 0) {
  let categories = await api.categories()
  prodeditCategoriesDiv.innerHTML = "";
  let radiobox = createRadio("prodedit-category", -1, "Без категорії", true)
  prodeditCategoriesDiv.append(radiobox)
  categories.forEach(category => {
    let radiobox = createRadio("prodedit-category", category.id, category.name, checked_id === category.id)
    prodeditCategoriesDiv.append(radiobox)
  })
}

minPriceDiv.addEventListener("change", () => load())
maxPriceDiv.addEventListener("change", () => load())

document.getElementById("btn-prodedit-save").addEventListener("click", () => save(
  work_id, prodeditNameDiv.value, prodeditDescDiv.value, parseFloat(prodeditPriceDiv.value), parseFloat(prodeditDiscountDiv.value), getCheckedProdeditCategory()
))
document.getElementById("btn-prodedit-back").addEventListener("click", () => loadProductsEditor())

document.getElementById("btn-stockedit-save").addEventListener("click", async () => { 
  try {
    const amount = parseInt(stockeditDiv.value);
    if (Number.isInteger(amount)) { await api.save_stock(auth.accountInfo.token, auth.accountInfo.employee.location_id, work_id, amount) }
  } catch (e) {
    alert("Щось пішло не так!")
  }
  loadProducts()
})
document.getElementById("btn-stockedit-back").addEventListener("click", () => loadProductsEditor())

document.getElementById("btn-sell").addEventListener("click", async () => {
  let stocks = [];
  Array.from(cashregTableDiv.children).forEach(item => {
    stocks.push({id: parseInt(item.dataset.id), amount: parseInt(item.getElementsByTagName("input")[0].value)})
  })

  if (stocks.length === 0) { alert("Неможливо оформити пустий кошик!"); return; }

  try {
    await api.sell(auth.accountInfo.token, stocks)
    alert("Замовлення успішно оформлено!")
    cashregTableDiv.innerHTML = ""
    recalculateSum()
  } catch(e) {
    alert("Щось пішло не так. Вибачте за незручності.")
  }
  loadStock()
})
document.getElementById("btn-cart-clear").addEventListener("click", () => {
  cashregTableDiv.innerHTML = ""
  recalculateSum()
})

function recalculateSum() {
  let sum = 0.0;
  Array.from(cashregTableDiv.children).forEach(item => {
    sum += parseFloat(item.dataset.price) * parseInt(item.getElementsByTagName("input")[0].value);
  })
  cashregSumDiv.innerText = `${sum} грн.`
}