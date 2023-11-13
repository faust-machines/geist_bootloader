use std::error::Error;
use std::process::Command;
use std::process::Output;

const BUILD_PATH: &str = "geist_ws/src/geist";
const IMAGE_NAME: &str = "geist";
const CONTAINER_NAME: &str = "geist";
const USERNAME: &str = "july5613";

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

pub async fn start(version: Option<String>) -> Result<(), Box<dyn Error>> {
    // lets make a version string
    let ver = match version {
        Some(ver) => ver,
        None => "latest".to_string(),
    };

    println!("\n");
    println!("=== Starting Geist ===");
    let image_name = format!("{}/{}:{}", USERNAME, IMAGE_NAME, ver);

    let run_command = format!(
        "docker run -it --rm \
        --name {} \
        --net=host \
        -d \
        --env=\"DISPLAY\" \
        --volume=\"/tmp/.X11-unix:/tmp/.X11-unix:rw\" \
        -v /dev/bus/usb:/dev/bus/usb --device-cgroup-rule='c 189:* rmw' \
        -p 9090:9090 \
        -p 9091:9091 \
        {} \
        /bin/bash -c \"source install/setup.sh && cd src/geist && ros2 launch geist/launch/launch.py\"",
        CONTAINER_NAME,
        image_name,
    );

    // Starting the Docker container
    let start_status = Command::new("bash").arg("-c").arg(run_command).status()?;

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

pub async fn list_services() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_docker_exec("ros2 service list")?;

    let filtered_output: Vec<String> = output
        .stdout
        .split(|c| c == &b'\n')
        .filter_map(|line| {
            let line_str = String::from_utf8_lossy(line);
            if !line_str.ends_with("/describe_parameters")
                && !line_str.ends_with("/get_parameters")
                && !line_str.ends_with("/list_parameters")
                && !line_str.ends_with("/set_parameters")
                && !line_str.ends_with("/get_parameter_types")
                && !line_str.ends_with("/set_parameters_atomically")
            {
                Some(line_str.to_string())
            } else {
                None
            }
        })
        .collect();

    // Joining the filtered lines back into a single string
    let joined_output = filtered_output.join("\n");
    println!("{}", joined_output);

    Ok(())
}

pub async fn call_service(
    address: String,
    type_: String,
    data: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let command = format!("ros2 service call {} {} \"{}\"", address, type_, data);
    let output = run_docker_exec(&command)?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub async fn list_topics() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_docker_exec("ros2 topic list")?;

    let filtered_output: Vec<String> = output
        .stdout
        .split(|c| c == &b'\n')
        .filter_map(|line| {
            let line_str = String::from_utf8_lossy(line);
            if !line_str.contains("/client_count")
                && !line_str.contains("/connected_clients")
                && !line_str.contains("/parameter_events")
                && !line_str.contains("/rosout")
            {
                Some(line_str.to_string())
            } else {
                None
            }
        })
        .collect();

    // If you want to print the filtered output as a single string
    let joined_output = filtered_output.join("\n");
    println!("{}", joined_output);

    Ok(())
}

pub async fn get_topic_type(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let command = format!("ros2 topic type {}", name);
    let output = run_docker_exec(&command)?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub async fn echo_topic(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let command = format!("ros2 topic echo {}", name);
    let output = run_docker_exec(&command)?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

// Helper function to run `docker exec`
fn run_docker_exec(command: &str) -> Result<Output, Box<dyn std::error::Error>> {
    let cmd = format!(
        "docker exec {} bash -c \"source install/setup.sh && {}\"",
        CONTAINER_NAME, command
    );
    let output = Command::new("bash").arg("-c").arg(cmd).output()?;

    if !output.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Docker exec command failed",
        )));
    }

    Ok(output)
}

/// Returns the version of geist that is running
pub async fn version() -> Result<(), Box<dyn std::error::Error>> {
    // Run `cat` inside the Docker container
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "docker exec {} cat /root/geist_ws/src/geist/VERSION",
            CONTAINER_NAME
        ))
        .output()?;

    if !output.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Docker exec command failed",
        )));
    }

    // Print the output
    println!("\n");
    println!("=== Geist Version ===");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

/// Updates the bootloader.
pub async fn update(version: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = format!("{}/{}", USERNAME, IMAGE_NAME);

    let ver = match version {
        Some(ver) => ver,
        None => "latest".to_string(),
    };

    // clean up
    println!("\n");
    println!("=== Cleaning up Geist ===");
    println!("Pruning old unused images...");
    Command::new("bash")
        .arg("-c")
        .arg("docker system prune -f")
        .status()?;
    println!("Pruning complete.");

    // update geist
    println!("\n");
    println!("=== Updating Geist ===");
    println!("Updating to version: {}", ver);

    // Add logic to update to the specified version
    Command::new("bash")
        .arg("-c")
        .arg(format!("docker pull {}:{}", image_path, ver))
        .status()?;

    Ok(())
}

pub async fn status() -> Result<(), Box<dyn Error>> {
    println!("\n");
    println!("=== Geist Status ===");
    // Check if Geist is running
    let running_container_id = Command::new("docker")
        .args(["ps", "-q", "-f", &format!("name={}", CONTAINER_NAME)])
        .output()?;

    if running_container_id.stdout.is_empty() {
        println!("Geist is not running.");
    } else {
        println!("Geist is running.");

        // Show the currently running version
        let current_tag = Command::new("docker")
            .args(["inspect", "--format", "{{.Config.Image}}", CONTAINER_NAME])
            .output()?;

        if !current_tag.stdout.is_empty() {
            println!(
                "Current tag: {}",
                String::from_utf8_lossy(&current_tag.stdout).trim()
            );
        }
    }

    Ok(())
}

pub async fn exec() -> Result<(), Box<dyn Error>> {
    // Check if Geist is running
    let running_container_id = Command::new("docker")
        .args(["ps", "-q", "-f", &format!("name={}", CONTAINER_NAME)])
        .output()?;

    if running_container_id.stdout.is_empty() {
        println!("Geist is not currently running. Do you want to start it? [y/N]");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;

        if user_input.trim().eq_ignore_ascii_case("y") {
            start(None).await?;
        } else {
            println!("Operation canceled.");
            return Ok(());
        }
    }

    println!("Connecting to Geist. Press Ctrl+D to exit.");
    let exec_status = Command::new("docker")
        .args(["exec", "-ti", CONTAINER_NAME, "/bin/bash"])
        .status()?;

    if !exec_status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to execute an interactive shell",
        )));
    }

    Ok(())
}
