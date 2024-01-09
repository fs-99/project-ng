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
    
    /// Prints to the host through the serial interface.
    #[macro_export]
    macro_rules! serial_print {
        ($($arg:tt)*) => {
            printer._print(format_args!($($arg)*));
        };
    }

    serial_print!("test");

    loop {}
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
