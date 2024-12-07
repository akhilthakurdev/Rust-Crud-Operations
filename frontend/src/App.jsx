import React, { useState, useEffect } from "react";
import "./App.css";

const API_URL = "http://127.0.0.1:8080/items";

function App() {
  const [items, setItems] = useState([]);
  const [id, setId] = useState("");
  const [name, setName] = useState("");
  const [editing, setEditing] = useState(false);
  const [responseMessage, setResponseMessage] = useState("");

  useEffect(() => {
    fetchItems();
  }, []);

  const fetchItems = async () => {
    try {
      const response = await fetch(API_URL);
      const data = await response.json();
      setItems(data);
    } catch (error) {
      console.error("Error fetching items:", error);
      setResponseMessage("Failed to fetch items. Please try again.");
    }
  };

  const handleAddOrUpdate = async () => {
    try {
      const method = editing ? "PUT" : "POST";
      const url = editing ? `${API_URL}/${id}` : API_URL;

      const response = await fetch(url, {
        method,
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: parseInt(id), name }),
      });

      console.log("Full Response:", response);

      // Read the response body as text or JSON
      const responseBody = await response.text();
      console.log("Response Body:", responseBody);

      if (response.ok) {
    
        fetchItems();
        setId("");
        setName("");
        setEditing(false);
        setResponseMessage(editing ? "Item updated successfully!" : "Item added successfully!");
      } else {
        setResponseMessage("Error saving the item. Please try again.");
        console.error("Error saving item");
      }
    } catch (error) {
      console.error("Error adding/updating item:", error);
      setResponseMessage("Error adding/updating item. Please try again.");
    }
  };

  const handleEdit = (item) => {
    setId(item.id);
    setName(item.name);
    setEditing(true);
    setResponseMessage("");
  };

  const handleDelete = async (itemId) => {
    try {
      const response = await fetch(`${API_URL}/${itemId}`, { method: "DELETE" });
      if (response.ok) {
        fetchItems();
        setResponseMessage("Item deleted successfully!");
      } else {
        setResponseMessage("Error deleting item. Please try again.");
        console.error("Error deleting item");
      }
    } catch (error) {
      console.error("Error deleting item:", error);
      setResponseMessage("Error deleting item. Please try again.");
    }
  };

  return (
    <div className="container">
      <h1 className="heading">Simple Item Manager</h1>

      {responseMessage && <div className="message">{responseMessage}</div>}

      <div className="form">
        <input
          type="text"
          placeholder="ID"
          value={id}
          onChange={(e) => setId(e.target.value)}
          className="input"
          required
        />
        <input
          type="text"
          placeholder="Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="input"
          required
        />
        <button onClick={handleAddOrUpdate} className="button">
          {editing ? "Update" : "Add"} Item
        </button>
      </div>

      <ul className="list">
        {items.map((item) => (
          <li key={item.id} className="list-item">
            <span>
              <strong>ID:</strong> {item.id}, <strong>Name:</strong> {item.name}
            </span>
            <div>
              <button onClick={() => handleEdit(item)} className="edit-button">
                Edit
              </button>
              <button onClick={() => handleDelete(item.id)} className="delete-button">
                Delete
              </button>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
