use crate::resources::gpio_list::ServoResources;
use crate::builder::servo_builder::ServoBuilder;

use core::time::Duration;
use embassy_rp::pwm::{Config, Pwm};
use embassy_time::Timer;

const SERVO_FREQ: u32 = 50;

#[embassy_executor::task]
pub async fn servo(r: ServoResources) {
    let mid_pwm = Pwm::new_output_a(r.SERVO_MID_SLICE, r.SERVO_MID_PIN, Config::default());

    let mut mid_servo = ServoBuilder::new(mid_pwm)
        .set_servo_freq(SERVO_FREQ)
        .set_max_degree_rotation(180)
        .set_min_pulse_width(Duration::from_micros(1000))
        .set_max_pulse_width(Duration::from_micros(2000))
        .build();
    
    mid_servo.disable();
    Timer::after_secs(1).await;
    mid_servo.enable();

    loop {
        mid_servo.rotate(180);
        Timer::after_secs(1).await;
    }
}