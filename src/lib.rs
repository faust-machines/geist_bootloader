use std::process::Command;
use std::error::Error;

const BUILD_PATH: &str = "geist_ws/src/geist";
const IMAGE_NAME: &str = "geist";
const CONTAINER_NAME: &str = "geist";

/// Starts the bootloader.
pub async fn start() -> Result<(), Box<dyn Error>> {
    let tag = "latest";
    let image_name = format!("{}:{}", IMAGE_NAME, tag);
    let home = std::env::var("HOME").unwrap();
    let build_path = format!("{}/{}", home, BUILD_PATH);

    println!("Building Docker image from: {}", build_path);

    // Building the Docker image
    let build_status = Command::new("bash")
        .current_dir(build_path)
        .arg("-c")
        .arg(format!("docker build -t {} .", image_name))
        .status()?;

    if !build_status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Docker build command failed",
        )));
    }

    println!("Docker image built successfully.");
    Ok(())

}

pub async fn stop() -> Result<(), Box<dyn Error>> {
    let stop_status = Command::new("bash")
        .arg("-c")
        .arg(format!("docker stop {}", CONTAINER_NAME))
        .status()?;

    if !stop_status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Docker stop command failed",
        )));
    }

    println!("{} stopped successfully.", CONTAINER_NAME);
    Ok(())
}

/// Returns the version of the bootloader.
pub async fn version() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME").unwrap();
    let version_path = format!("{}/{}/VERSION", home, BUILD_PATH);

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("cat {}", version_path))
        .output()?;

    // print the output
    println!("Geist Version:");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

/// Updates the bootloader.
pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME").unwrap();
    let build_path = format!("{}/{}", home, BUILD_PATH);

    Command::new("bash")
        .arg("-c")
        .arg("docker system prune")
        .output()?;

    // Docker system prune
    _ = Command::new("bash")
        .current_dir(build_path)
        .arg("prune")
        .output()?;

    // git pull
    std::process::Command::new("bash")
        .arg("-c")
        .current_dir("~/geist_ws/src/geist")
        .arg(format!("git pull origin main"))
        .output()?;

    Ok(())
}