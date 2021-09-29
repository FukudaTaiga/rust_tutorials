let load = () => {
  let addArea = document.getElementById("add-area");
  let addButton = document.getElementById("add-button");

  addButton.addEventListener("click", () => {
    handleAdd(addArea);
  });
};

var counter = 0;

let handleAdd = (target) => {
  let el = document.createElement("p");
  el.appendChild(document.createTextNode(("this is a " + counter + "'th added text")));
  el.classList.add("added");
  target.appendChild(el);
  counter++;
}

window.onload = load;
