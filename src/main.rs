#![deny(unsafe_code)]
#![no_main]
#![no_std]

//#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
//extern crate panic_halt; // panic handler
extern crate panic_itm; // panic handler

use alt_stm32f30x_hal as hal;
use hal::gpio;
use hal::prelude::*;

struct Reel {
    sensor: gpio::PAx<gpio::PullUp, gpio::Input>,
    led: gpio::PEx<gpio::PullNone, gpio::Output<gpio::PushPull, gpio::LowSpeed>>,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let device = hal::pac::Peripherals::take().unwrap();
    let mut rcc = device.RCC.constrain();

    let gpioa = device.GPIOA.split(&mut rcc.ahb);
    let gpioe = device.GPIOE.split(&mut rcc.ahb);

    // let user_button = gpioa.pa0.input().pull_type(PullNone);

    let mut reels: [Reel; 3] = [
        Reel {
            sensor: gpioa.pa1.input().pull_type(gpio::PullUp).downgrade(),
            led: gpioe.pe9.output().push_pull().downgrade(),
        },
        Reel {
            sensor: gpioa.pa2.input().pull_type(gpio::PullUp).downgrade(),
            led: gpioe.pe10.output().push_pull().downgrade(),
        },
        Reel {
            sensor: gpioa.pa3.input().pull_type(gpio::PullUp).downgrade(),
            led: gpioe.pe8.output().push_pull().downgrade(),
        },
    ];

    loop {
        for reel in &mut reels {
            if reel.sensor.is_low().unwrap() {
                reel.led.set_high().unwrap();
            } else {
                reel.led.set_low().unwrap();
            }
        }
    }
}
