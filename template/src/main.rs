//! Main

#![no_std]
#![no_main]

mod resources;
mod tasks;
mod builder;

use crate::builder::servo_builder;
use crate::tasks::fade::fade;
use crate::tasks::servo::servo;

use resources::gpio_list::{AssignedResources, LedFadeResources, ServoResources, ADCResources};
use embassy_executor::Spawner;
use embassy_rp::config::Config;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner){
    let p = embassy_rp::init(Config::default());
    let r = split_resources!(p);

    spawner.spawn(fade(r.led_resources)).unwrap();
    spawner.spawn(servo(r.servo_resources)).unwrap();
}
