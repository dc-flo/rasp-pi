use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use rppal::gpio::{Gpio, Level};
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking LED on {}", DeviceInfo::new()?.model());
    let mut led = Gpio::new()?
        .get(18)?
        .into_output();
    loop {
        led.write(Level::High);
        sleep(Duration::from_secs(1));
        led.write(Level::Low);
        sleep(Duration::from_secs(1));
    }
}
