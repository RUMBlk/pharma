import { showSection } from './main.js'
import * as api from './api.js'
import * as auth from './auth.js';
import * as positions from './positions.js';

let position = null;
let work_id = null;

const tableDiv = document.getElementById("empledit-table");
const empledit_title = document.getElementById("empledit-title");
const empleditSurnameDiv = document.getElementById("inp-empledit-surname");
const empleditNameDiv = document.getElementById("inp-empledit-name");
const empleditPatronimDiv = document.getElementById("inp-empledit-patronim");
const empleditSalbonusDiv = document.getElementById("inp-empledit-salbonus");
  
  function createPosDiv(surname, name, patronim, hired_at, salary_bonus) {
    let div = document.createElement("div");
  
    let nameDiv = document.createElement("h1");
    nameDiv.textContent = `${surname} ${name} ${patronim}`;
    div.appendChild(nameDiv);
  
    let hiredDiv = document.createElement("span");
    hiredDiv.classList.add("larger-text") 
    hiredDiv.innerText = `Дата найняття: ${hired_at}`
    div.append(hiredDiv)

    let salaryDiv = document.createElement("div");
    let salaryText = document.createElement("strong");
    salaryText.classList.add("final-price")
    salaryText.textContent = `Премія: ${salary_bonus} грн.`;
    salaryDiv.appendChild(salaryText);
    
    div.appendChild(salaryDiv);
  
    return div;
  }
  
  export async function loadEmpl() {
    showSection("menu-empl")
    tableDiv.innerHTML = "";

    let employees = await api.employees(auth.accountInfo.token, null, position)
    
    employees.forEach(element => {
      let div = createPosDiv(element.surname, element.name, element.patronim, element.hired_at, element.salary_bonus);

      let editBtn = document.createElement("button");
      editBtn.innerText = "Редагувати"
      editBtn.addEventListener("click", () => showEditor(element.id))

      let regBtn = document.createElement("button");
      regBtn.innerText = "Рег. акаунта"
      regBtn.addEventListener("click", () => auth.showRegisterForm(element.id))

      let delBtn = document.createElement("button");
      delBtn.innerText = "Звільнити"
      delBtn.addEventListener("click", async () => {
        await del(element.id)
        await loadEmpl()
      })
  
      div.appendChild(editBtn);
      div.appendChild(regBtn);
      div.appendChild(delBtn);
      tableDiv.appendChild(div);
    });
  }
  
  export async function load(_position = null) {
    position = _position
    await loadEmpl()
}


  //Editor

  async function showEditor(id = null) {
    empleditSurnameDiv.value = ""
    empleditNameDiv.value = ""
    empleditPatronimDiv.value = ""
    empleditSalbonusDiv.value = ""

    if (id !== null) {
      work_id = id;
      const empl = await api.employee(auth.accountInfo.token, id);
      empleditSurnameDiv.placeholder = empl.surname
      empleditNameDiv.placeholder = empl.name
      empleditPatronimDiv.placeholder = empl.patronim
      empleditSalbonusDiv.placeholder = empl.salary_bonus
      empledit_title.innerText = `Редагування працівника: ${position.name}`
    } else {
      work_id = null;
      empleditSurnameDiv.placeholder = ""
      empleditNameDiv.placeholder = ""
      empleditPatronimDiv.placeholder = ""
      empleditSalbonusDiv.placeholder = ""
      empledit_title.innerText = `Створення нової посади`
    }
    showSection("wdv-empl-form")
  }

  async function save(id=null, surname=null, name=null, patronim=null, position=null, salary_bonus=null) {
    if (surname === "") { surname = null };
    if (name === "") { name = null };
    if (patronim === "") { patronim = null };
    try {
      if (id === null) {
        await api.create_empl(auth.accountInfo.token, surname, name, patronim, position, salary_bonus);
      } else {
        await api.edit_empl(auth.accountInfo.token, id, surname, name, patronim, position, salary_bonus)
      }
    }  
    catch(e) {
      if (e === "Exists") {
        alert("Працівник з таким ID вже існує!")
      } else if (e === "Not Found") {
        alert("Працівника не знайдено")
      } else {
        alert("Щось пішло не так")
      }
    }
  }

  async function del(id = null) {
    try {
      await api.delete_empl(auth.accountInfo.token, id)
    }  
    catch(e) {
      if (e === "NotFound") {
        alert("Працівника не знайдено")
      } else {
        alert("Щось пішло не так")
      }
    }
  }

document.getElementById("btn-empledit-save").addEventListener("click", async () => {
  await save(
    work_id, empleditSurnameDiv.value, empleditNameDiv.value, empleditPatronimDiv.value, position, parseFloat(empleditSalbonusDiv.value) 
  )
  await loadEmpl()
})

document.getElementById("btn-empledit-back").addEventListener("click", () => loadEmpl())

document.getElementById("btn-empledit-new").addEventListener("click", () => showEditor())
document.getElementById("btn-empledit-pos").addEventListener("click", () => positions.loadPos())