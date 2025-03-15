use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use rppal::gpio::{Gpio, Level};
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Device {}", DeviceInfo::new()?.model());
    let mut led = Gpio::new()?
        .get(24)?
        .into_output();
    loop {
        println!("up");
        led.write(Level::High);
        sleep(Duration::from_secs(1));
        println!("down");
        led.write(Level::Low);
        sleep(Duration::from_secs(1));
    }
}
