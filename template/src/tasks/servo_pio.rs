//! Servo PIO Task with state machine 0 and 1 

use {
    core::time::Duration,
    crate::builder::servo_pio_builder::ServoPioBuilder,
    crate::resources::gpio_list::{
        Irqs,
        ServoPioResources,
    },
    embassy_rp::{
        pio::Pio,
        pio_programs::pwm::{
            PioPwm, 
            PioPwmProgram,
        },
    },
    embassy_time::Timer,
    {defmt_rtt as _, panic_probe as _},
};

const REFRESH_INTERVAL: u64 = 20000;

#[embassy_executor::task]
pub async fn servo_pio(r: ServoPioResources) {
    let Pio { mut common, sm0, sm1, .. } = Pio::new(r.SERVO_PIO_CH, Irqs);
    let prg = PioPwmProgram::new(&mut common);

    let body_pwm_pio = PioPwm::new(&mut common, sm0, r.SERVO_BODY_PIN, &prg);
    let head_pwm_pio = PioPwm::new(&mut common, sm1, r.SERVO_HEAD_PIN, &prg);

    let mut body_servo = ServoPioBuilder::new(body_pwm_pio)
        .set_period(Duration::from_micros(REFRESH_INTERVAL))
        .set_max_degree_rotation(180)
        .set_min_pulse_width(Duration::from_micros(1000))
        .set_max_pulse_width(Duration::from_micros(2000))
        .build();

    let mut head_servo = ServoPioBuilder::new(head_pwm_pio)
        .set_period(Duration::from_micros(REFRESH_INTERVAL))
        .set_max_degree_rotation(180)
        .set_min_pulse_width(Duration::from_micros(1000))
        .set_max_pulse_width(Duration::from_micros(2000))
        .build();

    body_servo.stop();
    head_servo.stop();
    Timer::after_secs(1).await;
    body_servo.start();
    head_servo.start();

    loop {
        log::info!("Servo_PIO");
        body_servo.rotate(90);
        head_servo.rotate(180);
        Timer::after_millis(1).await;
    }
}