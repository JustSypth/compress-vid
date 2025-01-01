use std::env;

use crate::ConfigInit;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn help() {
    //.next() here doesnt actually skip index 0 but starts at index 0
    let program_name = env::args().next().unwrap_or_else(|| "program".to_string());
    
    println!("An ffmpeg wrapper for compressing videos with more ease.\n");
    println!("Usage: {} [OPTION]... [FILE]...", program_name);
    
    // Main options
    let options = [
        ("-c, --crf <0-51>", "Compression intensity (0 = lossless, 51 = worst quality)"),
        ("-p, --preset <value>", "Encoding speed (ultrafast, veryfast, medium, slow, veryslow)"),
        ("    --help", "display this help and exit"),
        ("    --version", "output version information and exit (not yet implemented)"),
    ];
    let arguments = [("FILE", "Input video file to compress")];

    println!("\nOptions:");
    for (opt, desc) in options.iter() {
        println!("  {:<25} {}", opt, desc);
    }

    println!("\nArguments:");
    for (arg, desc) in arguments.iter() {
        println!("  {:<25} {}", arg, desc);
    }

    std::process::exit(0);
}

pub fn version() {
    println!("compress-vid {}", VERSION);
    std::process::exit(0);
}

pub fn debug(config: &ConfigInit) {
    println!(
        "Crf: {}, Preset: {}\nVideo: {}", 
        config.c, config.p, config.video.display()
    );
}
