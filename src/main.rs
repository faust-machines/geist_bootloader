use clap::Parser;
use geist_bootloader;

#[derive(Parser)]
enum Opts {
    Build,
    Logs,
    Start,
    Stop,
    Version,
    Update,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    match opts {
        Opts::Build => geist_bootloader::build().await?,
        Opts::Logs => geist_bootloader::logs().await?,
        Opts::Start => geist_bootloader::start().await?,
        Opts::Stop => geist_bootloader::stop().await?,
        Opts::Version => geist_bootloader::version().await?,
        Opts::Update => geist_bootloader::update().await?,
    }
    Ok(())
}