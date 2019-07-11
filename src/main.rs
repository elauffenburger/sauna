extern crate websocket;

pub mod logger;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use logger::*;

fn make_logger<'a>(name: &'a str) -> Box<Logger> {
    Box::new(ConsoleLogger::new(name.to_string()))
}

fn main() {
    let mut server_logger = make_logger("sauna");

    let server_addr = "127.0.0.1:2794";
    let server = Server::bind(server_addr).unwrap();

    server_logger.log(LogLevel::Info, format!("Welcome to sauna on {}!\n", DeviceInfo::new().unwrap().model()));
    server_logger.log(LogLevel::Info, format!("Server listening on {}", server_addr));

    for request in server.filter_map(Result::ok) {
        thread::spawn(|| {
            let mut req_logger = make_logger("thread-logger");
            let gpio = Gpio::new().unwrap();

            let mut conn = request.accept().unwrap();
            let ip = conn.peer_addr().unwrap();

            req_logger.log(LogLevel::Info, format!("Received connection from {}", ip));

            let message = OwnedMessage::Text("Hi!".to_string());
            conn.send_message(&message).unwrap();

            let (mut rx, mut tx) = conn.split().unwrap();

            for msg in rx.incoming_messages() {
                match msg {
                    Err(err) => {
                        req_logger.log(LogLevel::Info, format!("Error: {}", err));
                        break;
                    },
                    Ok(message) => {
                        match message {
                            OwnedMessage::Text(msg_text) => {
                                req_logger.log(LogLevel::Info, format!("Received message from {}: '{}'", ip, msg_text));

                                tx.send_message(&OwnedMessage::Text(msg_text.to_string()));
                            },
                            raw_msg @ _ => req_logger.log(LogLevel::Info, format!("Received non-text message: {:?}", raw_msg))
                        }
                    }
                }
            }
        });
    }
}
