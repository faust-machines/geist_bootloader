use std::process::Command;
use std::error::Error;

const BUILD_PATH: &str = "geist_ws/src/geist";
const IMAGE_NAME: &str = "geist";
const CONTAINER_NAME: &str = "geist";

/// builds geist
pub async fn build() -> Result<(), Box<dyn Error>> {
    println!("\n");
    println!("=== Building Geist ===");
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

pub async fn logs() -> Result<(), Box<dyn Error>> {
    println!("\n");
    println!("=== Geist Logs ===");
    let logs_status = Command::new("bash")
        .arg("-c")
        .arg(format!("docker logs -f {}", CONTAINER_NAME))
        .status()?;

    if !logs_status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Docker logs command failed",
        )));
    }

    Ok(())
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    println!("\n");
    println!("=== Starting Geist ===");
    let tag = "latest";
    let image_name = format!("{}:{}", IMAGE_NAME, tag);

    let run_command = format!(
        "docker run -it --rm \
        --name {} \
        --net=host \
        -d \
        --env=\"DISPLAY\" \
        --volume=\"/tmp/.X11-unix:/tmp/.X11-unix:rw\" \
        -p 9090:9090 \
        -p 9091:9091 \
        {} \
        /bin/bash -c \"source install/setup.sh && cd src/geist && ros2 launch geist/launch/launch.py\"",
        CONTAINER_NAME,
        image_name
    );

    // Starting the Docker container
    let start_status = Command::new("bash")
        .arg("-c")
        .arg(run_command)
        .status()?;

    if !start_status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Docker run command failed",
        )));
    }

    println!("{} started successfully.", CONTAINER_NAME);
    Ok(())
}

pub async fn stop() -> Result<(), Box<dyn Error>> {
    println!("\n");
    println!("=== Stopping Geist ===");
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
    println!("\n");
    println!("=== Geist Version ===");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

/// Updates the bootloader.
pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME").unwrap();
    let build_path = format!("{}/{}", home, BUILD_PATH);

    println!("\n");
    println!("=== Cleaning up Geist ===");
    println!("Pruning old unused images...");
    Command::new("bash")
        .arg("-c")
        .arg("docker system prune -f")
        .status()?;
    println!("Pruning complete.");

    // git pull
    println!("\n");
    println!("=== Updating Geist ===");
    Command::new("bash")
        .arg("-c")
        .current_dir(build_path)
        .arg(format!("git pull origin main"))
        .status()?;

    Ok(())
}