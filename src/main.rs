use std::error::Error;
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
    let mut before = false;
    loop {
        before = up;
        up = btn.is_high();
        if before != up {
            println!("Button is {}", if up { "up" } else { "down" });
            if up {
                led.write(Level::High);
            } else {
                led.write(Level::Low);
            }
        }
    }
}
