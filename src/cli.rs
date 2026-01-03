use crate::identity::{Credentials, Identity, PersonalInfo};
use crate::storage::Vault;
use crate::yubikey::YubiKeyAuth;
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use rpassword::read_password;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aliaser")]
#[command(about = "A secure, local identity and password manager", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new vault with a master password
    Init,
    /// Add a new identity
    Add,
    /// List all stored services
    List,
    /// Get an identity for a service
    Get {
        /// Service name to retrieve
        service: String,
    },
    /// Update an existing identity
    Update {
        /// Service name to update
        service: String,
    },
    /// Delete an identity
    Delete {
        /// Service name to delete
        service: String,
    },
    /// Export vault to a file (encrypted backup)
    Export {
        /// Path to export file
        path: PathBuf,
    },
    /// Import vault from a file
    Import {
        /// Path to import file
        path: PathBuf,
    },
    /// Change master password
    ChangeMaster,
}

pub fn init() -> Result<()> {
    let mut vault = Vault::new()?;

    if vault.is_initialized() {
        println!("{}", "Vault already initialized!".yellow());
        return Ok(());
    }

    println!("{}", "Initializing new vault...".cyan().bold());
    println!();

    // ask about YubiKey
    let use_yubikey = if YubiKeyAuth::is_available() {
        println!("{}", "YubiKey detected!".green());
        prompt_yes_no("Enable YubiKey authentication? (y/n): ")?
    } else {
        println!("{}", "No YubiKey detected (optional)".dimmed());
        false
    };

    let master_password = prompt_new_password("Enter master password: ")?;

    if use_yubikey {
        println!();
        println!("{}", "Please touch your YubiKey...".cyan());
    }

    vault.initialize(&master_password, use_yubikey)?;

    println!();
    println!("{}", "✓ Vault initialized successfully!".green().bold());
    
    if use_yubikey {
        println!(
            "{}",
            "⚠ YubiKey required: Keep your YubiKey safe!".yellow()
        );
        println!(
            "{}",
            "  You'll need both your password AND YubiKey to unlock.".yellow()
        );
    }
    println!(
        "{}",
        "⚠ Remember your master password - it cannot be recovered!".yellow()
    );

    Ok(())
}

pub fn add_identity() -> Result<()> {
    let mut vault = Vault::new()?;
    unlock_vault(&mut vault)?;

    println!("{}", "Add New Identity".cyan().bold());
    println!();

    // Service name
    let service = prompt("Service name: ")?;

    // Credentials
    println!("{}", "Credentials:".bold());
    let username = prompt("  Username: ")?;
    let password = prompt_password("  Password (leave empty to generate): ")?;
    let password = if password.is_empty() {
        generate_password()
    } else {
        password
    };

    let email = prompt_optional("  Email (optional): ")?;
    let alias = prompt_optional("  Alias (optional): ")?;

    let credentials = Credentials {
        username,
        password: password.clone(),
        email,
        alias,
    };

    // Personal info
    println!();
    let add_personal = prompt_yes_no("Add personal information? (y/n): ")?;

    let personal_info = if add_personal {
        Some(collect_personal_info()?)
    } else {
        None
    };

    // Notes
    println!();
    let notes = prompt_optional("Notes (optional): ")?;

    // Create identity
    let mut identity = Identity::new(service.clone(), credentials);
    identity.personal_info = personal_info;
    identity.notes = notes;

    // Save
    vault.add_identity(identity)?;

    println!();
    println!("{}", "✓ Identity added successfully!".green().bold());
    if !password.is_empty() {
        println!("Generated password: {}", password.bright_yellow());
    }

    Ok(())
}

pub fn list_identities() -> Result<()> {
    let mut vault = Vault::new()?;
    unlock_vault(&mut vault)?;

    let services = vault.list_services()?;

    if services.is_empty() {
        println!("{}", "No identities stored yet.".yellow());
        return Ok(());
    }

    println!("{}", "Stored Identities:".cyan().bold());
    println!();

    for (i, service) in services.iter().enumerate() {
        println!("  {}. {}", i + 1, service.bright_white());
    }

    println!();
    println!("Total: {}", services.len().to_string().green());

    Ok(())
}

