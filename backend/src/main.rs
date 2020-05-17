#![feature(proc_macro_hygiene, decl_macro)]

use std::thread::spawn;

#[macro_use]
extern crate rocket;

mod socket;

#[get("/connect")]
fn connect() -> &'static str {
    "I'm sorry Dave, I'm afraid I can't do that"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![connect])
}

fn main() {
    let port = 9001;
    let socket = socket::Socket::new(port).unwrap();

    // Socket echo server
    let handle = spawn(move || {
        socket.listen();
    });

    // Rest part
    rocket().launch();

    handle.join().unwrap();
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
