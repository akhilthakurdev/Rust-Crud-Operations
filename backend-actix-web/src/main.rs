use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Define a struct for the data
#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
}

// Define shared state
struct AppState {
    items: Mutex<Vec<Item>>, // Store items in memory
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Mutex::new(Vec::new()),
    });

    // Start the server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin() // Allow requests from any origin
            .allow_any_method() // Allow all HTTP methods (GET, POST, PUT, DELETE)
            .allow_any_header(); // Allow all headers

        App::new()
            .wrap(cors) // Apply CORS middleware
            .app_data(app_state.clone()) // Add shared state
            .route("/items", web::get().to(get_items)) // Get all items
            .route("/items/{id}", web::get().to(get_item)) // Get a specific item
            .route("/items", web::post().to(create_item)) // Create a new item
            .route("/items/{id}", web::put().to(update_item)) // Update an item
            .route("/items/{id}", web::delete().to(delete_item)) // Delete an item
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Get all items
async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(items.clone()) // Return all items as JSON
}

// Get a specific item by ID
async fn get_item(data: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let items = data.items.lock().unwrap();
    if let Some(item) = items.iter().find(|i| i.id == id) {
        HttpResponse::Ok().json(item.clone()) // Return the item if found
    } else {
        HttpResponse::NotFound().body("Item not found") // Return 404 if not found
    }
}

// Create a new item
async fn create_item(data: web::Data<AppState>, item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    if items.iter().any(|i| i.id == item.id) {
        return HttpResponse::BadRequest().body("Item with this ID already exists"); // Return 400 if duplicate ID
    }
    items.push(item.into_inner()); // Add the item to the list
    HttpResponse::Created().body("Item created") // Return 201 Created
}

// Update an item by ID
async fn update_item(
    data: web::Data<AppState>,
    path: web::Path<u32>,
    updated_item: web::Json<Item>,
) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.lock().unwrap();
    if let Some(item) = items.iter_mut().find(|i| i.id == id) {
        *item = updated_item.into_inner(); // Update the item
        HttpResponse::Ok().body("Item updated") // Return 200 OK
    } else {
        HttpResponse::NotFound().body("Item not found") // Return 404 if not found
    }
}

// Delete an item by ID
async fn delete_item(data: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.lock().unwrap();
    if let Some(pos) = items.iter().position(|i| i.id == id) {
        items.remove(pos); // Remove the item
        HttpResponse::Ok().body("Item deleted") // Return 200 OK
    } else {
        HttpResponse::NotFound().body("Item not found") // Return 404 if not found
    }
}
