#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};

use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    let hello = b"Hello, World!"; // The prefix b makes it a straight binary array
    let vga_buffer = 0xb8000 as *mut u8; // 753664 in binary. We cast it to *mut u8, a mutable pointer of bytes
    let mut i = 0;
    for byte in hello.iter() { // iter allows us to loop over the bytes
        unsafe {
            *vga_buffer.add(i as usize * 2) = *byte; // Dereference the borrowed byte and buffer, make one part the character
            *vga_buffer.add(i as usize * 2 + 1) = 0xb // preset color
        }
        i += 1; // incremeent i
    }
    rust_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //println!("{}", info);
    rust_os::hlt_loop();
}