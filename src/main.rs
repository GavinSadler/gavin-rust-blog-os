// main.rs

// We do not have the standard libs in our os code
#![no_std]
// No C runtime == no main(), so we use _start instead
#![no_main]

use core::panic::PanicInfo;

// TODO: Write a note about this sfdkl;gjasfd
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\nPANIC! {}", _info);
    loop {}
}

mod vga_buffer;

// static HELLO: &[u8] = b"Hello world!";

// The real starting point of our program, since we don't have the C runtime to
// propel us into a main() function. Make sure we extern "C" to remain
// compatible with the correct ABI. Since this function doesn't return, we
// 'return' ! instead
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print!("Hello, with no newline!");
    println!("Hello, with a newline!");
    println!("Hello, with a newline! 😁");
    println!();
    print!("O___O");

    #[allow(unconditional_panic)]
    let x = 5 / 0;

    loop {}
}
