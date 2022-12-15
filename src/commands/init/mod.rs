use anyhow::Result;
use clap::Parser;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::util::{Print, SEP};

#[derive(Debug, Parser)]
#[clap(about = "Initialize configuration file.")]
pub struct Options {
    #[clap(short = 'f', long = "force", help = "Overwrite existing config file.")]
    force: bool,
    #[clap(
        short = 'p',
        long = "packed",
        help = "Generate a packed configuration file."
    )]
    packed: bool,
}

fn create_config(is_packed: bool) -> Result<(), (i32, &'static str)> {
    let current = match env::current_dir() {
        Ok(current) => current,
        Err(..) => return Err((2, "Unable to locate current directory.")),
    };
    let binding = current.to_string_lossy();
    let cwd_name = match binding.split(SEP).last() {
        Ok(..) => Ok(()),
        Err(..) => return Err((2, "")),
    };
    let mut file = match File::create(".kernelrc") {
        Ok(file) => file,
        Err(..) => return Err((2, "Unable to locate current working directory.")),
    };

    let packed_json = format!("{{\n\"kernel\": \"{}\",\n\"kernel_type\": \"packed\",\n\"dev_cmd\": \"popcorn dev\",\n\"seed_cmd\": \"go build -o @dest\",\n\"advanced\": {{\n\"dev_node\": \"-dev\"\n}}\n}}", cwd_name).into_bytes();
    let unpacked_json = format!("{{\n\t\"kernel_name\": \"{}\",\n\t\"kernel_type\": \"unpacked\",\n\t\"unpacked_husk\": \"python @local/popcorn.py @args\",\n\t\"dev_cmd\": \"popcorn dev\",\n\t\"seed_cmd\": \"cp -r * @dest\",\n\t\"advanced\": {{\n\t\t\"dev_node\":  \"-dev\"\n\t}}\n}}", cwd_name).into_bytes();

    if is_packed {
        return match file.write_all(&*packed_json) {
            Ok(..) => Ok(()),
            Err(..) => return Err((2, "")),
        };
    } else {
        return match file.write_all(&*unpacked_json) {
            Ok(..) => Ok(()),
            Err(..) => return Err((2, "")),
        };
    }
}

pub async fn handle(options: Options) -> Result<(), i32> {
    if !Path::new(".kernelrc").exists() {
        return match create_config(options.packed) {
            Ok(..) => Ok(()),
            Err(..) => Err(2),
        };
    } else if options.force {
        return match create_config(options.packed) {
            Ok(..) => Ok(()),
            Err(..) => Err(2),
        };
    } else {
        return match Print::info(".kernelrc already exists; no changes made.") {
            Ok(_) => Ok(()),
            Err(_) => Err(2),
        };
    }
}
