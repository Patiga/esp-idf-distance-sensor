use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::task::block_on;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let trigger_pin = peripherals.pins.gpio13;
    let echo_pin = peripherals.pins.gpio12;

    let mut trigger_driver = PinDriver::output(trigger_pin).unwrap();
    trigger_driver.set_low().unwrap();
    let mut echo_driver = PinDriver::input(echo_pin).unwrap();

    let mut start;
    for i in 0.. {
        trigger_driver.set_high().unwrap();
        Ets::delay_us(10);
        trigger_driver.set_low().unwrap();
        start = SystemTime::now();
        block_on(echo_driver.wait_for_high()).unwrap();
        println!(
            "Iteration {} - Time in micros: {}",
            i,
            start.elapsed().unwrap().as_micros()
        );
        thread::sleep(Duration::from_secs(1));
    }
}
