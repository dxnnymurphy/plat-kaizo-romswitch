use anyhow::{Result, Context};
use log;

pub const VANILLA_DS_CODE: &[u8; 4] = b"CPUE";
pub const RAW_SAVE_SIZE: usize = 524288;

/// ROM Patching
/// Platinum Kaizo uses the internal DS game code of JAK7, Melon DS cannot read this so saves
/// as 8kb file instead of 256kb, by patching in the vanilla Platinum code of CPUE this resolves.
pub fn crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;

    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }

    crc
}

pub fn patch_rom_game_code(rom: &mut [u8], new_code: &[u8; 4]) -> Result<()> {
    if rom.len() < 0x160 {
        return Err(anyhow::anyhow!("Rom too small"));
    }

    rom[0x0C..0x10].copy_from_slice(new_code);

    let crc = crc16_ccitt(&rom[0x000..0x15E]);
    rom[0x15E..0x160].copy_from_slice(&crc.to_le_bytes());

    Ok(())
}

pub fn read_rom_current_code(rom: &[u8]) -> Result<[u8; 4]> {
    if rom.len() < 0x160 {
        return Err(anyhow::anyhow!("Rom too small"));
    }

    let mut code = [0u8; 4];
    code.copy_from_slice(&rom[0x0C..0x10]);

    Ok(code)
}

pub fn patch_rom(filename: String, output_filename: Option<String>) -> Result<()> {
    use std::fs;
    use std::str;
    
    let mut rom_contents = fs::read(&filename).context(format!("Failed to read input ROM file {}", &filename))?;

    let current_code = read_rom_current_code(&rom_contents)?;
    let str_current_code = str::from_utf8(&current_code).context("Invalid UTF-8 in ROM code.")?;

    log::info!("Current ROM DS code {} | Patching to CPUE", str_current_code);

    patch_rom_game_code(&mut rom_contents, VANILLA_DS_CODE)?;

    let output = output_filename.unwrap_or(filename);

    fs::write(&output, rom_contents)?;
    log::info!("Patched ROM written to: {}", output);

    Ok(())
}

pub fn patch_dsv_savefile(filename: String, template_filename: String, output_filename: Option<String>) -> Result<()> {
    use std::fs;
    
    let raw_contents = fs::read(&filename).context(format!("Failed to read raw save data file {}", &filename))?;
    if raw_contents.len() != RAW_SAVE_SIZE {
        return Err(anyhow::anyhow!("Save content length mismatch, expected = {}, actual = {}", RAW_SAVE_SIZE, raw_contents.len()));
    }
    
    let template_contents = fs::read(&template_filename).context(format!("Failed to read raw save data file {}", &template_filename))?;
    if template_contents.len() <= RAW_SAVE_SIZE {
        return Err(anyhow::anyhow!("Template content length mismatch, expected > {}", RAW_SAVE_SIZE));
    }

    let mut out = Vec::with_capacity(template_contents.len());
    out.extend_from_slice(&raw_contents);
    out.extend_from_slice(&template_contents[RAW_SAVE_SIZE..]);

    let output = output_filename.unwrap_or_else(|| {
        let stem = filename.strip_suffix(".sav").unwrap_or(&filename);
        format!("{}.dsv", stem)
    });

    fs::write(&output, out)?;
    log::info!("Converted save file written to: {}", output);

    Ok(())
}

pub fn extract_sav_from_dsv(filename: String, output_filename: Option<String>) -> Result<()> {
    use std::fs;
    
    let dsv = fs::read(&filename).context(format!("Failed to read dsv data file {}", &filename))?;
    
    if dsv.len() < RAW_SAVE_SIZE {
        return Err(anyhow::anyhow!("DSV file smaller than expected, expected > {}", RAW_SAVE_SIZE));
    }

    let output = output_filename.unwrap_or_else(|| {
        let stem = filename.strip_suffix(".dsv").unwrap_or(&filename);
        format!("{}.sav", stem)
    });

    let file_data = &dsv[..RAW_SAVE_SIZE];

    fs::write(&output, file_data)?;
    log::info!("Extracted save file written to: {}", output);

    Ok(())
}