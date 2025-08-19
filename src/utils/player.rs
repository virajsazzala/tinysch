use std::path::Path;
use tokio::process::Command;

pub async fn play_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        eprintln!("File not found: {}", path);
        return Ok(());
    }

    let status = Command::new("mpv").arg(path).status().await?;

    if status.success() {
        println!("Finished Playing: {}", path);
    } else {
        eprintln!("mpv exited with an error.");
    }

    Ok(())
}
