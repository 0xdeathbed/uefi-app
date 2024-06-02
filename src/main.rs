#![no_std] // disable linking standard library
#![no_main] // disable entry function based on crt0
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::Vec;
use core::{alloc::Layout, fmt::Write};
use log::info;
use uefi::prelude::*;

// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

/*
https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html

This is the main entry point for a UEFI Image. This entry point is the same for
UEFI applications and UEFI drivers.

typedef
EFI_STATUS
(EFIAPI *EFI_IMAGE_ENTRY_POINT) (
  IN EFI_HANDLE                  ImageHandle,
  IN EFI_SYSTEM_TABLE            *SystemTable
  );

ImageHandle
The firmware allocated handle for the UEFI image.

SystemTable
A pointer to the EFI System Table.

`efi_main` is common convention for UEFI applications and Rust compiler will look for it by default
*/
// #[no_mangle]
// pub extern "efiapi" fn efi_main(image: *mut c_void, system_table: *const c_void) -> usize {
//     loop {}
// }

#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // initialize the allocator and loggings
    uefi::helpers::init(&mut system_table).unwrap();
    // initialize the allocator
    // unsafe {
    //     uefi::allocator::init(&mut system_table);
    // }

    let stdout = system_table.stdout();
    stdout.clear().unwrap();
    writeln!(stdout, "Hello World!").unwrap();

    // let rev = system_table.uefi_revision();
    // info!("UEFI {}.{}", rev.major(), rev.minor());
    // system_table.boot_services().stall(10_000_000); // Stall for 10 seconds

    // Using the allocator
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    // info!("v = {v:?}");
    writeln!(stdout, "v = {v:?}").unwrap();

    loop {}
}

/// This function is called when an allocation fails,
/// typically because the system is out of memory
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("out of memory")
}
