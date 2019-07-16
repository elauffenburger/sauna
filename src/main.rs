extern crate websocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod logger;

use std::thread;
use std::io::{self, Write};
use std::sync::mpsc::channel;
use std::cell::RefCell;
use std::rc::Rc;
use websocket::sync::Server;
use websocket::OwnedMessage;

use logger::*;

#[derive(Debug, Serialize, Deserialize)]
struct SaunaMessage {
    pub t: String
}

fn make_logger<'a>(name: &'a str) -> Box<Logger> {
    Box::new(ConsoleLogger::new(name.to_string()))
}

fn main() {
    let mut server_logger = make_logger("sauna");

    let mut server_addr = String::from("");
    for arg in std::env::args() {
        server_addr = arg;
    }

    if server_addr.as_str() == "" {
        server_addr = "127.0.0.1:2794".to_string();
    }

    let server = Server::bind(&server_addr).unwrap();

    server_logger.info(format!("Welcome to sauna!\n"));
    server_logger.info(format!("Server listening on {}", &server_addr));

    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            let mut req_logger = make_logger("thread-logger");

            let mut conn = request.accept().unwrap();
            let ip = conn.peer_addr().unwrap();

            req_logger.debug(format!("Received connection from {}", ip));

            let (mut rx, mut tx) = conn.split().unwrap();

            let ready_message = OwnedMessage::Text(serde_json::to_string(&SaunaMessage{ t: "ready".trim().to_string() }).unwrap());
            tx.send_message(&ready_message).unwrap();

            loop {
                print!("{}:>", ip);
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                tx.send_message(&format_message(SaunaMessage{ t: input.trim_right().to_string() }));
            }
        });
    }
}

fn format_message(msg: SaunaMessage) -> OwnedMessage {
    OwnedMessage::Text(serde_json::to_string(&msg).unwrap())
}
