use std::fs;

use crate::bus::Bus;

pub fn save_sram(bus: &Bus) {
    let path = bus.rom.save_path();

    if bus.mbc1.ram().is_empty() {
        println!("SRAM: cartridge nie posiada RAM.");
        return;
    }

    match fs::write(&path, bus.mbc1.ram()) {
        Ok(_) => {
            println!(
                "SRAM zapisano ({} bajtów): {}",
                bus.mbc1.ram().len(),
                path.display()
            );
        }
        Err(e) => {
            eprintln!("SRAM SAVE ERROR: {}", e);
        }
    }
}

pub fn load_sram(bus: &mut Bus) {
    let path = bus.rom.save_path();

    if !path.exists() {
        println!("SRAM: brak pliku {}", path.display());
        return;
    }

    match fs::read(&path) {
        Ok(data) => {
            let ram = bus.mbc1.ram_mut();
            let len = ram.len().min(data.len());

            ram[..len].copy_from_slice(&data[..len]);

            println!(
                "SRAM wczytano ({} bajtów): {}",
                len,
                path.display()
            );
        }
        Err(e) => {
            eprintln!("SRAM LOAD ERROR: {}", e);
        }
    }
}