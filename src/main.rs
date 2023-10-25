use clap::Parser;
use geist_bootloader;

#[derive(Parser)]
struct StartOpts {
    #[clap(short, long)]
    version: Option<String>,
}

#[derive(Parser)]
struct UpdateOpts {
    #[clap(short, long)]
    version: Option<String>,
}

#[derive(Parser)]
enum Opts {
    Build,
    Logs,
    Start(StartOpts),
    Stop,
    Version,
    Update(UpdateOpts),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    match opts {
        Opts::Build => geist_bootloader::build().await?,
        Opts::Logs => geist_bootloader::logs().await?,
        Opts::Start(start_opts) => geist_bootloader::start(start_opts.version).await?,
        Opts::Stop => geist_bootloader::stop().await?,
        Opts::Version => geist_bootloader::version().await?,
        Opts::Update(update_opts) => geist_bootloader::update(update_opts.version).await?,
    }
    Ok(())
}