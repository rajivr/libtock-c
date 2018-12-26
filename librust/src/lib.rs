#![no_std]
#![feature(asm, core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
pub unsafe extern "C" fn panic_fmt(_info: &PanicInfo) -> ! {
    intrinsics::abort();
}

#[no_mangle]
pub unsafe extern "C" fn rust_hello() {
    asm!("nop" :::: "volatile");
}
