import { showSection } from './main.js'
import * as api from './api.js'
import * as auth from './auth.js';

const categoriesTableDiv = document.getElementById("categories-table");
const catedit_title = document.getElementById("catedit-title");
const inp_catedit_name = document.getElementById("inp-catedit-name");

let work_id = null;
async function show(id = null) {
  inp_catedit_name.value = ""
  if (id !== null) {
    work_id = id;
    const category = await api.category(id);
    inp_catedit_name.placeholder = category.name
    catedit_title.innerText = `Редагування категорії: ${category.name}`
  } else {
    work_id = null;
    catedit_title.innerText = `Створення нової категорії`
    inp_catedit_name.placeholder = ""
  }
  showSection("wdv-category-form")
}


async function save(id=null, name=null) {
  if (name === "") { name = null };
  try {
    if (id === null) {
      await api.create_category(auth.accountInfo.token, name);
    } else {
      await api.edit_category(auth.accountInfo.token, id, name);
    }
    await load()
  }  
  catch(e) {
    if (e === "Exists") {
      alert("Така категорія вже існує!")
    } else if (e === "Not Found") {
      alert("Такої категорії вже не існує! Можливо вона була видалена кимось іншим під час редагування.")
    } else {
      alert("Щось пішло не так. Можливо така категорія вже існує або ви не заповнили всі поля у випадку додавання.")
    }
  }
}


async function del(id = null) {
  try {
    await api.delete_category(auth.accountInfo.token, id)
  }  
  catch(e) {
    if (e === "NotFound") {
      alert("Така категорія не існує")
    } else {
      alert("Щось пішло не так.")
    }
  }
}

export async function load(name=null) {
  let categories = await api.categories(name);
  categoriesTableDiv.innerHTML = "";
  categories.forEach(category => {
    let div = document.createElement("div");
    
    let nameSpan = document.createElement("span");
    nameSpan.innerText = category.name;

    let editBtn = document.createElement("button");
    editBtn.innerText = "Редагувати"
    editBtn.addEventListener("click", () => show(category.id))

    let delBtn = document.createElement("button");
    delBtn.innerText = "Видалити"
    delBtn.addEventListener("click", async () => {
      await del(category.id)
      await load(name)
    })

    div.append(nameSpan);
    div.append(editBtn);
    div.append(delBtn);

    categoriesTableDiv.append(div)
  })
  showSection("menu-categories")
}

document.getElementById("btn-catedit-save").addEventListener("click", () => save(work_id, inp_catedit_name.value))

document.getElementById("btn-backto-catedit").addEventListener("click", () => { 
  load();
})
document.getElementById("btn-catedit-add").addEventListener("click", () => show());