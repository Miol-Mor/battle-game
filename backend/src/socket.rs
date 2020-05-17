use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::thread::spawn;
use tungstenite::server::accept;

pub struct Socket {
    server: TcpListener,
}

impl Socket {
    pub fn new(port: u16) -> Result<Socket, std::io::Error> {
        let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);

        match TcpListener::bind(address) {
            Ok(server) => Ok(Socket { server }),
            Err(error) => Err(error),
        }
    }

    // TODO:
    // * Catch error globaly, not in every socket usage
    // * Match error types and exit thread for ConnectionAbort and similar errors
    //      and do something else on other error types (like timeout)
    pub fn listen(self) {
        for stream in self.server.incoming() {
            spawn(move || {
                let stream = match stream {
                    Ok(stream) => stream,
                    Err(error) => panic!("Stream error: {}", error),
                };

                let mut socket = match accept(stream) {
                    Ok(socket) => socket,
                    Err(error) => panic!("Stream accept error: {}", error),
                };

                loop {
                    let msg = match socket.read_message() {
                        Ok(msg) => msg,
                        Err(error) => {
                            println!("Error reading message: {}", error);
                            break;
                        }
                    };

                    // We do not want to send back ping/pong messages.
                    if msg.is_binary() || msg.is_text() {
                        println!("{}", msg);
                        match socket.write_message(msg) {
                            Ok(_) => (),
                            Err(error) => {
                                print!("Error writing message: {}", error);
                                break;
                            }
                        }
                    }
                }
            });
        }
    }
}

#[cfg(test)]
mod test {
    use super::Socket;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    #[test]
    fn new() {
        let port = 9999;

        let socket = Socket::new(port).unwrap();
        assert_eq!(
            socket.server.local_addr().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port)),
        )
    }
}
