use anyhow::Result;
use clap::{Parser, Subcommand};
use log;
use plat_kaizo_romswitch::{patch_rom, patch_dsv_savefile, extract_sav_from_dsv};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Command
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    PatchRom {
        input_file: String,
        output_file: Option<String>
    },
    SavToDsv {
        input_file: String,
        template_file: String,
        output_file: Option<String>
    },
    DsvToSav {
        input_file: String,
        output_file: Option<String>
    }
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_secs()
        .init();

    if let Err(e) = run() {
        log::error!("Encountered error: {:#?}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Command::PatchRom{input_file, output_file} => patch_rom(input_file, output_file)?,
        Command::SavToDsv { input_file, template_file, output_file } => patch_dsv_savefile(input_file, template_file, output_file)?,
        Command::DsvToSav { input_file, output_file } => extract_sav_from_dsv(input_file, output_file)?,
    }
    
    Ok(())
}