#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/connect")]
fn connect() -> &'static str {
   "I'm sorry Dave, I'm afraid I can't do that"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![connect])
}

fn main() {
   rocket().launch();
}
