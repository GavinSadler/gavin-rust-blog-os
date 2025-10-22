// We do not have the standard libs in our os code
#![no_std]
// No C runtime == no main(), so we use _start instead
#![no_main]
// Enable custom test runners
#![feature(custom_test_frameworks)]
#![test_runner(gavin_rust_blog_os::test_runner)]
// We rename the test runner function to test_main and call it in _start. The
// test runner tries to call main, but we do #![no_main] above so it doesn't
// exist
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use gavin_rust_blog_os::{print, println};

// The real starting point of our program, since we don't have the C runtime to
// propel us into a main() function. Make sure we extern "C" to remain
// compatible with the correct ABI. Since this function doesn't return, we
// 'return' ! instead
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print!("Hello, with no newline{}", "!");
    println!("Hello, with a newline!");
    println!("Hello, with a newline! 😁");
    println!();
    print!("O___O");

    // #[allow(unconditional_panic)]
    // let x = 5 / 0;

    // Run test suites if in testing mode
    #[cfg(test)]
    test_main();

    loop {}
}

// Non-test panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\nPANIC! {}", info);
    loop {}
}

// Delegate test panicking to lib
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gavin_rust_blog_os::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
