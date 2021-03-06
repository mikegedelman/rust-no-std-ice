#![no_std]
#![feature(lang_items)]

#[lang = "start"]
fn lang_start() {}

#[panic_handler]
fn panic(_: &::core::panic::PanicInfo) -> ! {loop {}}
