document.addEventListener("DOMContentLoaded", () => {
  const itemForm = document.getElementById("item-form");
  const itemName = document.getElementById("item-name");
  const itemList = document.getElementById("item-list");

  const fetchItems = async () => {
    const response = await fetch("/items");
    const items = await response.json();
    itemList.innerHTML = "";
    items.forEach((item) => {
      const li = document.createElement("li");
      li.textContent = item.name;
      const deleteButton = document.createElement("button");
      deleteButton.textContent = "Delete";
      deleteButton.addEventListener("click", () => deleteItem(item.id));
      li.appendChild(deleteButton);
      itemList.appendChild(li);
    });
  };

  const createItem = async (name) => {
    const response = await fetch("/items", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ name }),
    });
    const newItem = await response.json();
    const li = document.createElement("li");
    li.textContent = newItem.name;
    const deleteButton = document.createElement("button");
    deleteButton.textContent = "Delete";
    deleteButton.addEventListener("click", () => deleteItem(newItem.id));
    li.appendChild(deleteButton);
    itemList.appendChild(li);
  };

  const deleteItem = async (id) => {
    await fetch(`/items/${id}`, {
      method: "DELETE",
    });
    fetchItems();
  };

  itemForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    await createItem(itemName.value);
    itemName.value = "";
  });

  fetchItems();
});
