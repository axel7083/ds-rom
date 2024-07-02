use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem::size_of,
    path::PathBuf,
};

use anyhow::{bail, Result};
use clap::Parser;
use nds_io::{crypto::blowfish::Blowfish, rom::raw};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Nintendo DS game ROM
    #[arg(short = 'r', long)]
    rom: PathBuf,

    /// Shows the contents of the ROM header
    #[arg(short = 'H', long)]
    show_header: bool,

    /// Nintendo DS ARM7 BIOS file
    #[arg(short = '7', long)]
    arm7_bios: Option<PathBuf>,

    /// Prints the contents of the ARM9 program. If an ARM7 BIOS is provided, the contents will be decrypted.
    #[arg(short = 'n', long)]
    print_arm9: bool,

    /// Encrypts the secure area.
    #[arg(short = 'e', long)]
    encrypt: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let key = if let Some(arm7_bios) = args.arm7_bios {
        let mut file = File::open(arm7_bios)?;
        let size = file.metadata()?.len() as usize;
        if size < 0x30 + size_of::<Blowfish>() {
            bail!("No key found in ARM7 BIOS, file should be at least {} bytes long", size_of::<Blowfish>());
        }
        let mut key = [0u8; size_of::<Blowfish>()];
        file.seek(SeekFrom::Start(0x30))?;
        file.read_exact(&mut key)?;
        Some(key)
    } else {
        None
    };

    let rom = raw::Rom::from_file(args.rom)?;
    let header = rom.header()?;
    let arm9 = {
        let mut arm9 = rom.arm9()?;
        if arm9.is_encrypted() && key.is_some() {
            let Some(key) = key else { unreachable!() };
            let gamecode = u32::from_le_bytes(header.gamecode.0);
            arm9.decrypt(&key, gamecode)?;
        }
        if args.encrypt && !arm9.is_encrypted() && key.is_some() {
            let Some(key) = key else { unreachable!() };
            let gamecode = u32::from_le_bytes(header.gamecode.0);
            arm9.encrypt(&key, gamecode)?;
        }
        arm9
    };

    if args.show_header {
        println!("ROM header:\n{}", header.display(2));
    }

    if args.print_arm9 {
        print_hex(arm9.as_ref());
    }
    Ok(())
}

fn print_hex(data: &[u8]) {
    for (offset, chunk) in data.chunks(16).enumerate() {
        print!("{:08x}", offset * 16);
        for byte in chunk {
            print!(" {byte:02x}");
        }
        println!();
    }
}
