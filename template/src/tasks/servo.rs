//! Servo Task

use {
    crate::resources::gpio_list::ServoResources,
    crate::builder::servo_builder::ServoBuilder,
    embassy_rp::pwm::{
        Config, 
        Pwm
    },
    embassy_time::Timer,
};

const SERVO_FREQ: u32 = 50;

#[embassy_executor::task]
pub async fn servo(r: ServoResources) {
    let mid_pwm = Pwm::new_output_a(r.SERVO_MID_SLICE, r.SERVO_MID_PIN, Config::default());
    let base_pwm = Pwm::new_output_a(r.SERVO_BASE_SLICE, r.SERVO_BASE_PIN, Config::default());

    let mut mid_servo = ServoBuilder::new(mid_pwm)
        .set_servo_freq(SERVO_FREQ)
        .set_max_degree_rotation(180)
        .set_min_pulse_width(1000)
        .set_max_pulse_width(2000)
        .build();
    
    let mut base_servo = ServoBuilder::new(base_pwm)
        .set_servo_freq(SERVO_FREQ)
        .set_max_degree_rotation(180)
        .set_min_pulse_width(1000)
        .set_max_pulse_width(2000)
        .build();
    
    mid_servo.disable();
    base_servo.disable();
    Timer::after_secs(1).await;

    mid_servo.enable();
    base_servo.enable();
    Timer::after_secs(1).await;

    loop {
        mid_servo.rotate(180);
        base_servo.rotate(90);
        Timer::after_secs(1).await;
    }
}