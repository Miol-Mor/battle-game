#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

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

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

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
