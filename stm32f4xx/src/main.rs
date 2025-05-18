#![no_std]
#![no_main]

// Entry point macro for Cortex-M
use cortex_m_rt::entry;

// Panic handler (halts on panic)
use panic_halt as _;

// Import HAL (Hardware Abstraction Layer)
use stm32f4xx_hal::{
    gpio::{gpioa::PA0, gpioc::PC13, Input, Output, PushPull}, pac, prelude::*
};

#[entry]
fn main() -> ! {
    // Get microcontroller peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure system clocks (84 MHz for STM32F401)
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    // Delay abstraction (needed for LED blink)
    let mut delay = cp.SYST.delay(&clocks);

    // Split GPIO ports
    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    // PA0 as pull-up input for IR sensor
    let ir_sensor: PA0<Input> = gpioa.pa0.into_pull_up_input();

    // PC13 as push-pull output (onboard LED on Nucleo boards)
    let mut led: PC13<Output<PushPull>> = gpioc.pc13.into_push_pull_output();

    loop {
        // Read sensor: LOW = object detected
        if ir_sensor.is_low() {
            // Turn ON LED
            led.set_high();
        } else {
            // Turn OFF LED
            led.set_low();
        }

        delay.delay_ms(50u32); // debounce / smooth transition
    }
}
