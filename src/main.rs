extern crate websocket;

pub mod controls;
pub mod logger;

use rppal::gpio::{Gpio, Trigger};
use rppal::system::DeviceInfo;
use std::thread;
use std::time::Duration;
use websocket::sync::Server;
use websocket::OwnedMessage;

use controls::*;
use logger::*;

const CONTROLS_POLL_RATE_MS: u64 = 100;

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

    server_logger.info(format!(
        "Welcome to sauna on {}!\n",
        DeviceInfo::new().unwrap().model()
    ));

    server_logger.info(format!("Server listening on {}", &server_addr));

    let gpio = Gpio::new().unwrap();
    let mut controls = Controls::new(&gpio);

    controls.up.set_interrupt(Trigger::FallingEdge);
    controls.right.set_interrupt(Trigger::FallingEdge);
    controls.down.set_interrupt(Trigger::FallingEdge);
    controls.left.set_interrupt(Trigger::FallingEdge);
    controls.button.set_interrupt(Trigger::RisingEdge);

    for request in server.filter_map(Result::ok) {
        let mut req_logger = make_logger("thread-logger");

        let mut conn = request.accept().unwrap();
        let ip = conn.peer_addr().unwrap();

        req_logger.debug(format!("Received connection from {}", ip));

        let (mut rx, mut tx) = conn.split().unwrap();

        loop {
            let interrupted_pin = gpio
                .poll_interrupts(
                    &[
                        &controls.up,
                        &controls.right,
                        &controls.down,
                        &controls.left,
                        &controls.button,
                    ],
                    true,
                    None,
                )
                .unwrap();

            match interrupted_pin {
                None => {}
                Some((pin, _)) => match Controls::get_pin_control(pin) {
                    ControlPin::Up => {
                        tx.send_message(&OwnedMessage::Text("nav-up".to_string()));
                    }
                    ControlPin::Right => {}
                    ControlPin::Down => {
                        tx.send_message(&OwnedMessage::Text("nav-down".to_string()));
                    }
                    ControlPin::Left => {}
                    ControlPin::Button => {
                        tx.send_message(&OwnedMessage::Text("complete-task".to_string()));
                    }
                },
            }

            thread::sleep(Duration::from_millis(CONTROLS_POLL_RATE_MS));
        }
    }
}
