
#![no_std]
fn main() {
}

use core::panic::PanicInfo;

/// Defines the function that the compiler should invoke when a panic occurs
/// standard lib usually provides this function, but as we are using no_std, let's define our own
/// Returning `!` means that the function is returning `never` type
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {

    }
}
