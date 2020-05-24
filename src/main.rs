#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial::WRITER.lock().init();

    println!("Hello World");
    // serial::print_something();
    // let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
    // serial_port.init();
    // serial_port.send(42);

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     serial_port.send(byte);
    // }
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
