//! Resource Allocation Module
//!
//! This module defines the hardware resources used by various components of the robot.
//! It uses the `assign_resources` macro to allocate specific pins and peripherals to each component.

use assign_resources::assign_resources;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals;
use embassy_rp::adc::InterruptHandler as AdcInterruptHandler;
use embassy_rp::pio::InterruptHandler as PioInterruptHandler;

assign_resources! {
    led_resources: LedFadeResources {
        LED_PIN: PIN_25,
        LED_SLICE: PWM_SLICE4,
        PIO_CH: PIO0, //Please also change the Irqs to change this
    },

    servo_resources: ServoResources {
        SERVO_MID_PIN: PIN_18,
        SERVO_MID_SLICE: PWM_SLICE1,
        SERVO_END_PIN: PIN_20,
        SERVO_END_SLICE: PWM_SLICE2,
        SERVO_BASE_PIN: PIN_22,
        SERVO_BASE_SLICE: PWM_SLICE3,
    },
    
    adc_resources: ADCResources{
        ADC_X_PIN: PIN_26,
        ADC_Y_PIN: PIN_27,
        ADC_SPEED_PIN: PIN_28,
    },
}

bind_interrupts!(pub struct Irqs {
    ADC_IRQ_FIFO => AdcInterruptHandler;
    PIO0_IRQ_0 => PioInterruptHandler<peripherals::PIO0>;
});

