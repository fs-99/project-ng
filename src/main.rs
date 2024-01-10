#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

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
 
    // - needs to be static because of .load, to be safe
    // - putting it in a Mutex did not change that, .lock leads to a borrow checker error or it not being static
    // - this means TODO: we have to use static, but maybe we could put all globals in one place, or just get on with life
    static mut IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable::new());
    {
        let idt_lock = unsafe { IDT.get_mut() };
        idt_lock.breakpoint.set_handler_fn(breakpoint_handler);
        idt_lock.load();
    }
    
    // trigger breakpoint
    x86_64::instructions::interrupts::int3();

    serial_println!(printer, "Breakpoint: {:#?}", *BREAKPOINT_OUT.lock());

    loop {
        core::hint::spin_loop();
    }
}

static BREAKPOINT_OUT: Mutex<Option<InterruptStackFrame>> = Mutex::new(None);

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    *BREAKPOINT_OUT.lock() = Some(stack_frame);
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
