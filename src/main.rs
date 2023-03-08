#![no_std]
#![no_main]

use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};
use panic_halt as _;

const MAX_ON: u32 = 20000;
const MIN_OFF: u32 = 1000;
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

// fn breathe_in(led: &mut PB5<Output>) {
fn breathe_in<T: PinOps>(led: &mut Pin<Output, T>) {
    let mut on_time = 1000;
    let mut off_time = 20000;
    loop {
        led.toggle();
        arduino_hal::delay_us(on_time);
        led.toggle();
        arduino_hal::delay_us(off_time);
        if on_time == MAX_ON {
            break;
        }
        on_time += 1000;
        off_time -= 1000;
    }
}

fn breathe_out<T: PinOps>(led: &mut Pin<Output, T>) {
    let mut on_time = 20000;
    let mut off_time = 1000;
    loop {
        led.toggle();
        arduino_hal::delay_us(on_time);
        led.toggle();
        arduino_hal::delay_us(off_time);
        if off_time == MIN_OFF {
            break;
        }
        on_time -= 1000;
        off_time += 1000;
    }
}
