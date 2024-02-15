#![no_std]

use ::core::panic::PanicInfo;

pub use generic_ec::*;
pub use generic_ec_curves::*;
pub use generic_ec_zkp::*;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
