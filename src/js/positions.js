import { showSection } from './main.js'
import * as api from './api.js'
import * as auth from './auth.js';
import * as empl from './empl.js';
import { createCheckbox, createRadio } from './utils.js';

let search = null
let work_id = null;

const locationsDiv = document.getElementById("posedit-locations");
const minSalaryDiv = document.getElementById("pos_salary_min");
const maxSalaryDiv = document.getElementById("pos_salary_max");
const tableDiv = document.getElementById("posedit-table");
const posedit_title = document.getElementById("posedit-title");
const poseditNameDiv = document.getElementById("inp-posedit-name");
const poseditSalaryDiv = document.getElementById("inp-posedit-salary");
const cme = document.getElementById("posedit-cme");
const cmp = document.getElementById("posedit-cmp");
const cmc = document.getElementById("posedit-cmc");
const csp = document.getElementById("posedit-csp");
const cml = document.getElementById("posedit-cml");

async function loadSalary() {
    const range = await api.positions_salary_range(auth.accountInfo.token, search, getCheckedLocations());
  
    const mi = Math.floor(parseFloat(range[0]));
    minSalaryDiv.placeholder = mi;
  
    const ma = Math.ceil(parseFloat(range[1]));
    maxSalaryDiv.placeholder = ma;
  }

  async function loadLocations() {
    let locations = await api.locations()
    locationsDiv.innerHTML = "";
    let firstChecked = false;
    locations.forEach(location => {
      let empl_location_id = auth.accountInfo?.employee?.location_id;
      let can_manage_locations = auth.accountInfo?.employee?.can_manage_locations;
      let box = createRadio("posedit-location", location.id, `${location.name} - ${location.address}`,
        !firstChecked || empl_location_id === location.id,
        empl_location_id !== undefined && can_manage_locations === false)
      firstChecked = true;
      if (empl_location_id !== null) {  }
      box.addEventListener("change", () => loadPos())
      locationsDiv.append(box)
    })
  }
  
  export function getCheckedLocations() {
    const checked = document.querySelectorAll('input[name="posedit-location"]:checked');
    return Array.from(checked).map(v => parseInt(v.value));
  }
  
  function createPosDiv(name, salary, cme, cmp, cmc, csp, cml, empl_amount) {
    let div = document.createElement("div");
  
    let nameDiv = document.createElement("h1");
    nameDiv.textContent = name;
    div.appendChild(nameDiv);
  
    let salaryDiv = document.createElement("div");
  
    let salaryText = document.createElement("strong");
    salaryText.classList.add("final-price")
    salaryText.textContent = `Зарплата: ${salary} грн.`;
    salaryDiv.appendChild(salaryText);
    
    div.appendChild(salaryDiv);
    
  
    let permsDiv = document.createElement("div");
    permsDiv.classList.add("form-fields");
    permsDiv.classList.add("green")
    permsDiv.classList.add("filter-form");

    let permsTitleDiv = document.createElement("h3");
    permsTitleDiv.innerText="Дозволи:"
    permsDiv.append(permsTitleDiv)

    permsDiv.append(createCheckbox("pos-item-perm", null, "Редагування працівників", cme, true))
    permsDiv.append(createCheckbox("pos-item-perm", null, "Редагування продуктів", cmp, true))
    permsDiv.append(createCheckbox("pos-item-perm", null, "Редагування категорій", cmc, true))
    permsDiv.append(createCheckbox("pos-item-perm", null, "Продаж продуктів", csp, true))
    permsDiv.append(createCheckbox("pos-item-perm", null, "Редагування локацій", cml, true))

    div.appendChild(permsDiv);

    let empl_counter = document.createElement("span");
    empl_counter.classList.add("larger-text") 
    empl_counter.innerText = `Працівників: ${empl_amount}`

    div.appendChild(empl_counter)
  
    return div;
  }
  
  export async function loadPos() {
    showSection("menu-positions")
  
    tableDiv.innerHTML = "";

    const salary_start = parseInt(minSalaryDiv.value);
    const salary_end = parseInt(maxSalaryDiv.value);
  
  
    if (salary_start > salary_end) {
      alert("Мінімальна зарплата не може бути більшою за максимальну!")
      await loadSalary()
      await loadPos()
      return;
    }

    let positions = await api.positions(auth.accountInfo.token, search, salary_start, salary_end, getCheckedLocations())
    
    const promises = positions.map(async element => {
      let empl_amount = await api.empl_amount(auth.accountInfo.token, element.id);
      let div = createPosDiv(element.name, element.salary, element.can_manage_empl, element.can_manage_products,
        element.can_manage_categories, element.can_sell_products, element.can_manage_locations, empl_amount);
  
      let emplBtn = document.createElement("button");
      emplBtn.innerText = "Працівники"
      emplBtn.addEventListener("click", async () => await empl.load(element.id))
      div.appendChild(emplBtn);

      if(auth.accountInfo.employee.permissions.manage_locations === true) {
        let editBtn = document.createElement("button");
        editBtn.innerText = "Редагувати"
        editBtn.addEventListener("click", () => showEditor(element.id))

        let delBtn = document.createElement("button");
        delBtn.innerText = "Видалити"
        delBtn.addEventListener("click", async () => {
          await del(element.id)
          await loadPos()
        })

        div.appendChild(editBtn);
        div.appendChild(delBtn);
      }
      
      return div
    });

    const divs = await Promise.all(promises);

    divs.forEach(div => {
      tableDiv.appendChild(div);
    })

  }
  
  export async function load(name = null) {
    search = name
    await loadLocations()
    await loadSalary()
    await loadPos()
}


  //Editor

  async function showEditor(id = null) {
    poseditNameDiv.value = ""
    poseditSalaryDiv.value = ""
    cme.checked = false
    cmp.checked = false
    cmc.checked = false
    csp.checked = false
    cml.checked = false

    if (id !== null) {
      work_id = id;
      const position = await api.position(auth.accountInfo.token, id);
      poseditNameDiv.placeholder = position.name
      poseditSalaryDiv.placeholder = position.salary
      cme.checked = parseInt(position.can_manage_empl) > 0
      cmp.checked = parseInt(position.can_manage_products) > 0
      cmc.checked = parseInt(position.can_manage_categories) > 0
      csp.checked = parseInt(position.can_sell_products) > 0
      cml.checked = parseInt(position.can_manage_locations) > 0
      posedit_title.innerText = `Редагування посади: ${position.name}`
    } else {
      work_id = null;
      poseditNameDiv.placeholder = ""
      poseditSalaryDiv.placeholder = ""
      cme.checked = false
      cmp.checked = false
      cmc.checked = false
      csp.checked = false
      cml.checked = false
      posedit_title.innerText = `Створення нової посади`
    }
    cml.disabled = true
    showSection("wdv-position-form")
  }

  async function save(id=null, name=null, salary=null, location=null, cme=null, cmp=null, cmc=null, csp=null) {
    if (name === "") { name = null };
    try {
      if (id === null) {
        await api.create_position(auth.accountInfo.token, name, salary, location, cme, cmp, cmc, csp)
      } else {
        await api.edit_position(auth.accountInfo.token, id, name, salary, location, cme, cmp, cmc, csp)
      }
    }  
    catch(e) {
      if (e === "Exists") {
        alert("Така посада вже існує!")
      } else if (e === "Not Found") {
        alert("Такої позиції вже не існує! Можливо вона була видалена кимось іншим під час редагування.")
      } else {
        alert("Щось пішло не так. Можливо така посада вже існує або ви не заповнили всі поля у випадку додавання.")
      }
    }
  }

  async function del(id = null) {
    try {
      await api.delete_position(auth.accountInfo.token, id)
    }  
    catch(e) {
      if (e === "NotFound") {
        alert("Такої посади не існує")
      } else {
        alert("Щось пішло не так. Можливо ця посада використовується робітниками, тому її неможливо видалити.")
      }
    }
  }

minSalaryDiv.addEventListener("change", () => loadPos())
maxSalaryDiv.addEventListener("change", () => loadPos())

document.getElementById("btn-posedit-save").addEventListener("click", async () => {
  let location = getCheckedLocations()[0];

  if (!Number.isInteger(location)) {
    alert("Неможливо створити нову посаду через відсутню вибрану локацію!")
    return;
  }
  await save(
    work_id, poseditNameDiv.value, parseFloat(poseditSalaryDiv.value), location,
    cme.checked, cmp.checked, cmc.checked, csp.checked 
  )
  await loadPos()
})
document.getElementById("btn-posedit-back").addEventListener("click", () => loadPos())

document.getElementById("btn-posedit-new").addEventListener("click", () => showEditor())