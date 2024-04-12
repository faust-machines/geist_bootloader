use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(version = "1.0", about = "Geist Command Line Interface")]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tails the logs from the geist
    Logs,
    Service(ServiceCommand),
    /// Checks the status of the Geist service
    Status,
    Start(StartOpts),
    /// Stops geist
    Stop,
    Topic(TopicCommand),
    /// Prints the version of geist
    Version,
    Update(UpdateOpts),
    /// Opens an interactive shell in the Geist container
    Exec,
}

#[derive(Args)]
/// Starts geist
struct StartOpts {
    #[clap(short, long)]
    version: Option<String>,
    /// Optional path to the .env file
    #[clap(short, long)]
    env_file: Option<String>,
}

#[derive(Args)]
/// Updates the geist to a specific version
struct UpdateOpts {
    #[clap(short, long)]
    version: Option<String>,
}

// Wrapper structs
#[derive(Args)]
struct ServiceCommand {
    #[clap(subcommand)]
    pub opt: ServiceOpts,
}

#[derive(Subcommand)]
/// Commands for interacting with Geist services
enum ServiceOpts {
    /// List all services
    List,
}

#[derive(Args)]
struct TopicCommand {
    #[clap(subcommand)]
    pub opt: TopicOpts,
}

#[derive(Subcommand)]
/// Commands for interacting with Geist topics
enum TopicOpts {
    /// List all topics
    List,
    /// Get the type of a topic
    Type { name: String },
    /// Echo a topic
    Echo { name: String },
}

#[derive(Args)]
struct VersionOpts {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Logs => geist_bootloader::logs().await?,
        Commands::Service(service_command) => match &service_command.opt {
            ServiceOpts::List => geist_bootloader::list_services().await?,
        },
        Commands::Start(start_opts) => {
            geist_bootloader::start(start_opts.version.clone(), start_opts.env_file.clone()).await?
        }
        Commands::Stop => geist_bootloader::stop().await?,
        Commands::Topic(topic_command) => match &topic_command.opt {
            TopicOpts::List => geist_bootloader::list_topics().await?,
            TopicOpts::Type { name } => geist_bootloader::get_topic_type(name.clone()).await?,
            TopicOpts::Echo { name } => geist_bootloader::echo_topic(name.clone()).await?,
        },
        Commands::Version => geist_bootloader::version().await?,
        Commands::Update(update_opts) => {
            geist_bootloader::update(update_opts.version.clone()).await?
        }
        Commands::Status => {
            geist_bootloader::status().await?;
        }
        Commands::Exec => {
            geist_bootloader::exec().await?;
        }
    }
    Ok(())
}
