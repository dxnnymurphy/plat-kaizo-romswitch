# Plat-Kaizo-RomSwitch

A utility tool for patching Pokémon ROM game codes and converting save file formats.

## ⚠️ LEGAL DISCLAIMER

**This tool is designed to work with legally-owned ROM files only.**

- This tool modifies binary files. It does not create, distribute, or facilitate obtaining copyrighted material.
- Users are solely responsible for ensuring they own the original ROM files before using this tool.
- Intellectual property rights belong to their respective owners (Game Freak, The Pokémon Company, Nintendo).
- We do not condone piracy or copyright infringement.

**Use this tool at your own legal risk.**

---

## What This Tool Does

This utility performs three operations on DS game files:

1. **PatchRom** — Patches a Platinum Kaizo ROM
   - Patches the internal game code from JAK7 to CPUE (vanilla Platinum code)
   - Fixes save file size issues when using emulators like Melon DS
   - Recalculates CRC16-CCITT checksum automatically

2. **SavToDsv** — Converts raw .sav files into .dsv format (DeSmuME save format)
   - Requires a template .dsv file as reference to append the correct metadata

3. **DsvToSav** — Extracts raw .sav data from .dsv files for use back with Melon DS (or similar)

---

## Prerequisites

- A legally-owned copy of Pokémon Platinum (or ROM dump thereof)
- Rust toolchain (for building) or pre-built binary
- UNIX-like shell (Linux, macOS, WSL) or Windows with appropriate tools

---

## Installation

### From Source
```bash
git clone https://github.com/yourusername/plat-kaizo-romswitch.git
cd plat-kaizo-romswitch
cargo build --release
./target/release/plat-kaizo-romswitch --help
```

### Pre-built Binary
Download from Releases

### Why This Tool Exists
For converting Platinum Kaizo savefiles between Melon DS (with dual screen support for certain emulator hardware), and DeSmuME for certain bugs that occur with the Melon DS emulator.