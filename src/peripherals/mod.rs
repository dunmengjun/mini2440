mod uart;
mod uart_register;

pub use uart::UartBox;

// static mut TAKEN: bool = false;

// pub struct Peripherals {
//     pub uart: UartBox,
// }

// impl Peripherals {

//     #[inline]
//     pub fn take() -> Option<Self> {
//         if unsafe { TAKEN } {
//             None
//         } else {
//             Some(unsafe { Peripherals::steal() })
//         }
//     }

//     #[inline]
//     unsafe fn steal() -> Self {
//         TAKEN = true;
//         Self { uart: Uart::new() }
//     }
// }