pub fn get_identity(service: &str) -> Result<()> {
    let mut vault = Vault::new()?;
    unlock_vault(&mut vault)?;

    let identity = vault.get_identity(service)?;

    println!();
    println!("{}", format!("Identity: {}", service).cyan().bold());
    println!("{}", "=".repeat(50).dimmed());
    println!();

    // Credentials
    println!("{}", "Credentials:".bold());
    println!("  Username: {}", identity.credentials.username.bright_white());
    println!("  Password: {}", identity.credentials.password.bright_yellow());
    if let Some(email) = &identity.credentials.email {
        println!("  Email: {}", email.bright_white());
    }
    if let Some(alias) = &identity.credentials.alias {
        println!("  Alias: {}", alias.bright_white());
    }

    // Personal info
    if let Some(info) = &identity.personal_info {
        println!();
        println!("{}", "Personal Information:".bold());
        if let Some(first) = &info.first_name {
            println!("  First Name: {}", first.bright_white());
        }
        if let Some(last) = &info.last_name {
            println!("  Last Name: {}", last.bright_white());
        }
        if let Some(birth) = &info.birthdate {
            println!("  Birthdate: {}", birth.bright_white());
        }
        if let Some(addr) = &info.address {
            println!("  Address: {}", addr.bright_white());
        }
        if let Some(phone) = &info.phone {
            println!("  Phone: {}", phone.bright_white());
        }

        if !info.custom_fields.is_empty() {
            println!();
            println!("  Custom Fields:");
            for field in &info.custom_fields {
                println!("    {}: {}", field.key, field.value.bright_white());
            }
        }
    }

    // Notes
    if let Some(notes) = &identity.notes {
        println!();
        println!("{}", "Notes:".bold());
        println!("  {}", notes.bright_white());
    }

    // Metadata
    println!();
    println!("{}", "Metadata:".dimmed());
    println!("  Created: {}", identity.created_at.format("%Y-%m-%d %H:%M:%S").to_string().dimmed());
    println!("  Updated: {}", identity.updated_at.format("%Y-%m-%d %H:%M:%S").to_string().dimmed());

    Ok(())
}

pub fn update_identity(service: &str) -> Result<()> {
    let mut vault = Vault::new()?;
    unlock_vault(&mut vault)?;

    let mut identity = vault.get_identity(service)?;

    println!("{}", format!("Update Identity: {}", service).cyan().bold());
    println!("{}", "(Press Enter to keep current value)".dimmed());
    println!();

    // Update credentials
    println!("{}", "Credentials:".bold());
    
    let new_username = prompt_optional("  Username: ")?;
    if let Some(username) = new_username {
        identity.credentials.username = username;
    }

    if prompt_yes_no("  Update password? (y/n): ")? {
        let new_password = prompt_password("  New password (leave empty to generate): ")?;
        identity.credentials.password = if new_password.is_empty() {
            generate_password()
        } else {
            new_password
        };
    }

    let new_email = prompt_optional("  Email: ")?;
    if new_email.is_some() {
        identity.credentials.email = new_email;
    }

    let new_alias = prompt_optional("  Alias: ")?;
    if new_alias.is_some() {
        identity.credentials.alias = new_alias;
    }

    // Update personal info
    if prompt_yes_no("\nUpdate personal information? (y/n): ")? {
        identity.personal_info = Some(collect_personal_info()?);
    }

    // Update notes
    let new_notes = prompt_optional("\nNotes: ")?;
    if new_notes.is_some() {
        identity.notes = new_notes;
    }

    vault.update_identity(service, identity)?;

    println!();
    println!("{}", "✓ Identity updated successfully!".green().bold());

    Ok(())
}

