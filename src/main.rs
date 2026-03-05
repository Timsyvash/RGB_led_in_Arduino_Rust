#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut red = pins.d5.into_output();
    let mut green = pins.d6.into_output();
    let mut blue = pins.d7.into_output();

    loop {
        red.set_high();
        green.set_low();
        blue.set_low();
        arduino_hal::delay_ms(1000);

        red.set_low();
        green.set_high();
        blue.set_low();
        arduino_hal::delay_ms(1000);

        red.set_low();
        green.set_low();
        blue.set_high();
        arduino_hal::delay_ms(1000);

        red.set_high();
        green.set_high();
        blue.set_low();
        arduino_hal::delay_ms(1000);

        red.set_high();
        green.set_high();
        blue.set_high();
        arduino_hal::delay_ms(1000);

        red.set_low();
        green.set_high();
        blue.set_high();
        arduino_hal::delay_ms(1000);

        red.set_high();
        green.set_low();
        blue.set_high();
        arduino_hal::delay_ms(1000);
    }
}
