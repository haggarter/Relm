use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { path: String },
    Delete { name: String },
    Start { name: String },
    Stop { name: String },
    Switch { name: String },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { path } => println!("create {}", path),
        Commands::Delete { name } => println!("delete {}", name),
        Commands::Start { name } => println!("start {}", name),
        Commands::Stop { name } => println!("stop {}", name),
        Commands::Switch { name } => println!("switch {}", name),
        Commands::List => println!("list"),
    }
}