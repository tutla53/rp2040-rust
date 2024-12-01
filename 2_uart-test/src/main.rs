
#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

use defmt::{info, panic, trace};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::uart::{PioUartRx, PioUartRxProgram, PioUartTx, PioUartTxProgram};
use embassy_rp::{bind_interrupts, pio};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pipe::Pipe;
use embassy_usb::driver::EndpointError;
use embedded_io_async::{Read, Write};
use {defmt_rtt as _, panic_probe as _};

//use crate::uart::PioUart;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello there!");

    let p = embassy_rp::init(Default::default());

    // PIO UART setup
    let pio::Pio {
        mut common, sm0, sm1, ..
    } = pio::Pio::new(p.PIO0, Irqs);

    let tx_program = PioUartTxProgram::new(&mut common);
    let mut uart_tx = PioUartTx::new(9600, &mut common, sm0, p.PIN_4, &tx_program);

    let rx_program = PioUartRxProgram::new(&mut common);
    let mut uart_rx = PioUartRx::new(9600, &mut common, sm1, p.PIN_5, &rx_program);

    // Pipe setup
    let mut uart_pipe: Pipe<NoopRawMutex, 20> = Pipe::new();
    let (mut uart_pipe_reader, mut uart_pipe_writer) = uart_pipe.split();

    let mut buf = [0; 1];
    
    loop {
        let n = uart_rx.read(&mut buf).await.expect("UART read error");
        if n == 0 {continue;}
        let data = &buf[..n];
        trace!("UART IN: {:x}", buf);
        let _ = uart_tx.write(data).await;
    }

}