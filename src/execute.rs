use std::process::Command;

use crate::ConfigInit;

pub fn execute(config: &ConfigInit) -> String {
    // Add "-output" to output filename
    let output_path = config.video.with_file_name(format!(
        "{}-output.{}",
        config.video.file_stem().unwrap().to_str().unwrap(),
        config.video.extension().unwrap().to_str().unwrap()
    ));

    let execute_arg = format!(
        "ffmpeg -i '{}' -vcodec libx264 -crf {} -preset {} -acodec aac -b:a 128k -y '{}'",
        &config.video.display(),
        &config.c,
        &config.p,
        &output_path.display()
    );

    let execute = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(&execute_arg)
            .output()
            .expect("Failed to execute ffmpeg.")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&execute_arg)
            .output()
            .expect("Failed to execute ffmpeg.")
    };

    let output = if execute.status.success() {
        "Process completed successfully".to_string()
    } else {
        format!("Process failed with status: {}", execute.status)
    };

    output
}
