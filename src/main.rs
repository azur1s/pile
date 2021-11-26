use std::{env, os::linux::fs::MetadataExt};
use std::path::PathBuf;
use std::fs::metadata;

use byte_unit::Byte;

use chrono::{DateTime, Utc};

use colored::Colorize;

mod info;

fn main() {
    
    let path = PathBuf::from(env::args().nth(1).unwrap());

    let path_name = path.to_string_lossy();
    let file_name = path.file_name().expect("Invalid PathBuf.").to_string_lossy();
    let file_info = info::get_info(path.clone());

    let metadata = metadata(&path).expect("Invalid metadata."); 
    let raw_size = Byte::from_bytes(metadata.len().into());
    let modified_date: DateTime<Utc> = metadata.modified().expect("Unsupported.").into();
    let permission = metadata.st_uid();
    let inode = metadata.st_ino();

    let prefix = "● ";

    println!();
    println!("{} {} {}", prefix, &file_name, &path_name.bright_black());
    println!();
    println!("{} Info: {}", prefix.red(), &file_info);
    println!("{} Size: {} {}", prefix.yellow(), &raw_size.get_appropriate_unit(false), &raw_size.to_string().bright_black());
    println!("{} Modified: {} UTC", prefix.green(), &modified_date.format("%d/%m/%Y %T"));
    println!("{} Permissions: {}", prefix.blue(), &permission);
    println!("{} INode: {}", prefix.purple(), &inode);

}
