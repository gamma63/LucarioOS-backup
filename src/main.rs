#![no_std]
#![no_main]
#![no_builtins]
#![feature(lang_items)]
#![feature(panic_info_message)]

#[macro_use]
mod log;
mod conv;
mod display;
mod multiboot;
mod ports;

use core::panic::PanicInfo;

use multiboot::MultibootHeader;

use crate::{display::{real_canvas::Canvas, console::TTY}, log::log::*, ports::com_init};
use crate::conv::fmt::Hexadecimal;

#[no_mangle]
#[allow(arithmetic_overflow)]
pub unsafe extern "C" fn _start(multiboot_addr: u32, _stack_top: u32) -> ! {
    com_init(DEBUG_PORT);

    debug!("Hello world from Rust!", 12345);

    let mb: *const MultibootHeader = multiboot_addr as *const MultibootHeader;
    let addr = (*mb).framebuffer_addr as usize;

    let width = (*mb).framebuffer_width as usize;
    let height = (*mb).framebuffer_height as usize;
    let fb_bpp = (*mb).framebuffer_bpp as usize;
    let fb_pitch = (*mb).framebuffer_pitch as usize;

    debug!("Screen address:", Hexadecimal::Unsigned(addr));
    debug!("Screen width:", width);
    debug!("Screen height:", height);
    
    let canvas = Canvas {
        buffer: addr as *mut u8,
        width,
        height,
        pitch: fb_pitch,
        bpp: fb_bpp,
    };

    // let canvas = Canvas::from_multiboot(mb);  // Needs memory?


    /*let mut console = TTY {
        canvas,
        x: 0,
        y: 0,
        color: 0xffffff
    };*/

    let mut console = TTY::new(&canvas);

    console.puts("Hyvaa yota, Valery Artemovich!\n");
    console.puts("0_0 I made a console?\n");
    
    canvas.pixel(40, 50, 0xff0000);

	// panic!("WHAT?");

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
extern "C" fn __eh_personality() {}

#[panic_handler]
#[no_mangle]
extern "Rust" fn __panic_handler(info: &PanicInfo) -> ! {
    // debug!("Panic encountered! ", file!(), " : --");
    // debug!("Panic! Message: ", info.message().unwrap().as_str().unwrap());
    loop {}
}
