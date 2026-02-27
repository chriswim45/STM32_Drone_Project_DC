use stm32f1xx_hal::{
    afio::AfioExt,
    gpio::{gpioa, Alternate},
    pac,
    prelude::*,
    rcc::RccExt,              // <- WICHTIG: explizit rein
    timer::{Channel, PwmHz, Tim2NoRemap},
    timer::SysDelay,
};


pub type MotorPin1 = gpioa::PA0<Alternate>; // TIM2_CH1
pub type MotorPin2 = gpioa::PA1<Alternate>; // TIM2_CH2

pub type MotorPwm = PwmHz<
    pac::TIM2,
    Tim2NoRemap,
    (stm32f1xx_hal::timer::Ch<0>, stm32f1xx_hal::timer::Ch<1>),
    (MotorPin1, MotorPin2),
>;

pub struct Board {
    pub pwm: MotorPwm,
    pub delay: SysDelay,
}

impl Board {
    pub fn init() -> Self {
        let dp = pac::Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap();

        // FLASH + RCC (HAL)
        let mut flash = dp.FLASH.constrain();
        // clock config (Standard: HSI)
        let mut rcc = dp.RCC.freeze(stm32f1xx_hal::rcc::Config::hsi(), &mut flash.acr);

        // clocks rausziehen
        let clocks = rcc.clocks;

        // Wenn deine bisherigen Zeilen mit &mut rcc schon kompiliert haben: so lassen.
        // (Bei manchen stm32f1xx-hal APIs sind es apb2 Tokens, bei dir scheint &mut rcc ok zu sein.)
        let mut afio = dp.AFIO.constrain(&mut rcc);
        let mut gpioa = dp.GPIOA.split(&mut rcc);

        // PA0 + PA1 als Alternate (TIM2 CH1/CH2)
        let pa0: MotorPin1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
        let pa1: MotorPin2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);

        let mut pwm: MotorPwm = dp
            .TIM2
            .pwm_hz::<Tim2NoRemap, _, _>((pa0, pa1), &mut afio.mapr, 20.kHz(), &mut rcc);

        pwm.disable(Channel::C1);
        pwm.set_duty(Channel::C1, 0);

        let delay = cp.SYST.delay(&clocks);

        Self { pwm, delay }
    }
}
