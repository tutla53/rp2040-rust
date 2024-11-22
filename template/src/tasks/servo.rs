//! This example shows how to create a pwm using the PIO module in the RP2040 chip.

use crate::resources::gpio_list::Irqs;
use crate::builder::ServoBuilder::ServoBuilder;

use core::time::Duration;

use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{Instance, InterruptHandler, Pio};
use embassy_rp::pio_programs::pwm::{PioPwm, PioPwmProgram};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
pub async fn servo() {
    let p = embassy_rp::init(Default::default());
    let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);

    let prg = PioPwmProgram::new(&mut common);
    let pwm_pio = PioPwm::new(&mut common, sm0, p.PIN_1, &prg);
    let mut servo = ServoBuilder::new(pwm_pio)
        .set_max_degree_rotation(120) // Example of adjusting values for MG996R servo
        .set_min_pulse_width(Duration::from_micros(350)) // This value was detemined by a rough experiment.
        .set_max_pulse_width(Duration::from_micros(2600)) // Along with this value.
        .build();

    servo.start();

    let mut degree = 0;
    loop {
        degree = (degree + 1) % 120;
        servo.rotate(degree);
        Timer::after_millis(50).await;
    }
}
