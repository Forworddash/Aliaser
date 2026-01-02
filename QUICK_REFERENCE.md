# Aliaser Quick Reference Card

## 📋 Command Cheat Sheet

### First Time Setup
```bash
aliaser init                    # Initialize vault with master password
```

### Daily Operations
```bash
aliaser add                     # Add new identity (interactive)
aliaser list                    # Show all stored services
aliaser get <service>           # Display identity details
```

### Management
```bash
aliaser update <service>        # Update existing identity
aliaser delete <service>        # Remove identity (with confirmation)
```

### Backup & Recovery
```bash
aliaser export backup.vault     # Create encrypted backup
aliaser import backup.vault     # Restore from backup
```

### Security
```bash
aliaser change-master           # Change master password
```

---

## 🔑 Keyboard Shortcuts

### During Password Input
- **Enter** (empty) → Auto-generate strong password
- **Type password** → Use your own password

### During Prompts
- **Enter** (empty) → Skip optional field
- **y/yes** → Confirm action
- **n/no** → Cancel action

---

## 📁 File Locations

```
~/.aliaser.vault    # Encrypted data (KEEP SAFE!)
~/.aliaser.config   # Config (hash + salt)
```

---

## 🔐 Security Quick Facts

| Feature | Value |
|---------|-------|
| Encryption | AES-256-GCM |
| Key Derivation | Argon2id |
| Key Size | 256 bits |
| Salt Size | 256 bits |
| Min Password | 8 characters |
| Network Code | ZERO |
| Data Storage | Local only |

---

## ⚡ Quick Examples

### Add GitHub Account
```bash
aliaser add
Service name: GitHub
Username: myusername
Password: [Enter for auto-generate]
Email: me@email.com
Alias: myalias
Personal info? n
Notes: My GitHub account
```

### View Credentials
```bash
aliaser get GitHub
# Shows username, password, email, etc.
```

### Create Daily Backup
```bash
aliaser export ~/backups/vault-$(date +%Y-%m-%d).vault
```

### Update Password
```bash
aliaser update GitHub
# Follow prompts, press Enter to skip unchanged fields
```

---

## ⚠️ Important Warnings

❌ **Master password CANNOT be recovered**  
❌ **Both files needed (.vault + .config)**  
❌ **No sync - backup manually**  
✅ **Keep backups in secure location**  
✅ **Use strong master password (16+ chars)**

---

## 🆘 Emergency Procedures

### Lost Master Password
```bash
# NO RECOVERY POSSIBLE
# Must delete vault and start over:
rm ~/.aliaser.vault ~/.aliaser.config
aliaser init
# Restore from unencrypted backup if available
```

### Corrupted Vault
```bash
# Try importing from backup
aliaser import ~/backups/vault-latest.vault

# If all backups corrupted, vault is lost
# This is why regular backups are critical!
```

### Forgot Which Services Stored
```bash
# List shows all service names (requires master password)
aliaser list
```

---

## 🎯 Best Practices

### Master Password
- ✅ Use 16+ characters
- ✅ Mix uppercase, lowercase, numbers, symbols
- ✅ Use passphrase: "correct-horse-battery-staple"
- ❌ Don't use: dictionary words, personal info, common patterns

### Backups
```bash
# Weekly backup script
#!/bin/bash
aliaser export ~/backups/vault-$(date +%Y-%m-%d).vault

# Keep last 4 backups
cd ~/backups
ls -t vault-*.vault | tail -n +5 | xargs rm -f
```

### Service Names
- ✅ Descriptive: "GitHub-Personal", "Gmail-Work"
- ✅ Consistent: Same naming pattern
- ❌ Avoid: "account1", "website", "login"

### Password Generation
- Let Aliaser generate passwords (20 chars, random)
- Copy immediately after generation
- Update in service right away

---

## 🔧 Troubleshooting

| Problem | Solution |
|---------|----------|
| "Vault not initialized" | Run `aliaser init` first |
| "Invalid master password" | Check password, no recovery if lost |
| "Identity already exists" | Use `update` instead of `add` |
| "Identity not found" | Check service name with `list` |
| Can't remember service name | Run `aliaser list` |

---

## 📞 Getting Help

```bash
aliaser --help              # General help
aliaser <command> --help    # Command-specific help
```

**Documentation:**
- README.md - Full documentation
- QUICKSTART.md - Detailed tutorial
- SECURITY.md - Security details

---

## 🎓 Learning Path

1. **Read:** README.md (10 mins)
2. **Initialize:** `aliaser init` (1 min)
3. **Practice:** Add 2-3 test identities (5 mins)
4. **Explore:** Try all commands (10 mins)
5. **Secure:** Create backup strategy (5 mins)

**Total time to proficiency: ~30 minutes**

---

## 💡 Pro Tips

1. **Backup Before Updating**
   ```bash
   aliaser export backup-before-update.vault
   aliaser update ServiceName
   ```

2. **Use Descriptive Notes**
   ```
   Notes: Created 2026-01-01, uses email login, has 2FA enabled
   ```

3. **Custom Fields for Recovery Codes**
   ```
   Add custom fields? y
   Field name: 2FA_Recovery_Code
   Field value: ABCD-1234-EFGH-5678
   ```

4. **Regular Maintenance**
   - Monthly: Review stored identities
   - Quarterly: Update passwords
   - Yearly: Change master password

5. **Test Backups**
   ```bash
   # Create test vault
   mv ~/.aliaser.vault ~/.aliaser.vault.test
   mv ~/.aliaser.config ~/.aliaser.config.test
   
   # Test restore
   aliaser import backup.vault
   aliaser list
   
   # Restore original
   mv ~/.aliaser.vault.test ~/.aliaser.vault
   mv ~/.aliaser.config.test ~/.aliaser.config
   ```

---

## 📊 Comparison Quick Reference

| Feature | Aliaser | Cloud Manager | Browser |
|---------|---------|---------------|---------|
| **Offline** | ✅ | ❌ | ⚠️ |
| **Open Source** | ✅ | ⚠️ | ⚠️ |
| **CLI** | ✅ | ❌ | ❌ |
| **GUI** | ❌ | ✅ | ✅ |
| **Sync** | ❌ | ✅ | ✅ |
| **Mobile** | ❌ | ✅ | ⚠️ |
| **Free** | ✅ | ⚠️ | ✅ |
| **Privacy** | ✅✅✅ | ⚠️ | ❌ |

---

## 🎯 One-Liner Commands

```bash
# Quick backup
aliaser export ~/vault-backup-$(date +%Y%m%d).vault

# List with count
aliaser list | tail -1

# Get username only (requires jq and manual parsing)
# Note: Aliaser outputs formatted text, not JSON

# Check if vault exists
[ -f ~/.aliaser.vault ] && echo "Vault exists" || echo "No vault"

# Backup with timestamp
aliaser export ~/backups/vault-$(date +%Y%m%d-%H%M%S).vault
```

---

**Print this page for quick reference!** 📄

*Keep your master password safe - it's your only key!* 🔑
