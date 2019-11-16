#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use serde_json::json;

use rocket_contrib::json::JsonValue;

#[get("/")]
fn index() -> JsonValue {
    JsonValue::from(json!({"message": "Hello, world!"}))
}

#[get("/gb")]
fn gb() -> JsonValue {
    JsonValue::from(json!({"message": "Goodbye!"}))
}

#[get("/ga")]
fn ga() -> JsonValue {
    JsonValue::from(json!({"message": "Good Afternoon!"}))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, ga, gb])
        .launch();
}
