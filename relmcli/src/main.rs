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
        Commands::Create { path } => create(path),
        Commands::Delete { name } => delete(name),
        Commands::Start { name } => start(name),
        Commands::Stop { name } => stop(name),
        Commands::Switch { name } => switch(name),
        Commands::List => list(),
    }
}

fn create(path: String) {
    println!("Creating new realm from definition in {}...", path);
}

fn delete(name: String) {
    println!("Deleting {}...", name);
}

fn start(name: String) {
    println!("Starting {}...", name);
}

fn stop(name: String) {
    println!("Stopping {}...", name);
}

fn switch(name: String) {
    println!("Stwitching to {}...", name);
}

fn list() {
    println!("Realms:");
}


