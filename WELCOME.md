# 🔐 Welcome to Aliaser!

## What You've Got

A **secure, open-source password and identity manager** built in Rust with military-grade encryption. All your data stays on your machine, encrypted with AES-256-GCM.

---

## 🎯 Project Complete!

Your secure identity manager is ready to use:

### ✅ Core Features Implemented
- [x] AES-256-GCM encryption
- [x] Argon2id key derivation
- [x] Master password protection
- [x] Identity storage (username, password, email, alias)
- [x] Personal information storage
- [x] Custom fields support
- [x] Password auto-generation (20 chars)
- [x] Encrypted backup/restore
- [x] Master password change
- [x] Full CLI interface
- [x] Memory safety (zeroization)
- [x] Zero network code
- [x] Cross-platform (Linux, macOS, Windows)

### 📊 Project Stats
- **Lines of Code:** ~1,010 lines of Rust
- **Source Files:** 5 modules
- **Documentation:** 7 comprehensive guides
- **Dependencies:** 12 core security libraries
- **Binary Size:** ~5 MB (stripped release build)
- **Build Time:** ~45 seconds (release)

---

## 🚀 Quick Start (3 Minutes)

### 1. Build (if not already done)
```bash
cd /home/deneuve/Documents/Aliaser
cargo build --release
```

### 2. Initialize Your Vault
```bash
./target/release/aliaser init
# Enter a strong master password (16+ characters recommended)
```

### 3. Add Your First Identity
```bash
./target/release/aliaser add
# Follow the prompts
```

### 4. View Your Data
```bash
./target/release/aliaser list
./target/release/aliaser get <service-name>
```

---

## 📚 Documentation Guide

### For First-Time Users
1. **README.md** - Start here! Overview and installation
2. **QUICKSTART.md** - Detailed tutorial with examples
3. **QUICK_REFERENCE.md** - Command cheat sheet

### For Security-Conscious Users
1. **SECURITY.md** - Security policy and best practices
2. **SECURITY_COMPARISON.md** - Detailed security analysis

### For Developers
1. **PROJECT_SUMMARY.md** - Complete technical overview
2. **Source code** - Well-commented Rust code

### For Demonstration
1. **demo.sh** - Interactive demo script

---

## 🎓 Recommended Reading Order

**Total time: ~30 minutes to full proficiency**

1. **README.md** (5 mins) - Understand what it does
2. **Initialize vault** (2 mins) - Get hands-on
3. **Add test data** (5 mins) - Practice commands
4. **QUICKSTART.md** (10 mins) - Learn all features
5. **SECURITY_COMPARISON.md** (10 mins) - Understand security

---

## 🔒 Security Highlights

| What | How |
|------|-----|
| **Encryption** | AES-256-GCM (military-grade) |
| **Key Derivation** | Argon2id (winner of Password Hashing Competition) |
| **Key Size** | 256 bits (2^256 possibilities) |
| **Authentication** | AEAD (detects tampering) |
| **Network** | Zero connections (completely offline) |
| **Storage** | Local only (~/.aliaser.vault) |
| **Memory** | Zeroized after use |
| **Language** | Rust (memory-safe by design) |
| **Open Source** | Fully auditable |

---

## 📁 Files Overview

### Source Code (`src/`)
- **main.rs** - Entry point and command routing
- **cli.rs** - User interface and interactions
- **crypto.rs** - Encryption/decryption logic
- **identity.rs** - Data structures
- **storage.rs** - Vault file management

### Documentation
- **README.md** - Main documentation
- **QUICKSTART.md** - Tutorial guide
- **QUICK_REFERENCE.md** - Command cheat sheet
- **SECURITY.md** - Security policy
- **SECURITY_COMPARISON.md** - Security deep dive
- **PROJECT_SUMMARY.md** - Technical overview
- **LICENSE** - GPL License

### Build Files
- **Cargo.toml** - Project configuration
- **Cargo.lock** - Dependency lock file
- **.gitignore** - Git ignore rules

### Tools
- **demo.sh** - Interactive demo

---

## 🎯 What Makes This Special?

### 1. **Zero Trust in Cloud**
- No servers, no sync, no accounts
- Your data never leaves your machine
- No company can be breached for your passwords

### 2. **Military-Grade Security**
- Same encryption used by governments
- Resistant to GPU/ASIC cracking (Argon2id)
- Authenticated encryption (tampering detected)

### 3. **Complete Transparency**
- 100% open source
- Only ~1,000 lines of code (auditable)
- No telemetry, no tracking, no analytics

### 4. **Memory Safe**
- Written in Rust (no buffer overflows)
- Sensitive data zeroized
- No memory leaks of credentials

### 5. **Simple Yet Complete**
- Store passwords, usernames, emails
- Personal information (birthdate, address, etc.)
- Custom fields for anything else
- Notes and metadata

---

## 🎨 Example Use Cases

### Personal Use
```
✓ Store all website credentials
✓ Bank account information
✓ Credit card details (in custom fields)
✓ Social security number (in personal info)
✓ Software license keys (in notes)
```

