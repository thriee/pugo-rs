use clap::Args;

#[derive(Debug, Args, Clone, Copy)]
pub struct BuildArgs {
    /// Watching for changes
    #[arg(short = 'w', long)]
    pub watch: bool,
    /// not clap argument
    #[arg(skip)]
    pub watch_in_spawn: bool,
    /// Clean old files before current build
    #[arg(short = 'c', long)]
    pub clean: bool,
    /// Compress built files to one tar.gz
    #[arg(short = 'a', long)]
    pub archive: bool,
}

#[derive(Debug, Args)]
pub struct ServerArgs {
    /// Set http port
    #[arg(short = 'p', long, default_value = "19292")]
    pub port: u16,
    /// Clean old files before current build
    #[arg(short = 'c', long)]
    pub clean: bool,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    #[arg(value_parser)]
    pub path: String,
    #[arg(short = 'p', long)]
    pub page: bool,
}
