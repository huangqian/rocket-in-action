#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod ns1 {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hello, world!"
    }
}

fn main() {
    rocket::ignite().mount("/hello", routes![ns1::world]).launch();
}
