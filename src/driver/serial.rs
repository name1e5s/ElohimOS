/// Simple Serial driver for boot the device.
/// Ref: https://www.xilinx.com/support/documentation/ip_documentation/axi_uart16550/v2_0/pg143-axi-uart16550.pdf
use bitflags::bitflags;
use core::fmt;
use x86_64::instructions::port::Port;

bitflags! {
    /// Line status register
    struct LineStatus : u8 {
        const INPUT_FULL = 1;
        const OUTPUT_EMPTY = 1 << 5;
    }
}

/// Abstract of a serial port
pub struct Serial {
    data: Port<u8>,
    interrupt_enable: Port<u8>,
    fifo_control: Port<u8>,
    line_control: Port<u8>,
    modem_control: Port<u8>,
    line_status: Port<u8>,
}

impl Serial {
    pub const unsafe fn new(base: u16) -> Self {
        Serial {
            data: Port::new(base),
            interrupt_enable: Port::new(base + 1),
            fifo_control: Port::new(base + 2),
            line_control: Port::new(base + 3),
            modem_control: Port::new(base + 4),
            line_status: Port::new(base + 5),
        }
    }

    /// Init the serial port
    pub fn init(&mut self) {
        unsafe {
            // Disable interrupts
            self.interrupt_enable.write(0x00);

            // Enable DLAB
            self.line_control.write(0x80);

            // Baud rate as 38400
            self.data.write(0x03);
            self.interrupt_enable.write(0x00);

            // Disable DLAB, set wprd length to 8 bits
            self.line_control.write(0x03);

            // Enable FIFO
            self.fifo_control.write(0xc7);

            // Mark data terminal ready
            self.modem_control.write(0x0b);

            // Enable interrupts
            self.interrupt_enable.write(0x01);
        }
    }

    fn line_status(&mut self) -> LineStatus {
        unsafe { LineStatus::from_bits_truncate(self.line_status.read()) }
    }

    // Send a byte to serial port
    pub fn send(&mut self, data: u8) {
        unsafe {
            match data {
                8 | 0x7F => {
                    while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {}
                    self.data.write(8);
                    while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {}
                    self.data.write(b' ');
                    while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {}
                    self.data.write(8)
                }
                _ => {
                    while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {}
                    self.data.write(data);
                }
            }
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}
