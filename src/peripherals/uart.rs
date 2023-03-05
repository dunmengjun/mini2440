use core::ops::{Deref, DerefMut};

use super::uart_register::{
    UartBaudRateDivisor, UartControl, UartErrorStatus, UartFIFOControl, UartFIFOStatus,
    UartLineControl, UartModemControl, UartModemStatus, UartReceiveBuffer, UartTransmitBuffer,
    UartTxRxStatus,
};

#[repr(C)]
pub struct Uart {
    line_contorl: UartLineControl::Register,
    contorl: UartControl::Register,
    fifo_contorl: UartFIFOControl::Register,
    modem_contorl: UartModemControl::Register,
    txrx_status: UartTxRxStatus::Register,
    error_status: UartErrorStatus::Register,
    fifo_status: UartFIFOStatus::Register,
    modem_status: UartModemStatus::Register,
    transmit_buffer: UartTransmitBuffer::Register,
    receive_buffer: UartReceiveBuffer::Register,
    baud_rate_divisor: UartBaudRateDivisor::Register,
}

impl Uart {
    pub fn init(&mut self) {
        self.contorl.modify(UartControl::ClockSelection::Clear);
        self.line_contorl.modify(UartLineControl::WordLength::Bits8 
            + UartLineControl::NumberOfStopBit::Clear 
            + UartLineControl::ParityMode::Clear 
            + UartLineControl::InfraredMode::Clear);
        self.contorl.modify(UartControl::ReceiveMode::Polling 
            + UartControl::TransmitMode::Polling);
        self.fifo_contorl.modify(UartFIFOControl::FIFOEnable::Clear);
        self.modem_contorl.modify(UartModemControl::AutoFlowControl::Clear);
        self.baud_rate_divisor.modify(UartBaudRateDivisor::UBRDIV::B26);
        self.contorl.modify(UartControl::ClockSelection::PCLK);
    }

    fn send_flag(&self) -> u32 {
        self.txrx_status.get_field(UartTxRxStatus::TransmitterEmpty::Read).unwrap().val()
    }

    fn recv_flag(&self) -> u32 {
        self.txrx_status.get_field(UartTxRxStatus::ReceiveBufferDataReady::Read).unwrap().val()
    }

    pub fn put_u8(&mut self, c: u8) {
        while self.send_flag() != 1 {}
        unsafe {
            self.transmit_buffer.write(c as u32);
        }
    }

    pub fn recv_u8(&self) -> Option<u8> {
        if self.recv_flag() == 1 {
            Some(self.receive_buffer.read() as u8)
        } else {
            None
        }
    }

    pub fn put_str(&mut self, str: &str) {
        for e in str.as_bytes() {
            self.put_u8(*e)
        }
    }
}

pub struct UartBox {
    pub address: usize,
}

impl Deref for UartBox {
    type Target = Uart;

    fn deref(&self) -> &Uart {
        unsafe { &*(self.address as *const Uart) }
    }
}

impl DerefMut for UartBox {
    fn deref_mut(&mut self) -> &mut Uart {
        unsafe { &mut *(self.address as *mut Uart) }
    }
}
