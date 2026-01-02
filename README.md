# Aliaser - Secure Local Identity Manager

> 📑 **New here?** See [INDEX.md](INDEX.md) for a complete documentation guide.

A secure, open-source identity and password manager built in Rust. All data is encrypted with AES-256-GCM and stored locally on your machine. No data is ever sent to any server.

## Features

- 🔒 **Military-grade encryption**: AES-256-GCM with Argon2id key derivation
- 🏠 **Completely local**: All data stored encrypted on your machine
- 🔑 **Password management**: Store passwords, usernames, emails, and aliases
- 👤 **Identity management**: Store personal information (birthdate, address, phone, etc.)
- 📝 **Custom fields**: Add custom key-value pairs to any identity
- 🎲 **Password generation**: Automatic strong password generation
- 💾 **Backup & restore**: Export/import encrypted vault backups
- 🧹 **Memory safety**: Sensitive data is zeroized after use
- 🎨 **Beautiful CLI**: Color-coded, user-friendly terminal interface

## Security

### Encryption
- **Algorithm**: AES-256-GCM (Authenticated Encryption with Additional Data)
- **Key Derivation**: Argon2id with random salt
- **Random Nonces**: Each encryption operation uses a unique random nonce
- **Password Hashing**: Argon2id for master password verification

### Data Protection
- Master password never stored (only hash)
- Encryption keys derived on-the-fly from master password
- Sensitive data structures implement zeroization
- All vault data encrypted at rest

### No Telemetry
- Zero network requests
- No analytics or tracking
- No user data collection
- Fully open source for audit

## Installation

### Prerequisites
- Rust 1.70 or higher

### Build from source

```bash
# Clone the repository
git clone <your-repo-url>
cd Aliaser

# Build release binary
cargo build --release

# Binary will be in target/release/aliaser
# Optionally, install to your system
cargo install --path .
```

## Usage

### Initialize Vault

First, initialize a new vault with a master password:

```bash
aliaser init
```

⚠️ **Important**: Your master password cannot be recovered. Keep it safe!

### Add an Identity

Add a new identity with credentials:

```bash
aliaser add
```

You'll be prompted for:
- Service name (e.g., "GitHub", "Gmail")
- Username
- Password (or auto-generate)
- Email (optional)
- Alias (optional)
- Personal information (optional)
- Notes (optional)

### List All Identities

```bash
aliaser list
```

### View an Identity

```bash
aliaser get <service>
```

Example:
```bash
aliaser get GitHub
```

### Update an Identity

```bash
aliaser update <service>
```

### Delete an Identity

```bash
aliaser delete <service>
```

### Export Vault (Backup)

Export your encrypted vault to a file:

```bash
aliaser export backup.vault
```

The exported file is encrypted with your master password.

### Import Vault (Restore)

Restore from a backup:

```bash
aliaser import backup.vault
```

⚠️ **Warning**: This will overwrite your current vault!

### Change Master Password

```bash
aliaser change-master
```

## Data Storage

All data is stored in your home directory:

- `~/.aliaser.config` - Vault configuration (password hash and salt)
- `~/.aliaser.vault` - Encrypted vault data

Both files are required for the vault to function. Keep backups safe!

## Commands Reference

| Command | Description |
|---------|-------------|
| `init` | Initialize a new vault |
| `add` | Add a new identity |
| `list` | List all stored services |
| `get <service>` | Retrieve an identity |
| `update <service>` | Update an existing identity |
| `delete <service>` | Delete an identity |
| `export <path>` | Export encrypted vault to file |
| `import <path>` | Import vault from file |
| `change-master` | Change master password |

## Example Workflow

```bash
# 1. Initialize vault
aliaser init

# 2. Add some identities
aliaser add  # Add GitHub account
aliaser add  # Add email account

# 3. List all services
aliaser list

# 4. Get credentials for a service
aliaser get GitHub

# 5. Create a backup
aliaser export ~/backups/aliaser-backup-2026-01-01.vault

# 6. Update password for a service
aliaser update GitHub
```

## Security Best Practices

1. **Master Password**: Use a strong, unique master password (minimum 8 characters)
2. **Backups**: Regularly export your vault to a secure backup location
3. **Physical Security**: Keep your computer and backups physically secure
4. **Updates**: Keep Aliaser updated for security patches
5. **Audit**: Review the source code - it's open source for your security

## Building for Production

For maximum security and performance:

```bash
cargo build --release
strip target/release/aliaser  # Remove debug symbols
```

The release build includes:
- Link-time optimization (LTO)
- Maximum optimization level
- Stripped binaries
- Single codegen unit for better optimization

## Technical Details

### Dependencies

Core security:
- `aes-gcm` - AES-256-GCM encryption
- `argon2` - Key derivation and password hashing
- `rand` - Cryptographically secure random number generation
- `zeroize` - Secure memory clearing

CLI & utilities:
- `clap` - Command-line argument parsing
- `rpassword` - Secure password input
- `colored` - Terminal colors
- `serde` - Serialization
- `chrono` - Date/time handling

### Architecture

```
┌─────────────┐
│     CLI     │  Command-line interface
└──────┬──────┘
       │
┌──────▼──────┐
│  Identity   │  Data structures
└──────┬──────┘
       │
┌──────▼──────┐
│   Storage   │  Vault management
└──────┬──────┘
       │
┌──────▼──────┐
│   Crypto    │  Encryption/decryption
└─────────────┘
```

## Contributing

This is an open-source project. Contributions are welcome!

### Areas for improvement:
- GUI frontend (using egui or iced)
- Browser integration
- Additional export formats
- Password strength checker
- Two-factor authentication storage

## License

GPL License - See LICENSE file

## Disclaimer

This software is provided "as is" without warranty. Always maintain backups of your data. The authors are not responsible for any data loss.

## FAQ

**Q: Where is my data stored?**  
A: In your home directory: `~/.aliaser.vault` and `~/.aliaser.config`

**Q: Can I sync across devices?**  
A: Not built-in, but you can manually export/import vault files. Consider using encrypted cloud storage for backups.

**Q: What if I forget my master password?**  
A: Unfortunately, it cannot be recovered. This is by design for security. Keep backups and remember your password!

**Q: Is this audited?**  
A: This is open-source software. You can audit the code yourself or hire a professional auditor.

**Q: Can I use this for my team?**  
A: This is designed for individual use. For team password management, consider enterprise solutions.

## Support

For issues, questions, or contributions, please open an issue on GitHub.

---

**Remember**: Your security is in your hands. Use strong passwords, keep backups, and stay vigilant! 🔐
