#![feature(abi_efiapi)]
#![no_std]
#![no_main]
use uefi::prelude::*;
use core::panic::PanicInfo;
use core::fmt::Write;
extern crate rlibc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {

    writeln!(st.stdout(), "Hello, UEFI!!").unwrap();

    loop {}
    // Status::SUCCESS
}