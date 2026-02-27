#![no_std]
#![no_main]

use panic_halt as _;
use stm32f1xx_hal::{prelude::*, timer::Channel};

mod board;
use board::Board;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut b = Board::init();

    let max = b.pwm.get_max_duty();
    b.pwm.enable(Channel::C1);

    loop {
        b.pwm.set_duty(Channel::C1, (max / 10) * 3);
        b.delay.delay_ms(1000u16);

        b.pwm.set_duty(Channel::C1, (max / 10) * 6);
        b.delay.delay_ms(1000u16);

        b.pwm.set_duty(Channel::C1, 0);
        b.delay.delay_ms(1000u16);
    }
}
