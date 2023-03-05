#![no_std]
#![no_main]
mod lang_items;
mod peripherals;

use core::arch::global_asm;

use peripherals::UartBox;

#[macro_use]
extern crate bounded_registers;
#[macro_use]
extern crate typenum;

global_asm!(include_str!("entry.asm"));


#[no_mangle]
pub fn rust_main() -> ! {
    let mut uart = UartBox {
        address: 0x50000000
    };

    // 8N1 115200 non-FIFO polling
    uart.init();
    uart.put_str("Hello World!\n");
    loop {
        if let Some(c) = uart.recv_u8() {
            uart.put_u8(c);
        }
    }
}
