# Aliaser Quick Start Guide

## Installation

After building, the binary is located at: `target/release/aliaser`

You can either:
1. Run it directly: `./target/release/aliaser`
2. Add to PATH: `sudo cp target/release/aliaser /usr/local/bin/`
3. Create an alias: `alias aliaser='./target/release/aliaser'`

## Quick Start

### 1. Initialize Your Vault

```bash
aliaser init
```

You'll be prompted to create a master password. This password:
- Must be at least 8 characters (16+ recommended)
- Cannot be recovered if lost
- Encrypts all your data

### 2. Add Your First Identity

```bash
aliaser add
```

Example interaction:
```
Service name: GitHub
Username: john_doe
Password: [leave empty to auto-generate]
Email (optional): john@example.com
Alias (optional): johndoe123
Add personal information? n
Notes (optional): Personal GitHub account
```

### 3. View Your Identities

List all services:
```bash
aliaser list
```

Get specific identity:
```bash
aliaser get GitHub
```

## Common Usage Examples

### Adding Identity with Personal Info

```bash
aliaser add

Service name: Amazon
Username: johndoe@email.com
Password: [leave empty to generate]
Email: johndoe@email.com
Alias: john_doe
Add personal information? y
  First Name: John
  Last Name: Doe
  Birthdate: 1990-05-15
  Address: 123 Main St, City, State 12345
  Phone: 555-0123
  Add custom fields? y
    Field name: Security Question
    Field value: Mother's maiden name: Smith
    Add another field? n
Notes: Main shopping account
```

### Update Existing Identity

```bash
aliaser update GitHub

# Only update what you want - press Enter to skip
Username: 
Update password? y
  New password: [leave empty to generate]
Email: newemail@example.com
...
```

### Delete Identity

```bash
aliaser delete OldService
```

### Backup Your Vault

```bash
# Create encrypted backup
aliaser export ~/backups/vault-2026-01-01.vault

# Restore from backup
aliaser import ~/backups/vault-2026-01-01.vault
```

### Change Master Password

```bash
aliaser change-master
```

## Tips & Best Practices

### 1. Strong Master Password
- Use a passphrase: "correct horse battery staple"
- Mix words, numbers, symbols: "MyC@t2024!Loves$Tuna"
- Minimum 16 characters for maximum security

### 2. Regular Backups
Create a backup script:
```bash
#!/bin/bash
DATE=$(date +%Y-%m-%d)
aliaser export ~/backups/vault-$DATE.vault
```

### 3. Password Generation
When prompted for password, press Enter to auto-generate a 20-character secure password.

### 4. Organizing Identities
Use descriptive service names:
- ✅ "GitHub-Personal"
- ✅ "Gmail-Work"
- ✅ "BankOfAmerica-Checking"
- ❌ "github"
- ❌ "email"

### 5. Using Notes Field
Store additional context:
- Account recovery codes
- Security question answers
- Account creation date
- Special features/permissions

### 6. Custom Fields
Use custom fields for:
- Security questions
- PIN codes
- Account numbers
- API keys
- Recovery emails

## Security Reminders

✓ Your master password is NEVER stored  
✓ All data encrypted with AES-256-GCM  
✓ Encryption keys derived with Argon2id  
✓ Data stored only on your local machine  
✓ No network requests - completely offline  

⚠️ If you forget master password, data CANNOT be recovered  
⚠️ Keep backups in a secure location  
⚠️ Use full disk encryption on your computer  
⚠️ Lock your screen when away from computer  

## Troubleshooting

### "Vault not initialized"
Run `aliaser init` first.

### "Invalid master password"
Make sure you're typing the correct master password. There's no "reset" option for security reasons.

### "Identity already exists"
Use `aliaser update <service>` to modify existing identities.

### Lost Master Password
Unfortunately, there's no recovery. You'll need to:
1. Delete `~/.aliaser.vault` and `~/.aliaser.config`
2. Run `aliaser init` to start fresh
3. Restore from an unencrypted backup if available

## Advanced Usage

### Scripting
You can automate exports:
```bash
# Daily backup cron job
0 2 * * * /usr/local/bin/aliaser export ~/backups/vault-$(date +\%Y\%m\%d).vault
```

### Multiple Vaults
To use multiple vaults, you can manually specify config locations (requires modification) or:
```bash
# Save current vault
mv ~/.aliaser.vault ~/.aliaser-personal.vault
mv ~/.aliaser.config ~/.aliaser-personal.config

# Initialize new vault
aliaser init

# Switch between vaults
mv ~/.aliaser.vault ~/.aliaser-work.vault
mv ~/.aliaser-personal.vault ~/.aliaser.vault
mv ~/.aliaser-personal.config ~/.aliaser.config
```

## File Locations

- Vault data: `~/.aliaser.vault` (encrypted)
- Configuration: `~/.aliaser.config` (contains hash and salt only)

Both files are required for the vault to function.

## Getting Help

Run any command without arguments to see usage:
```bash
aliaser --help
aliaser get --help
```

## Example Workflow

```bash
# Day 1: Setup
aliaser init
aliaser add  # Add GitHub
aliaser add  # Add Gmail
aliaser add  # Add Bank

# View your data
aliaser list
aliaser get GitHub

# Day 30: Update
aliaser update Bank  # Changed password
aliaser export ~/vault-backup.vault

# Day 60: Cleanup
aliaser delete OldAccount

# Day 90: Security
aliaser change-master  # New master password
```

---

**Remember**: Your security is your responsibility. Use strong passwords, keep backups, and never share your master password! 🔐
