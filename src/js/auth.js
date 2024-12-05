import * as api from './api.js'
import { showSection } from './main.js';
import * as shop from './shop.js'
import * as empl from './empl.js'

const header = document.getElementsByTagName("header")[0];
const regLoginDiv = document.getElementById("inp_reglogin");
const regPasswordDiv = document.getElementById("inp_regpass");
const logLoginDiv = document.getElementById("inp_loglogin");
const logPasswordDiv = document.getElementById("inp_logpass");

const btnMLocations = document.getElementById("btn-menu-locations");
const btnMPositions = document.getElementById("btn-menu-positions");
const btnMCategories = document.getElementById("btn-menu-categories");
const btnMProducts = document.getElementById("btn-menu-products");
const btnSell = document.getElementById("btn-sell");
const btnPoseditNew = document.getElementById("btn-posedit-new");

export let accountInfo = null;
let work_id = null;

function loadAccount() {
    if (accountInfo !== null) {
        document.getElementById("not-auth").remove();
    }

    let auth = document.createElement("button");
    auth.classList.add("auth");

    if (accountInfo.employee !== null) {
        let employee = accountInfo.employee;
        let snp = document.createElement("span");
        snp.classList.add("larger-text")
        snp.textContent = `${employee.surname} ${employee.name} ${employee.patronim}`;
        auth.append(snp);

        let location = document.createElement("span");
        location.classList.add("small-larger-text");
        location.textContent = `Локація: ${employee.location_name}`;
        auth.append(location);

        if (employee.permissions.manage_locations) { 
          btnMLocations.style.display = "block";
          btnPoseditNew.style.display = "block"
        }
        if (employee.permissions.manage_locations || employee.permissions.manage_empl) { btnMPositions.style.display = "block" }
        if (employee.permissions.manage_categories) { btnMCategories.style.display = "block" }
        if (employee.permissions.manage_products) { btnMProducts.style.display = "block" }
        if (employee.permissions.sell_products) { btnSell.style.display = "flex" }
    }

    auth.addEventListener('click', () => {
        showSection("menu")
    })

    header.append(auth)

    shop.load()
}

export async function showRegisterForm(id = null) {
  work_id = id;
  showSection("wdv-register")
};
  
async function register() {
    let login = regLoginDiv.value;
    let password = regPasswordDiv.value;

    if (login === "" || password === "") { alert("Деякі поля не є заповненими!"); return; }

    try {
        await api.register(accountInfo.token, login, password, work_id)

        regLoginDiv.value = ""
        regPasswordDiv.value = ""

        if (work_id === null) { loadAccount() } else {
          work_id = null; empl.loadEmpl()
        }
    } catch(e) {
        if (e === "Exists") {
        alert("Акаунт з таким логіном або емейлом вже існує!")
        } else if (e === "InvalidPassword") {
        alert("Пароль повинен складатися з 6 або більше символів!")
        } else {
        alert("Щось пішло не так.")
        }
    }
}

async function login() {
    let login = logLoginDiv.value;
    let password = logPasswordDiv.value;
  
    if (login === "" || password === "") { alert("Деякі поля не є заповненими!"); return; }
  
    try {
      accountInfo = await api.login(login, password)
      logLoginDiv.value = ""
      logPasswordDiv.value = ""
      loadAccount()
    } catch(e) {
      if (e === "NotFound") {
        alert("Акаунта з таким логіном не існує!");
      } else if (e === "InvalidPassword") {
        alert("Невірний пароль!")
      } else {
        alert("Щось пішло не так.")
      }
    }
}

export async function exit() {
  api.exit(accountInfo.token)
}

export async function debug_login(login, password) {
    try {
      accountInfo = await api.login(login, password)
      loadAccount()
    } catch(e) {
    }
  }

document.getElementById("btn-login").addEventListener('click', () => {
login()
})

document.getElementById("btn-register").addEventListener('click', () => {
register()
})