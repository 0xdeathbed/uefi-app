use anyhow::Result;
use std::{
    fs,
    io::{self, Seek},
    path::{Path, PathBuf},
};

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let efi_path = PathBuf::from(
        args.next()
            .expect("Path to `.efi` file must be given as argument"),
    );

    let fat_path = efi_path.with_extension("fat");
    let disk_path = fat_path.with_extension("gdt");

    let _ = create_fat_filesystem(&fat_path, &efi_path);
    let _ = create_gpt_disk(&disk_path, &fat_path);
}

fn create_gpt_disk(disk_path: &Path, fat_image: &Path) -> Result<()> {
    // create new file
    let mut disk = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&disk_path)?;

    // set file size
    let partition_size = fs::metadata(&fat_image)?.len();
    let disk_size = partition_size + 1024 * 64; // for GPT headers
    disk.set_len(disk_size)?;

    // create a protective MBR at LBA0 so that disk is not considered unformatted
    // on BIOS systems
    let mbr = gpt::mbr::ProtectiveMBR::with_lb_size(
        u32::try_from((disk_size / 521) - 1).unwrap_or(0xFF_FF_FF_FF),
    );
    mbr.overwrite_lba0(&mut disk)?;

    // create new GPT structure
    let block_size = gpt::disk::LogicalBlockSize::Lb512;
    let mut gpt = gpt::GptConfig::new()
        .writable(true)
        .initialized(false)
        .logical_block_size(block_size)
        .create_from_device(Box::new(&mut disk), None)?;
    gpt.update_partitions(Default::default())?;

    // add new EFI system partition and get its byte offset in the file
    let partition_id =
        gpt.add_partition("boot", partition_size, gpt::partition_types::EFI, 0, None)?;
    let partition = gpt.partitions().get(&partition_id).unwrap();
    let start_offset = partition.bytes_start(block_size)?;

    // close the GPT structure and write out changes
    gpt.write()?;

    // place the FAT file system in the newly created partition
    disk.seek(io::SeekFrom::Start(start_offset))?;
    io::copy(&mut fs::File::open(&fat_image)?, &mut disk)?;

    Ok(())
}

fn create_fat_filesystem(fat_path: &Path, efi_file: &Path) -> Result<()> {
    // retrieve size of `.efi` file and round it up
    let efi_size = fs::metadata(&efi_file)?.len();
    // size of a megabyte
    let mb = 1024 * 1024;
    // round it to next MB
    let efi_size_rounded = ((efi_size - 1) / mb + 1) * mb;

    // create new filesystem image file at the given path and set its length
    let fat_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&fat_path)?;

    fat_file.set_len(efi_size_rounded)?;

    // create new FAT file system and open it
    let format_opt = fatfs::FormatVolumeOptions::new();
    fatfs::format_volume(&fat_file, format_opt)?;

    let filesystem = fatfs::FileSystem::new(&fat_file, fatfs::FsOptions::new())?;

    // copy EFI file to FAT filesystem
    let root_dir = filesystem.root_dir();
    root_dir.create_dir("efi")?;
    root_dir.create_dir("efi/boot")?;

    let mut bootx64 = root_dir.create_file("efi/boot/bootx64.efi")?;
    bootx64.truncate()?;

    io::copy(&mut fs::File::open(&efi_file)?, &mut bootx64)?;

    Ok(())
}
