mod cli;
mod crypto;
mod identity;
mod storage;
mod yubikey;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init => {
            cli::init()?;
        }
        Commands::Add => {
            cli::add_identity()?;
        }
        Commands::List => {
            cli::list_identities()?;
        }
        Commands::Get { service } => {
            cli::get_identity(&service)?;
        }
        Commands::Update { service } => {
            cli::update_identity(&service)?;
        }
        Commands::Delete { service } => {
            cli::delete_identity(&service)?;
        }
        Commands::Export { path } => {
            cli::export_data(&path)?;
        }
        Commands::Import { path } => {
            cli::import_data(&path)?;
        }
        Commands::ChangeMaster => {
            cli::change_master_password()?;
        }
    }
    
    Ok(())
}
