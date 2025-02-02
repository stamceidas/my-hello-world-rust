use rocket::routes;
use rocket::{delete, get, post, put};
use rocket::response::status::NoContent;
use serde_json::{json, Value};

#[get("/rustaceans")]
async fn rustaceans() -> Value {
    json!([
        { "id": 1, "name": "Alice" },
        { "id": 2, "name": "Bob" },
        { "id": 3, "name": "Carol" },
        { "id": 4, "name": "Dave" },
        { "id": 5, "name": "Eve" },
    ])
}

#[get("/rustaceans/<id>")]
async fn rustacean_by_id(id: u32) -> Value {
    json!({
        "id": id,
        "name": "Alice",
        "email": "email"
    })
}

#[post("/rustaceans", format = "json")]
async fn create_rustacean() -> Value {
    json!({
        "id": 6,
        "name": "Ferris",
        "email": "email"
    })
}

#[put("/rustaceans/<id>", format = "json")]
async fn update_rustacean(id: u32) -> Value {
    json!({
        "id": id,
        "name": "Ferris",
        "email": "email"
    })
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: u32) -> NoContent {
    NoContent
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/api", routes![
            rustaceans,
            rustacean_by_id,
            create_rustacean,
            update_rustacean,
            delete_rustacean
        ])
        .launch()
        .await?;

    Ok(())
}
