use axum::response::Result;
use std::{
    path::{Path, PathBuf},
    process::ExitStatus,
};
use tokio::process::Command;

use notify_debouncer_full::DebouncedEvent;

pub async fn handle_event_async(pool: sqlx::PgPool, event: DebouncedEvent, output_base: String) {
    for path in &event.paths {
        if path.exists() && path.extension().map_or(false, |ext| ext == "mp4") {
            println!("🎬 New movie detected: {:?}", path);

            if let Err(e) = process_movie_lifecycle(&pool, path.clone(), &output_base).await {
                eprintln!("❌ Error processing movie: {}", e);
            }
        }
    }
}
async fn process_movie_lifecycle(
    pool: &sqlx::PgPool,
    path: PathBuf,
    output_base: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let title = path.file_stem().unwrap().to_str().unwrap().to_string();
    let movie_output = format!("{}/{}", output_base, title);
    let hls_index = format!("{}/master.m3u8", movie_output);
    let original_path = path.to_str().unwrap().to_string();
    // 1. Database: Mark as Processing
    let movie_id: uuid::Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO movies (title, original_path, status)
        VALUES ($1, $2, 'processing')
        RETURNING id
        "#,
        title,
        original_path,
    )
    .fetch_one(pool)
    .await?;

    // 2. Transcode: Non-blocking
    let status = run_transcode(&path, &movie_output).await?; // Notice the .await here!
    let _ = run_thumbnail(&path, &movie_output).await?;
    let duration = get_duration(&path).await?;

    // 3. Database: Mark as Completed
    if status.success() {
        sqlx::query!(
            r#"UPDATE movies SET status = 'completed', hls_path = $1, thumbnail_path = $2, duration_seconds = $3 WHERE id = $4"#,
            hls_index,
            format!("{}/thumbnail.jpg", movie_output),
            duration,
            movie_id
        )
        .execute(pool)
        .await?;
        println!("✅ {} is ready for streaming!", title);
    } else {
        sqlx::query!(
            r#"UPDATE movies SET status = 'failed' WHERE id = $1"#,
            movie_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn run_transcode(
    input: &Path,
    output_folder: &str,
) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    println!("Starting transcoding for: {:?}", input);

    std::fs::create_dir_all(output_folder).unwrap();

    let status = Command::new("ffmpeg")
        .args([
            // Input
            "-i",
            input.to_str().unwrap(),
            // Video codec (Apple Silicon hardware)
            "-c:v",
            "hevc_videotoolbox",
            "-q:v",
            "85",
            "-allow_sw",
            "1",
            // Keyframe settings
            "-keyint_min",
            "48",
            "-g",
            "48",
            "-sc_threshold",
            "0",
            // Quality Variant 1 (1080p)
            "-map",
            "0:v",
            "-map",
            "0:a",
            "-s:v:0",
            "1920x1080",
            "-b:v:0",
            "5000k",
            // Quality Variant 2 (720p)
            "-map",
            "0:v",
            "-map",
            "0:a",
            "-s:v:1",
            "1280x720",
            "-b:v:1",
            "2800k",
            // Quality Variant 3 (480p)
            "-map",
            "0:v",
            "-map",
            "0:a",
            "-s:v:2",
            "854x480",
            "-b:v:2",
            "1400k",
            // Audio codec
            "-c:a",
            "aac",
            "-b:a",
            "128k",
            // HLS output format
            "-start_number",
            "0",
            "-f",
            "hls",
            "-hls_time",
            "6",
            "-hls_list_size",
            "0",
            "-master_pl_name",
            "master.m3u8",
            "-var_stream_map",
            "v:0,a:0 v:1,a:1 v:2,a:2",
            &format!("{}/stream_%v.m3u8", output_folder),
        ])
        .status()
        .await?;

    if status.success() {
        println!("Transcoding completed successfully for: {:?}", input);
    } else {
        eprintln!(
            "Transcoding failed for: {:?} with status: {:?}",
            input, status
        );
    }

    Ok(status)
}

async fn run_thumbnail(
    input: &Path,
    output_folder: &str,
) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    println!("Starting thumbnail generation for: {:?}", input);

    std::fs::create_dir_all(output_folder).unwrap();

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-ss")
        .arg("00:00:05.000")
        .arg("-vframes")
        .arg("1")
        .arg("-q:v")
        .arg("2")
        .arg(format!("{}/thumbnail.jpg", output_folder))
        .status()
        .await?;

    if status.success() {
        println!("Thumbnail generated successfully for: {:?}", input);
    } else {
        eprintln!(
            "Thumbnail generation failed for: {:?} with status: {:?}",
            input, status
        );
    }

    Ok(status)
}

async fn get_duration(input: &Path) -> Result<i32, Box<dyn std::error::Error>> {
    println!("Getting duration for: {:?}", input);

    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(input)
        .output()
        .await?;

    if output.status.success() {
        let duration = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let duration_secs: i32 = duration.parse::<f32>().unwrap_or(0.0) as i32;
        println!("Duration for {:?}: {} seconds", input, duration);
        Ok(duration_secs)
    } else {
        eprintln!(
            "Failed to get duration for: {:?} with status: {:?}",
            input, output.status
        );
        Err("Failed to get duration".into())
    }
}
