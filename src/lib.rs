#![no_std]

#[no_mangle]
pub extern "C" fn foo(y: u32) -> u32 {
    y*y
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
