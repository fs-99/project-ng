#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uart_16550::SerialPort;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // SAFETY: the base address should be right
    // TODO: all port addresses could be bound only once, preventing access by someone else
    let mut serial_port = unsafe {
        SerialPort::new(0x3F8)
    }; 
    serial_port.init();
    // TODO: I left out Mutex here for now
    let mut printer = DebugPrinter::new(serial_port);

    b(&mut printer);

    serial_println!(printer, "test");

    loop {}
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($printer:expr, $($arg:tt)*) => {
        $printer._print(format_args!($($arg)*));
    };
}
/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    ($printer:expr) => ($crate::serial_print!($printer, "\n"));
    ($printer:expr, $fmt:expr) => ($crate::serial_print!($printer, concat!($fmt, "\n")));
    ($printer:expr, $fmt:expr, $($arg:tt)*) => ($crate::serial_print!($printer, concat!($fmt, "\n"), $($arg)*));
}

fn b(printer: &mut DebugPrinter) {
    serial_println!(printer, "test");
}

struct DebugPrinter {
    serial: SerialPort,
}

impl DebugPrinter {
    pub fn new(serial: SerialPort) -> Self {
        Self { serial }
    }
    pub fn _print(&mut self, args: ::core::fmt::Arguments) {
        use core::fmt::Write;
        self.serial.write_fmt(args).expect("Printing to serial failed");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
