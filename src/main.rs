use chrono::prelude::*;
use colored::Colorize;
use std::{
    fs::{self, DirEntry},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    time::SystemTime,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "My ls")]
#[command(version = "1.0")]
#[command(about = "List your directory with me", long_about = None)]
struct CLI {
    /// The path to the directory
    #[arg(default_value_t = String::from("./"))]
    path: String,
    /// Display all information
    #[arg(short, long, value_name = "ALL", default_value_t = false)]
    all: bool,
    /// Display the date of creation
    #[arg(short, long, value_name = "CREATED", default_value_t = false)]
    created: bool,
    /// Display the date of last modification
    #[arg(short, long, value_name = "UPDATED", default_value_t = false)]
    updated: bool,
    /// Display the size of the file
    #[arg(short, long, value_name = "SIZE", default_value_t = false)]
    size: bool,
}

fn print_size(entry: &DirEntry) {
    if let Ok(meta) = entry.metadata() {
        print!(" {:>10} B |", meta.size());
    }
}

fn print_created(entry: &DirEntry) {
    if let Ok(meta) = entry.metadata() {
        let created = meta
            .created()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let datetime = DateTime::from_timestamp(created.as_secs() as i64, 0).unwrap();

        // Format the datetime how you want
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
        print!(" {:>20} |", newdate.to_string().green())
    }
}

fn print_updated(entry: &DirEntry) {
    if let Ok(meta) = entry.metadata() {
        let updated = meta
            .modified()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let datetime = DateTime::from_timestamp(updated.as_secs() as i64, 0).unwrap();

        // Format the datetime how you want
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
        print!(" {:>20} |", newdate.to_string().yellow())
    }
}

fn print_row(entry: &DirEntry, config: &CLI) {
    if let Ok(file_type) = entry.file_type() {
        if file_type.is_dir() {
            print!("| {:<20} |", entry.path().display().to_string().blue());
        } else {
            print!("| {:<20} |", entry.path().display());
        }
        if config.size || config.all {
            print_size(entry);
        }
        if config.created || config.all {
            print_created(entry);
        }
        if config.updated || config.all {
            print_updated(entry);
        }
        println!();
    }
}

fn print_heading(cli: &CLI) {
    print!("|{:^22}|", "Name");
    if cli.size || cli.all {
        print!("{:^14}|", "Size");
    }
    if cli.created || cli.all {
        print!(" {:^21}|", "Created");
    }
    if cli.updated || cli.all {
        print!(" {:^21}|", "Updated");
    }
    print!("\n+{:-<22}+", "");
    if cli.size || cli.all {
        print!("{:-<14}+", "");
    }
    if cli.created || cli.all {
        print!("{:-<22}+", "");
    }
    if cli.updated || cli.all {
        print!("{:-<22}+", "");
    }
    println!();
}

fn run(cli: CLI) -> Result<(), Box<dyn std::error::Error>> {
    let mut paths: Vec<Result<DirEntry, std::io::Error>> =
        fs::read_dir(PathBuf::from(cli.path.clone()))?.collect();
    paths.reverse();

    print_heading(&cli);
    for path in &paths {
        if let Ok(entry) = path {
            print_row(entry, &cli);
        }
    }
    Ok(())
}

fn main() {
    let cli = CLI::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
