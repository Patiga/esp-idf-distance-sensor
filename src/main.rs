use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::{InterruptType, PinDriver, Pull};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::sys::esp_timer_get_time;
use std::thread;
use std::time::Duration;

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
    echo_driver.set_pull(Pull::Up).unwrap();
    echo_driver
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();

    static mut START: i64 = 0;
    static mut END: i64 = 0;

    unsafe {
        echo_driver
            .subscribe(|| {
                if END < START {
                    END = esp_timer_get_time();
                }
            })
            .unwrap();
    }

    for i in 0.. {
        echo_driver.enable_interrupt().unwrap();
        trigger_driver.set_high().unwrap();
        Ets::delay_us(10);
        trigger_driver.set_low().unwrap();
        unsafe { START = esp_timer_get_time() }
        thread::sleep(Duration::from_millis(500));
        unsafe {
            if END < START {
                println!("Iteration {i}: No echo!");
            } else {
                println!("Iteration {i}: time in micros: {}", END - START);
            }
            println!("Times: Start: {START}, End: {END}");
        }
    }
}
