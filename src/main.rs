#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let a0 = pins.a0.into_analog_input(&mut adc);

    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        let value = a0.analog_read(&mut adc);
        ufmt::uwrite!(&mut serial, "{}\n", value).unwrap();
        if value > 512 {
            led.set_high();
        } else {
            led.set_low();
        }
        arduino_hal::delay_ms(8);
    }
}
