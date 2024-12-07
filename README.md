# 🛠️ Backend - Rust Actix Web

This is the backend service for the **CRUD Operations** project, providing a RESTful API for managing items. Built using the **Rust** programming language and the **Actix Web** framework, it offers high performance and scalability.

---

## 📋 API Endpoints

### Base URL: `http://127.0.0.1:8080`

| **Method** | **Endpoint**       | **Description**           |
|------------|--------------------|---------------------------|
| `GET`      | `/items`           | Fetch all items           |
| `GET`      | `/items/{id}`      | Fetch a specific item     |
| `POST`     | `/items`           | Create a new item         |
| `PUT`      | `/items/{id}`      | Update an existing item   |
| `DELETE`   | `/items/{id}`      | Delete an item by ID      |

---

## 🚀 Features

- Lightweight RESTful API with **Actix Web**.
- Built with **Rust** for optimal performance.
- In-memory storage for fast prototyping and testing.
- Modular and extensible design.

---

## 🛠️ Installation & Setup

### Prerequisites
- Install **Rust** and **Cargo**:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
