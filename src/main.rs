#![no_std] // disable linking standard library
#![no_main] // disable entry function based on crt0

use core::{fmt::Write, panic::PanicInfo};
use uefi::{
    entry,
    table::{Boot, SystemTable},
    Handle, Status,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

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
    let stdout = system_table.stdout();
    stdout.clear().unwrap();
    writeln!(stdout, "Hello World!").unwrap();

    loop {}
}
