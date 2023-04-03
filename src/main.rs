#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};

use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rust_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //println!("{}", info);
    rust_os::hlt_loop();
}