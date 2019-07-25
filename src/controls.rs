use rppal::gpio::{Gpio, InputPin, Pin};

const BUTTON_PIN: u8 = 15;

const JOYSTICK_UP_PIN: u8 = 13;
const JOYSTICK_RIGHT_PIN: u8 = 26;
const JOYSTICK_DOWN_PIN: u8 = 6;
const JOYSTICK_LEFT_PIN: u8 = 19;

pub enum ControlPin {
    Up,
    Right,
    Down,
    Left,
    Button,
}

#[derive(Debug)]
pub struct Controls {
    pub up: InputPin,
    pub right: InputPin,
    pub down: InputPin,
    pub left: InputPin,

    pub button: InputPin,
}

impl Controls {
    pub fn new(gpio: &Gpio) -> Self {
        Controls {
            up: gpio.get(JOYSTICK_UP_PIN).unwrap().into_input_pullup(),
            right: gpio.get(JOYSTICK_RIGHT_PIN).unwrap().into_input_pullup(),
            down: gpio.get(JOYSTICK_DOWN_PIN).unwrap().into_input_pullup(),
            left: gpio.get(JOYSTICK_LEFT_PIN).unwrap().into_input_pullup(),
            button: gpio.get(BUTTON_PIN).unwrap().into_input_pulldown(),
        }
    }

    pub fn get_pin_control(pin: &InputPin) -> ControlPin {
        match pin.pin() {
            JOYSTICK_UP_PIN => ControlPin::Up,
            JOYSTICK_RIGHT_PIN => ControlPin::Right,
            JOYSTICK_DOWN_PIN => ControlPin::Down,
            JOYSTICK_LEFT_PIN => ControlPin::Left,
            BUTTON_PIN => ControlPin::Button,
            p @ _ => panic!("Unknown pin {}!", p),
        }
    }
}
