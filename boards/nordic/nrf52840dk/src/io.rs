// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

use core::fmt::Write;
use core::panic::PanicInfo;
use cortexm4;
use kernel::debug;
use kernel::debug::IoWrite;
use kernel::hil::led;
use kernel::hil::uart;
use kernel::hil::uart::Configure;
use nrf52840::gpio::Pin;

use crate::CHIP;
use crate::PROCESSES;
use crate::PROCESS_PRINTER;

enum Writer {
    WriterUart(/* initialized */ bool),
    WriterRtt(&'static capsules_extra::segger_rtt::SeggerRttMemory<'static>),
}

static mut WRITER: Writer = Writer::WriterUart(false);

// Wait a fixed number of cycles to avoid missing characters over the RTT console
fn wait() {
    for _ in 0..1000 {
        cortexm4::support::nop();
    }
}

/// Set the RTT memory buffer used to output panic messages.
pub unsafe fn set_rtt_memory(
    rtt_memory: &'static capsules_extra::segger_rtt::SeggerRttMemory<'static>,
) {
    WRITER = Writer::WriterRtt(rtt_memory);
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

impl IoWrite for Writer {
    fn write(&mut self, buf: &[u8]) -> usize {
        match self {
            Writer::WriterUart(ref mut initialized) => {
                // Here, we create a second instance of the Uarte struct.
                // This is okay because we only call this during a panic, and
                // we will never actually process the interrupts
                let uart = nrf52840::uart::Uarte::new();
                if !*initialized {
                    *initialized = true;
                    let _ = uart.configure(uart::Parameters {
                        baud_rate: 115200,
                        stop_bits: uart::StopBits::One,
                        parity: uart::Parity::None,
                        hw_flow_control: false,
                        width: uart::Width::Eight,
                    });
                }
                for &c in buf {
                    unsafe {
                        uart.send_byte(c);
                    }
                    while !uart.tx_ready() {}
                }
            }
            Writer::WriterRtt(rtt_memory) => {
                let up_buffer = unsafe { &*rtt_memory.get_up_buffer_ptr() };
                let buffer_len = up_buffer.length.get();
                let buffer = unsafe {
                    core::slice::from_raw_parts_mut(
                        up_buffer.buffer.get() as *mut u8,
                        buffer_len as usize,
                    )
                };

                let mut write_position = up_buffer.write_position.get();

                for &c in buf {
                    buffer[write_position as usize] = c;
                    write_position = (write_position + 1) % buffer_len;
                    up_buffer.write_position.set(write_position);
                    wait();
                }
            }
        };
        buf.len()
    }
}

#[cfg(not(test))]
#[no_mangle]
#[panic_handler]
/// Panic handler
pub unsafe extern "C" fn panic_fmt(pi: &PanicInfo) -> ! {
    // The nRF52840DK LEDs (see back of board)
    let led_kernel_pin = &nrf52840::gpio::GPIOPin::new(Pin::P0_13);
    let led = &mut led::LedLow::new(led_kernel_pin);
    let writer = &mut WRITER;
    debug::panic(
        &mut [led],
        writer,
        pi,
        &cortexm4::support::nop,
        &PROCESSES,
        &CHIP,
        &PROCESS_PRINTER,
    )
}
