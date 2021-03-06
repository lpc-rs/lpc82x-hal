#![no_main]
#![no_std]

extern crate panic_rtt_target;

use lpc8xx_hal::{
    cortex_m_rt::entry, delay::Delay, gpio::Level, prelude::*, CorePeripherals,
    Peripherals,
};

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    // Get access to the device's peripherals. Since only one instance of this
    // struct can exist, the call to `take` returns an `Option<Peripherals>`.
    // If we tried to call the method a second time, it would return `None`, but
    // we're only calling it the one time here, so we can safely `unwrap` the
    // `Option` without causing a panic.
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    // Initialize the APIs of the peripherals we need.
    let mut delay = Delay::new(cp.SYST);
    #[cfg(feature = "82x")]
    let gpio = p.GPIO; // GPIO is initialized by default on LPC82x.
    #[cfg(feature = "845")]
    let gpio = {
        let mut syscon = p.SYSCON.split();
        p.GPIO.enable(&mut syscon.handle)
    };

    // Select pin for LED
    #[cfg(feature = "82x")]
    let (led, token) = (p.pins.pio0_12, gpio.tokens.pio0_12);
    #[cfg(feature = "845")]
    let (led, token) = (p.pins.pio1_1, gpio.tokens.pio1_1);

    // Configure the LED pin. The API tracks the state of pins at compile time,
    // to prevent any mistakes.
    let mut led = led.into_output_pin(token, Level::Low);

    // Blink the LED using the systick with the delay traits
    loop {
        delay.delay_ms(1_000_u16);
        led.set_high();
        delay.delay_ms(1_000_u16);
        led.set_low();
    }
}