pub fn delete_identity(service: &str) -> Result<()> {
    let mut vault = Vault::new()?;
    unlock_vault(&mut vault)?;

    println!(
        "{}",
        format!("Delete identity for '{}'?", service).yellow().bold()
    );
    println!("{}", "This action cannot be undone!".red());

    if !prompt_yes_no("\nConfirm deletion (y/n): ")? {
        println!("Cancelled.");
        return Ok(());
    }

    vault.delete_identity(service)?;

    println!();
    println!("{}", "✓ Identity deleted successfully.".green().bold());

    Ok(())
}

pub fn export_data(path: &PathBuf) -> Result<()> {
    let mut vault = Vault::new()?;
    unlock_vault(&mut vault)?;

    vault.export(path)?;

    println!();
    println!(
        "{}",
        format!("✓ Vault exported to: {}", path.display()).green().bold()
    );
    println!(
        "{}",
        "The exported file is encrypted with your master password.".dimmed()
    );

    Ok(())
}

pub fn import_data(path: &PathBuf) -> Result<()> {
    let mut vault = Vault::new()?;
    unlock_vault(&mut vault)?;

    println!(
        "{}",
        "This will overwrite your current vault!".yellow().bold()
    );
    if !prompt_yes_no("Continue? (y/n): ")? {
        println!("Cancelled.");
        return Ok(());
    }

    vault.import(path)?;

    println!();
    println!("{}", "✓ Vault imported successfully!".green().bold());

    Ok(())
}

pub fn change_master_password() -> Result<()> {
    let mut vault = Vault::new()?;

    println!("{}", "Change Master Password".cyan().bold());
    println!();

    print!("Current master password: ");
    io::stdout().flush()?;
    let old_password = read_password()?;

    println!();
    let new_password = prompt_new_password("New master password: ")?;

    vault.change_master_password(&old_password, &new_password)?;

    println!();
    println!("{}", "✓ Master password changed successfully!".green().bold());

    Ok(())
}

// Helper functions

fn unlock_vault(vault: &mut Vault) -> Result<()> {
    if !vault.is_initialized() {
        anyhow::bail!("Vault not initialized. Run 'aliaser init' first.");
    }

    print!("Master password: ");
    io::stdout().flush()?;
    let password = read_password()?;
    println!();

    vault.unlock(&password)?;

    Ok(())
}

fn prompt(message: &str) -> Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_optional(message: &str) -> Result<Option<String>> {
    let input = prompt(message)?;
    if input.is_empty() {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

fn prompt_password(message: &str) -> Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let password = read_password()?;
    Ok(password)
}

fn prompt_new_password(message: &str) -> Result<String> {
    loop {
        print!("{}", message);
        io::stdout().flush()?;
        let password = read_password()?;

        if password.len() < 8 {
            println!("{}", "Password must be at least 8 characters!".red());
            continue;
        }

        print!("Confirm password: ");
        io::stdout().flush()?;
        let confirm = read_password()?;

        if password != confirm {
            println!("{}", "Passwords don't match!".red());
            continue;
        }

        return Ok(password);
    }
}

fn prompt_yes_no(message: &str) -> Result<bool> {
    loop {
        let input = prompt(message)?;
        match input.to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}

fn collect_personal_info() -> Result<PersonalInfo> {
    println!("{}", "Personal Information:".bold());

    let first_name = prompt_optional("  First Name: ")?;
    let last_name = prompt_optional("  Last Name: ")?;
    let birthdate = prompt_optional("  Birthdate (YYYY-MM-DD): ")?;
    let address = prompt_optional("  Address: ")?;
    let phone = prompt_optional("  Phone: ")?;

    let mut info = PersonalInfo {
        first_name,
        last_name,
        birthdate,
        address,
        phone,
        custom_fields: Vec::new(),
    };

    // Custom fields
    if prompt_yes_no("\nAdd custom fields? (y/n): ")? {
        loop {
            let key = prompt("  Field name: ")?;
            if key.is_empty() {
                break;
            }
            let value = prompt("  Field value: ")?;
            info.add_custom_field(key, value);

            if !prompt_yes_no("  Add another field? (y/n): ")? {
                break;
            }
        }
    }

    Ok(info)
}

fn generate_password() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*()_+-=[]{}|;:,.<>?";
    const PASSWORD_LEN: usize = 20;
    
    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    password
}
