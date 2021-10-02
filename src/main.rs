mod html_page;
mod static_site;
mod command;
use static_site::StaticSite;
use command::Command;
use std::env;
use std::path;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> std::io::Result<()> {
    let command = command::get_command_from_args(env::args());

    let site_options = match command {
	Command::PrintHelp => {
            print_help();
            None
        }
        Command::PrintVersion => {
            print_version();
            None
        }
        Command::GenerateSite { input_path, output_dir_path } => Some((input_path, output_dir_path)),
    };

    if site_options == None {
        return Ok(());
    } 

    let (file_name, output_dir) = site_options.unwrap();
    let input_path = path::Path::new(&file_name);

    if input_path.is_dir() {
        let site = StaticSite::from_directory(input_path)?;
        site.create(path::Path::new(&output_dir))?;
    } else if input_path.is_file() {
        let site = StaticSite::from_file(input_path);
        site.create(path::Path::new(&output_dir))?;
    }

    Ok(())
}

fn print_help() {
    print_version();

    println!("USAGE:");
    println!("\t{} [OPTIONS]\n", PKG_NAME);

    println!("OPTIONS:");
    println!("\t-v, --version\t\t\tPrint the version of the compiled package");
    println!("\t-h, --help\t\t\tPrint this screen");
    println!("\t-i <PATH, --input <PATH>\tGenerate HTML files from TXT files. PATH can be a path to an individual file, or to a folder");
    println!("\n");
}

fn print_version() {
    println!("{} version {}", PKG_NAME, PKG_VERSION);
}
