#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/gb")]
fn gb() -> &'static str {
    "Goodbye!"
}#[get("/ga")]
fn ga() -> &'static str {
    "Good Afternoon!"
}

fn main(){
    rocket::ignite()
        .mount("/", routes![index, ga, gb])
        .launch();
}
