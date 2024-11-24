use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use core::time::Duration;
use {defmt_rtt as _, panic_probe as _};

const DEFAULT_SERVO_FREQ: u32 = 50; //Hertz
const SERVO_PERIOD: u16 = 20_000; //ns
const DEFAULT_MIN_PULSE_WIDTH: u64 = 1000; //us 
const DEFAULT_MAX_PULSE_WIDTH: u64 = 2000;  //us
const DEFAULT_MAX_DEGREE_ROTATION: u64 = 180; //degree

pub struct ServoBuilder<'d> {
    pwm: Pwm<'d>,
    cfg: Config,
    freq: u32,
    min_pulse_width: Duration,
    max_pulse_width: Duration,
    max_degree_rotation: u64,
}

impl<'d> ServoBuilder<'d> {
    pub fn new(pwm: Pwm<'d>) -> Self {
        Self {
            pwm,
            cfg: Config::default(),
            freq: DEFAULT_SERVO_FREQ,
            min_pulse_width: Duration::from_micros(DEFAULT_MIN_PULSE_WIDTH),
            max_pulse_width: Duration::from_micros(DEFAULT_MAX_PULSE_WIDTH),
            max_degree_rotation: DEFAULT_MAX_DEGREE_ROTATION,
        }
    }

    pub fn set_servo_freq(mut self, freq: u32) -> Self {
        self.freq = freq;
        self
    }

    pub fn set_min_pulse_width(mut self, duration: Duration) -> Self {
        self.min_pulse_width = duration;
        self
    }

    pub fn set_max_pulse_width(mut self, duration: Duration) -> Self {
        self.max_pulse_width = duration;
        self
    }

    pub fn set_max_degree_rotation(mut self, degree: u64) -> Self {
        self.max_degree_rotation = degree;
        self
    }

    pub fn build(mut self) -> Servo<'d> {
        
        let clock_freq = embassy_rp::clocks::clk_sys_freq();
        self.cfg.top = (clock_freq / self.freq) as u16 - 1;
        self.pwm.set_config(&self.cfg);

        Servo {
            pwm: self.pwm,
            cfg: self.cfg,
            min_pulse_width: self.min_pulse_width,
            max_pulse_width: self.max_pulse_width,
            max_degree_rotation: self.max_degree_rotation,
        }
    }
}

pub struct Servo<'d> {
    pwm: Pwm<'d>,
    cfg: Config,
    min_pulse_width: Duration,
    max_pulse_width: Duration,
    max_degree_rotation: u64,
}

impl<'d> Servo<'d> {
    pub fn enable(&mut self) {
        self.cfg.enable = true;
        self.pwm.set_config(&self.cfg);
    }

    pub fn disable(&mut self) {
        self.cfg.enable = false;
        self.pwm.set_config(&self.cfg);
    }

    pub fn set_servo_duty(&mut self, percent: u8) {
        self.pwm.set_duty_cycle_percent(percent).unwrap();
    }

    pub fn rotate(&mut self, degree: u64) {
        let micro_second_per_degree = (self.max_pulse_width.as_micros() as u64 - self.min_pulse_width.as_micros() as u64)
            / self.max_degree_rotation;
        let mut duration =
            Duration::from_micros(degree * micro_second_per_degree + self.min_pulse_width.as_micros() as u64);
        if self.max_pulse_width < duration {
            duration = self.max_pulse_width;
        }

        self.pwm.set_duty_cycle_fraction(duration.as_micros() as u16, SERVO_PERIOD).unwrap();
    }
}
