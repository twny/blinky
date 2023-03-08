#![no_std]
#![no_main]

use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};
use panic_halt as _;

const MAX_ON: u32 = 10000;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    loop {
        breathe_in(&mut led);
        breathe_out(&mut led);
    }
}

// duty cycle % = on_time / (on_time + off_time) * 100

// duty cycle the duration the light is on (on_time)
// brightness increases as the delay decreases (off_time)

// fn breathe_in(led: &mut PB5<Output>) {
fn breathe_in<T: PinOps>(led: &mut Pin<Output, T>) {
    let mut on_time = 50;
    let mut off_time = 10000;
    loop {
        led.toggle();
        arduino_hal::delay_us(on_time);
        led.toggle();
        arduino_hal::delay_us(off_time);
        if on_time == MAX_ON {
            break;
        }
        on_time += 50;
        off_time -= 50;
    }
}

fn breathe_out<T: PinOps>(led: &mut Pin<Output, T>) {
    let mut on_time = 10000;
    let mut off_time = 50;
    loop {
        led.toggle();
        arduino_hal::delay_us(on_time);
        led.toggle();
        arduino_hal::delay_us(off_time);
        if off_time == MAX_ON {
            break;
        }
        on_time -= 50;
        off_time += 50;
    }
}
