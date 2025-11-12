#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod mock_hal;
mod drivers;

use mock_hal::{MockGpio, MockSpi, MockI2c, MockDelay};
use drivers::{led_controller, spi_sensor, i2c_sensor};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Create mock hardware implementations
    // In a real embedded system, these would be provided by the HAL crate
    let mut gpio = MockGpio::new();
    let mut spi = MockSpi::new();
    let mut i2c = MockI2c::new();
    let mut delay = MockDelay::new();

    // Use portable drivers that work with any HAL implementation
    led_controller::blink_led(&mut gpio, &mut delay);

    let _sensor_data = spi_sensor::read_sensor(&mut spi);

    let _temp = i2c_sensor::read_temperature(&mut i2c);

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
