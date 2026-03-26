#![no_std]
#![no_main]

use esp32c6_app::{init_led, status::Status};
use esp_backtrace as _;
use esp_hal::{
    main,
    tsens::{Config as TsensConfig, TemperatureSensor},
    Config,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let p = esp_hal::init(Config::default());
    let mut led = init_led!(p);

    let tsens = TemperatureSensor::new(p.TSENS, TsensConfig::default()).unwrap();
    let mut status = Status::default();
    let mut counter = 0;

    loop {
        led.blink_green(500);

        counter += 1;
        if counter >= 4 {
            status.temp = tsens.get_temperature().to_celsius();
            println!("temp={:.2}", status.temp);
            counter = 0;
        }
    }
}
