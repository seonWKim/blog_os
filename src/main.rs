#![no_std]
#![no_main]
/// We don't want to use the normal entry point
use core::panic::PanicInfo;

/// Defines the function that the compiler should invoke when a panic occurs
/// standard lib usually provides this function, but as we are using no_std, let's define our own
/// Returning `!` means that the function is returning `never` type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// To ensure Rust compiler doesn't change our function name
/// extern "C" is used to tell the compiler that it should use the C calling convention for this  function(instead of the unspecified Rust calling convention)
/// This entry point is not called by any function, but invoked directly by the OS or bootloader. So instead of returning, the entry point should invoke the `exit` system call
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
