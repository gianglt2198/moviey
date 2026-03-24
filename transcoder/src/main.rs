use std::{path::Path, process::Command, time::Duration};

use notify_debouncer_full::{DebounceEventResult, DebouncedEvent, new_debouncer, notify::*};

fn main() {
    let input_dir = "./uploads";
    let output_dir = "./stream_output";

    // 1. Initialize the Debouncer (2-second delay)
    // This waits for the file to "stop changing" before firing the event
    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                for event in events {
                    handle_event(event, output_dir);
                }
            }
            Err(errors) => errors
                .iter()
                .for_each(|e| eprintln!("Watch error: {:?}", e)),
        },
    )
    .unwrap();

    // 2. Watch the input directory
    debouncer
        .watch(input_dir, RecursiveMode::NonRecursive)
        .expect("Failed to start watcher");

    println!("Watching directory: {}...", input_dir);

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn handle_event(event: DebouncedEvent, output_base: &str) {
    // We only care about "Create" or "Write" events that have finished
    for path in &event.paths {
        if !path.exists() {
            continue; // Skip this event, the file is gone
        }
        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext == "mp4" {
                println!("Detected new file: {:?}", path);

                // Create a unique output folder for this file
                let movie_name = path.file_stem().unwrap().to_str().unwrap();
                let movie_output = format!("{}/{}", output_base, movie_name);

                run_transcode(path, &movie_output);
            }
        }
    }
}

fn run_transcode(input: &Path, output_folder: &str) {
    println!("Starting transcoding for: {:?}", input);

    std::fs::create_dir_all(output_folder).unwrap();

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-profile:v")
        .arg("baseline")
        .arg("-level")
        .arg("3.0")
        .arg("-s")
        .arg("1280x720")
        .arg("-start_number")
        .arg("0")
        .arg("-hls_time")
        .arg("10")
        .arg("-hls_list_size")
        .arg("0")
        .arg("-f")
        .arg("hls")
        .arg(format!("{}/index.m3u8", output_folder))
        .status()
        .expect("FFmpeg failed to run");

    if status.success() {
        println!("Transcoding completed successfully for: {:?}", input);
    } else {
        eprintln!(
            "Transcoding failed for: {:?} with status: {:?}",
            input, status
        );
    }
}
