mod args;
mod execute;
mod cli;

use std::env;
use std::path::PathBuf;

pub struct ConfigInit {
    c: String,
    p: String,
    video: PathBuf,
}

impl ConfigInit {
    fn new() -> Self {
        Self {
            c: String::from("32"),
            p: String::from("medium"),
            video: PathBuf::from("INVALID"),
        }
    }

    fn has_video(&self) -> bool {
        args::is_video(self.video.to_str().unwrap_or(""))
    }
}

fn main() {
    // Default values for the command
    let mut config = ConfigInit::new();

    handle_args(&mut config);

    // DEBUG
    println!(
        "\nConfig: C: {}, P: {} \nVideo: {}\n",
        config.c,
        config.p,
        config.video.display()
    );

    if config.has_video() {
        let output = execute::execute(&config);
        println!("{output}");
    } else {
        eprintln!("Error: Missing a video");
        std::process::exit(1);
    }
}

fn handle_args(config: &mut ConfigInit) {

    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter().skip(1);

    // Loop through args and check if it's a flag or vid
    while let Some(flag) = args_iter.next() {
        match flag.to_lowercase().as_str() {
            "-h" => cli::print_help(),
            "-c" => args::compress(config, flag, args_iter.next()),
            "-p" => args::preset(config, flag, args_iter.next()),
            _ => {
                if args::is_video(flag) {
                    config.video = PathBuf::from(flag);
                } else if flag.starts_with("-") {
                    eprintln!("Error: Invalid flag \"{}\"", flag);
                    std::process::exit(1);
                } else {
                    eprintln!("Error: Invalid path or not a video \"{}\"", flag);
                    std::process::exit(1);
                }
            }
        }
    }

}