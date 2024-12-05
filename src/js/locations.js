import { showSection } from './main.js'
import * as api from './api.js'
import * as auth from './auth.js';

const tableDiv = document.getElementById("locations-table");
const locedit_title = document.getElementById("locedit-title");
const inp_locedit_name = document.getElementById("inp-locedit-name");
const inp_locedit_addr = document.getElementById("inp-locedit-addr");

let work_id = null;
async function show(id = null) {
  inp_locedit_name.value = ""
  inp_locedit_addr.value = ""
  if (id !== null) {
    work_id = id;
    const location = await api.location(id);
    inp_locedit_name.placeholder = location.name
    inp_locedit_addr.placeholder = location.address
    locedit_title.innerText = `Редактор локації: ${location.name}`
  } else {
    work_id = null;
    locedit_title.innerText = `Створення нової локації`
    inp_locedit_name.placeholder = ""
  }
  showSection("wdv-location-form")
}


async function save(id=null, name=null, addr=null) {
  if (name === "") { name = null };
  try {
    if (id === null) {
      await api.create_location(auth.accountInfo.token, name, addr);
    } else {
      await api.edit_location(auth.accountInfo.token, id, name, addr);
    }
    await load()
  }  
  catch(e) {
    if (e === "Exists") {
      alert("Така локація вже існує!")
    } else if (e === "Not Found") {
      alert("Такої локації вже не існує! Можливо вона була видалена кимось іншим під час редагування.")
    } else {
      alert("Щось пішло не так. Можливо така локація вже існує або ви не заповнили всі поля у випадку додавання.")
    }
  }
}


async function del(id = null) {
  try {
    await api.delete_location(auth.accountInfo.token, id)
  }  
  catch(e) {
    if (e === "NotFound") {
      alert("Така локація не існує")
    } else {
      alert("Щось пішло не так.")
    }
  }
}

export async function load(name=null) {
  let locations = await api.locations(name);
  tableDiv.innerHTML = "";
  locations.forEach(location => {
    let div = document.createElement("div");
    
    let nameSpan = document.createElement("span");
    nameSpan.innerText = `${location.name} - ${location.address}`;

    let editBtn = document.createElement("button");
    editBtn.innerText = "Редагувати"
    editBtn.addEventListener("click", () => show(location.id))

    let delBtn = document.createElement("button");
    delBtn.innerText = "Видалити"
    delBtn.addEventListener("click", async () => {
      await del(location.id)
      await load(name)
    })

    div.append(nameSpan);
    div.append(editBtn);
    div.append(delBtn);

    tableDiv.append(div)
  })
  showSection("menu-locations")
}

document.getElementById("btn-locedit-save").addEventListener("click", () => save(work_id, inp_locedit_name.value, inp_locedit_addr.value))

document.getElementById("btn-locedit-back").addEventListener("click", () => { 
  load();
})
document.getElementById("btn-locedit-add").addEventListener("click", () => show());