use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;

// pub static PORT: SerialPort = unsafe { SerialPort::new(0x3f8) });

lazy_static! {
    pub static ref WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter {
        serial_port: unsafe { SerialPort::new(SERIAL_IO_PORT) },
    });
}

pub struct SerialWriter {
    serial_port: SerialPort,
}

impl SerialWriter {
    pub fn init(&mut self) {
        self.serial_port.init();
    }
    fn write_string(&mut self, s: &str) {
        for c in s.as_bytes() {
            self.serial_port.send(*c);
        }
    }
}

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
