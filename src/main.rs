#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde;

mod db;
mod api;

fn main() {
    api::run();
}
