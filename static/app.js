const baseUrl = "http://127.0.0.1:8080";

async function createItem() {
  const itemName = document.getElementById("itemName").value;
  const response = await fetch(`${baseUrl}/items`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ name: itemName }),
  });

  if (response.ok) {
    alert("Item created successfully!");
    document.getElementById("itemName").value = "";
    getItems();
  } else {
    alert("Error creating item");
  }
}

async function getItems() {
  const response = await fetch(`${baseUrl}/items`);
  const items = await response.json();
  const itemsList = document.getElementById("itemsList");
  itemsList.innerHTML = "";
  items.forEach((item) => {
    const li = document.createElement("li");
    li.textContent = item.name;
    itemsList.appendChild(li);
  });
}

document.addEventListener("DOMContentLoaded", getItems);
