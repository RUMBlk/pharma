import * as auth from './auth.js';
import * as shop from './shop.js';
import * as locations from './locations.js'
import * as categories from './categories.js';
import * as positions from './positions.js';
import * as api from './api.js'

const main = document.getElementsByTagName("main")[0];
const footer = document.getElementsByTagName("footer")[0];
const searchbox = document.getElementById("inp-search")
const searchbtn = document.getElementById("btn-search");
window.addEventListener("DOMContentLoaded", () => {

  document.getElementById("show-login").addEventListener('click', () => {
    showSection("wdv-login")
  })

  document.getElementById("mainpage").addEventListener("click", () => {
    shop.load(searchbox.value, true)
  })

  searchbtn.addEventListener('click', () => {
    searchbox.enabled = true
    searchbtn.enabled = true
    if (currentSection === "menu-locations") {
      locations.load(searchbox.value)
    } else if (currentSection === "menu-positions") {
      positions.load(searchbox.value)
    } else if (currentSection === "menu-categories") {
      categories.load(searchbox.value)
    } else {
      shop.load(searchbox.value)
    }
  })

  document.getElementById("btn-menu-locations").addEventListener('click', () => locations.load())
  document.getElementById("btn-menu-positions").addEventListener('click', () => positions.load())
  document.getElementById("btn-menu-categories").addEventListener('click', () => categories.load())
  document.getElementById("btn-menu-products").addEventListener('click', () => shop.loadProductsEditor())
  document.getElementById("btn-menu-exit").addEventListener('click', async () => {
    await auth.exit();
    location.reload();
  })

  main.style.display = "flex";
  shop.load("", true)
  //auth.debug_login("owner", "B0$$25")
});

let currentSection = null;
export function showSection(section) {
  let sections = main.getElementsByTagName("section");
  
  if (currentSection !== null) {
    let prev = document.getElementById(currentSection);

    prev.classList.add("unactive-main-section")
  } else {
    for(let i=0; i<sections.length; i++) {
      sections[i].classList.add("unactive-main-section")
    }
  }
  currentSection = section

  if (currentSection === "menu-empl" || currentSection.includes("form")) {
    searchbox.disabled = true
    searchbtn.disabled = true
  } else {
    searchbox.disabled = false
    searchbtn.disabled = false
  }

  if (currentSection === "shop") {
    footer.innerText = "Товари"
  } else if (currentSection === "menu-positions") {
    footer.innerText = "Вакансії"
  } else if (currentSection === "menu-empl") {
    footer.innerText = "Працівники"
  } else if (currentSection === "wdv-login") {
    footer.innerText = "Авторизація"
  } else if (currentSection === "wdv-register") {
    footer.innerText = "Реєстрація"
  } else if (currentSection === "menu") {
    footer.innerText = "Меню"
  } else if (currentSection === "menu-locations") {
    footer.innerText = "Локації"
  } else if (currentSection === "menu-categories") {
    footer.innerText = "Категорії"
  } else if (currentSection === "wdv-position-form") {
    footer.innerText = "Редактор вакансії"
  } else if (currentSection === "wdv-empl-form") {
    footer.innerText = "Редактор працівника"
  } else if (currentSection === "wdv-category-form") {
    footer.innerText = "Редактор категорії"
  } else if (currentSection === "wdv-product-form") {
    footer.innerText = "Редактор продукту"
  } else if (currentSection === "wdv-stock-form") {
    footer.innerText = "Редактор кількості продукту на складі"
  } else {
    footer.innerText = "Pharma"
  }

  document.getElementById(section).classList.remove("unactive-main-section");
} 