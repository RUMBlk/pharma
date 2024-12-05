export function createCheckbox(name, value, label, checked=true, disabled=false) {
    let chkbox = document.createElement("input");
    chkbox.type = "checkbox";
    chkbox.id = `${name}_${value}`
    chkbox.value = value;
    chkbox.name = name;
    chkbox.checked = checked;
    chkbox.disabled = disabled;
  
    let labelDiv = document.createElement("label");
    labelDiv.for = `${name}_${value}`
    labelDiv.classList.add("larger-text");
    labelDiv.innerHTML = label
  
    let wrapper = document.createElement("div");
    wrapper.append(chkbox, labelDiv)
    return wrapper
  }
  
export function createRadio(name, value, label, checked = false, disabled = false) {
    let box = document.createElement("input");
    box.type = "radio";
    box.id = `${name}_${value}`
    box.value = value;
    box.name = name;
    box.checked = checked;
    box.disabled = disabled;
  
    let labelDiv = document.createElement("label");
    labelDiv.for = `${name}_${value}`
    labelDiv.classList.add("larger-text");
    labelDiv.innerHTML = label
  
    let wrapper = document.createElement("div");
    wrapper.append(box, labelDiv)
    return wrapper
  }
  