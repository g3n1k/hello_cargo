use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
    price: f64,
}

// In-memory data store
static mut ITEMS: Vec<Item> = Vec::new();

// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is running")
}

// Get all items
async fn get_items() -> impl Responder {
    unsafe { HttpResponse::Ok().json(&ITEMS) }
}

// Add a new item
async fn add_item(item: web::Json<Item>) -> impl Responder {
    unsafe {
        ITEMS.push(item.into_inner());
    }
    HttpResponse::Created().body("Item added")
}

// Update an item by ID
async fn update_item(id: web::Path<u32>, item: web::Json<Item>) -> impl Responder {
    let item_id = id.into_inner();
    unsafe {
        if let Some(existing_item) = ITEMS.iter_mut().find(|i| i.id == item_id) {
            existing_item.name = item.name.clone();
            existing_item.price = item.price;
            return HttpResponse::Ok().body("Item updated");
        }
    }
    HttpResponse::NotFound().body("Item not found")
}

// Delete an item by ID
async fn delete_item(id: web::Path<u32>) -> impl Responder {
    let item_id = id.into_inner();
    unsafe {
        let len_before = ITEMS.len();
        ITEMS.retain(|i| i.id != item_id);
        if ITEMS.len() < len_before {
            return HttpResponse::Ok().body("Item deleted");
        }
    }
    HttpResponse::NotFound().body("Item not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(add_item))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
