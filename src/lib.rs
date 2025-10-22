// We do not have the standard libs in our os code
#![no_std]
// No C runtime == no main(), so we use _start instead
#![no_main]
// Enable custom test runners
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// We rename the test runner function to test_main and call it in _start. The
// test runner tries to call main, but we do #![no_main] above so it doesn't
// exist
#![reexport_test_harness_main = "test_main"]

// Make our modules public and accesible via this lib
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    // Define the Qemu exit port at 0xf4
    let mut port = Port::new(0xf4);

    unsafe {
        // Use u32 because the port is 4 bytes wide
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]")
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
