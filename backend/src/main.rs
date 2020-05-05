#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/connect")]
fn connect() -> &'static str {
   "I'm sorry Dave, I'm afraid I can't do that"
}

#[get("/")]
fn main_page() -> &'static str {
    "\t
    Welcome to our first Rocket server.\n
    You can go to http://localhost:8000/connect to see reference to Stanley Kubrick
    "
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![connect, main_page])
}

fn main() {
   rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn connect() {
        let path = "/connect";
        let response_msg = "I'm sorry Dave, I'm afraid I can't do that";

        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get(path).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(response_msg.into()));
    }
}
