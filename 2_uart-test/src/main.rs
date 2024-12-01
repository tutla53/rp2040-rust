
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::uart::{PioUartRx, PioUartRxProgram, PioUartTx, PioUartTxProgram};
use embassy_rp::{bind_interrupts, pio};
use {defmt_rtt as _, panic_probe as _};

//use crate::uart::PioUart;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});


#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    let p = embassy_rp::init(Default::default());

    // PIO UART setup
    let pio::Pio {
        mut common, sm0, sm1, ..
    } = pio::Pio::new(p.PIO0, Irqs);

    let tx_program = PioUartTxProgram::new(&mut common);
    let mut uart_tx = PioUartTx::new(9600, &mut common, sm0, p.PIN_4, &tx_program);

    let rx_program = PioUartRxProgram::new(&mut common);
    let mut uart_rx = PioUartRx::new(9600, &mut common, sm1, p.PIN_5, &rx_program);

    let mut n:u8;
    loop{
        n = uart_rx.read_u8().await;
        uart_tx.write_u8(n).await;
    }
}
