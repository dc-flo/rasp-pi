use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use rppal::gpio::{Gpio, Level};
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Device {}", DeviceInfo::new()?.model());
    let gpio = Gpio::new()?;
    let mut led = gpio.get(24)?
        .into_output();
    let mut btn = gpio.get(23)?
        .into_input();
    let mut up = false;
    loop {
        if btn.read() == Level::High {
            up = !up;
        } else {
            up = false;
        }
        if up {
            led.write(Level::High);
        } else {
            led.write(Level::Low);
        }
    }
}
