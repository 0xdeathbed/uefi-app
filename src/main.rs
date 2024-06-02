#![no_std] // disable linking standard library
#![no_main] // disable entry function based on crt0
#![feature(alloc_error_handler)]

// link alloc crate
extern crate alloc;

use alloc::vec::Vec;
use core::{alloc::Layout, fmt::Write};
use log::info;
use uefi::{prelude::*, table::cfg};

/*
https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html

This is the main entry point for a UEFI Image. This entry point is the same for
UEFI applications and UEFI drivers.

`efi_main` is common convention for UEFI applications and Rust compiler will look for it by default
*/
#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // initialize the allocator and loggings
    uefi::helpers::init(&mut system_table).unwrap();
    let sys_table = uefi::helpers::system_table();

    let stdout = system_table.stdout();
    stdout.clear().unwrap();
    writeln!(stdout, "Hello World!").unwrap();

    let rev = sys_table.uefi_revision();
    info!("UEFI {}.{}", rev.major(), rev.minor());

    // Using the allocator
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    // info!("v = {v:?}");
    writeln!(stdout, "v = {v:?}").unwrap();

    let config_entries = sys_table.config_table();
    // adddress of relevant ACPI Root System Description table
    let rsdp_addr = config_entries
        .iter()
        .find(|&e| matches!(e.guid, cfg::ACPI_GUID | cfg::ACPI2_GUID))
        .map(|e| e.address);

    writeln!(stdout, "rsdp addr: {:?}", rsdp_addr).unwrap();

    loop {}
}

/// This function is called when an allocation fails,
/// typically because the system is out of memory
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("out of memory")
}