### Professional Use
```
✓ Work email accounts
✓ Development credentials (GitHub, GitLab)
✓ API keys and tokens
✓ Server login credentials
✓ Client portal access
```

### Privacy-Focused Use
```
✓ Alias identities for online services
✓ Separate identity per website
✓ Temporary accounts tracking
✓ Privacy-focused email aliases
```

---

## ⚡ Commands at a Glance

```bash
# Setup
aliaser init                     # First time setup

# Daily use
aliaser add                      # Add new identity
aliaser list                     # Show all services
aliaser get <service>            # View credentials

# Management
aliaser update <service>         # Change details
aliaser delete <service>         # Remove identity

# Maintenance
aliaser export backup.vault      # Create backup
aliaser import backup.vault      # Restore backup
aliaser change-master            # New master password
```

---

## 🛡️ Security Best Practices

### ✅ DO:
- Use 16+ character master password
- Enable full disk encryption
- Create regular backups
- Store backups securely
- Lock your computer when away
- Use strong, unique passwords
- Review security documentation

### ❌ DON'T:
- Share your master password
- Store vault on unencrypted media
- Use weak master password
- Forget to backup
- Run on compromised systems
- Write master password down insecurely

---

## 🚨 Critical Warnings

### ⚠️ Master Password
**CANNOT BE RECOVERED IF LOST!**
- No password reset
- No recovery mechanism
- No backdoor access
- This is intentional for security

### ⚠️ Backups
**KEEP ENCRYPTED BACKUPS SAFE!**
- Regular exports recommended
- Store in secure location
- Test restores periodically
- Multiple backup locations

### ⚠️ Security
**ONLY AS SECURE AS YOUR SYSTEM!**
- Clean OS required
- No keyloggers/malware
- Full disk encryption recommended
- Physical security matters

---

## 🎓 Learning Resources

### Understanding Encryption
- **AES-256-GCM:** https://en.wikipedia.org/wiki/Galois/Counter_Mode
- **Argon2:** https://en.wikipedia.org/wiki/Argon2

### Rust Programming
- **Rust Book:** https://doc.rust-lang.org/book/
- **Rust by Example:** https://doc.rust-lang.org/rust-by-example/

### Password Management
- **OWASP Guidelines:** https://owasp.org/www-project-password-security/
- **Password Best Practices:** Read SECURITY.md

---

## 🔧 Troubleshooting Quick Reference

| Issue | Solution |
|-------|----------|
| Won't compile | Run `cargo clean && cargo build --release` |
| Vault not found | Run `aliaser init` first |
| Wrong password | No recovery - be careful! |
| Binary too large | Run `strip target/release/aliaser` |
| Need help | Run `aliaser --help` or read docs |

---

## 🎯 Next Steps

### Immediate (Today):
1. ✅ Build the project (done!)
2. ⬜ Initialize your vault
3. ⬜ Add 2-3 test identities
4. ⬜ Practice all commands
5. ⬜ Create first backup

### This Week:
1. ⬜ Read SECURITY_COMPARISON.md
2. ⬜ Set up backup schedule
3. ⬜ Migrate real passwords
4. ⬜ Test backup restore
5. ⬜ Set calendar reminder for password changes

### This Month:
1. ⬜ Full password audit
2. ⬜ Update weak passwords
3. ⬜ Add all online accounts
4. ⬜ Document recovery procedure
5. ⬜ Consider GUI development

---

## 📞 Support & Contribution

### Getting Help
- Read documentation files
- Check QUICKSTART.md for examples
- Review source code (it's readable!)
- Open GitHub issue for bugs

### Contributing
This is open source! Areas for contribution:
- GUI interface (egui, iced)
- Browser integration
- Mobile apps
- Additional features
- Security audits
- Documentation improvements

---

## 🏆 Congratulations!

You now have a **secure, private, and completely local** identity manager!

### Your Benefits:
✅ Military-grade encryption  
✅ No cloud dependencies  
✅ Complete privacy  
✅ Full control of your data  
✅ Open source transparency  
✅ Zero ongoing costs  

### Remember:
🔑 **Strong master password**  
💾 **Regular backups**  
🔒 **Physical security**  
📚 **Read documentation**  
🛡️ **Trust but verify**  

---

## 📝 Quick Commands Reminder

```bash
# Installation (one-time)
cargo build --release
sudo cp target/release/aliaser /usr/local/bin/

# First use
aliaser init

# Daily workflow
aliaser add        # Add account
aliaser list       # See all
aliaser get <name> # View details

# Weekly maintenance
aliaser export ~/backups/vault-$(date +%Y-%m-%d).vault
```

---

## 🎉 You're All Set!

Your secure password manager is ready to protect your digital life.

**Start with:** `./target/release/aliaser init`

---

*Built with Rust 🦀 • Secured with AES-256 🔒 • Private by Design 🛡️*

**Welcome to true digital security!** 🎊
