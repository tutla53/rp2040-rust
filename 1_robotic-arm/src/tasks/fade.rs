use crate::resources::gpio_list::{LedFadeResources, Irqs};
use core::time::Duration;
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::pwm::{PioPwm, PioPwmProgram};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const REFRESH_INTERVAL: u64 = 20000;

#[embassy_executor::task]
pub async fn fade(r: LedFadeResources){
    let Pio { mut common, sm0, .. } = Pio::new(r.PIO_CH, Irqs);

    // Note that PIN_25 is the led pin on the Pico
    let prg = PioPwmProgram::new(&mut common);
    let mut pwm_pio = PioPwm::new(&mut common, sm0, r.LED_PIN, &prg);
    pwm_pio.set_period(Duration::from_micros(REFRESH_INTERVAL));
    pwm_pio.start();

    let mut add: i16 = 10;
    let mut duration = add;

    loop {
        if (duration == 0) || (duration > (REFRESH_INTERVAL as i16)/2)  {add = -1*add;} 
        duration = duration + add;

        pwm_pio.write(Duration::from_micros(duration as u64));
        Timer::after_millis(1).await;
    }
}
