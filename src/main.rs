#![no_std] // disable linking standard library
#![no_main] // disable entry function based on crt0
#![feature(alloc_error_handler)]

// link alloc crate
extern crate alloc;

use alloc::vec::Vec;
use core::{alloc::Layout, fmt::Write};
use log::info;
use uefi::{
    prelude::*,
    proto::{
        console::gop::GraphicsOutput,
        device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly},
        loaded_image::LoadedImage,
    },
    table::{
        boot::{MemoryType, SearchType},
        cfg,
    },
    Identify, Result,
};

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
    print_image_path(sys_table.boot_services()).unwrap();

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

    // Graphic Output Protocol for writing
    let bt = sys_table.boot_services();

    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = bt
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();

    writeln!(stdout, "current gop mode: {:?}", gop.current_mode_info()).unwrap();
    writeln!(
        stdout,
        "framebuffer at: {:#p}",
        gop.frame_buffer().as_mut_ptr()
    )
    .unwrap();

    // // estimated map size
    // let mmap_size = bt.memory_map_size();
    // let max_mmap_size = mmap_size.map_size + 8 * mem::size_of::<MemoryDescriptor>();
    // let mut buffer: Vec<u8> = Vec::with_capacity(max_mmap_size);
    // unsafe {
    //     buffer.set_len(max_mmap_size);
    // }

    // let mmap_storage = bt.memory_map(&mut buffer).unwrap();

    uefi::allocator::exit_boot_services();
    let (_sys_table, _mem_map) =
        uefi::helpers::system_table().exit_boot_services(MemoryType::LOADER_DATA);

    loop {}
}

fn print_image_path(bt: &BootServices) -> Result {
    let loaded_img = bt.open_protocol_exclusive::<LoadedImage>(bt.image_handle())?;

    let device_path_to_text_handle = *bt
        .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
        .first()
        .expect("DevicePathToText is missing");

    let device_path_to_text =
        bt.open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle)?;

    let image_device_path = loaded_img.file_path().expect("File path is not set");

    let image_device_path_text = device_path_to_text.convert_device_path_to_text(
        bt,
        image_device_path,
        DisplayOnly(true),
        AllowShortcuts(false),
    )?;

    info!("Image path: {}", &*image_device_path_text);

    Ok(())
}

/// This function is called when an allocation fails,
/// typically because the system is out of memory
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("out of memory")
}
