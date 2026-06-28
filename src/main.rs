use clap::{Parser, Subcommand};

mod cmd;
mod models;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Init sample site project
    Init {},
    /// Create new post or page
    Create(cmd::CreateArgs),
    /// Build site
    Build(cmd::BuildArgs),
    /// Serve site and watch for changes
    Serve(cmd::ServerArgs),
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::max())
        .format_module_path(false)
        .init();

    let args = Cli::parse();
    match args.command {
        Commands::Init {} => {
            cmd::run_init();
        }
        Commands::Create(args) => {
            cmd::run_create(args);
        }
        Commands::Build(args) => {
            cmd::run_build(args);
        }
        Commands::Serve(args) => {
            cmd::run_serve(args);
        }
    }
}
