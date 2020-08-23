#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

mod database;

pub mod schema;
pub mod models;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    database::init_db();
    // TODO: enrich database (separate process)
    // TODO: more endpoints
    rocket::ignite().mount("/", routes![index]).launch();
}