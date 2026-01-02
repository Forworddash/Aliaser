# Aliaser Project Summary

## 🎯 Project Overview

**Aliaser** is a secure, open-source identity and password manager written in Rust. It stores all data locally with military-grade encryption and zero network connectivity.

## 📁 Project Structure

```
Aliaser/
├── src/
│   ├── main.rs          # Application entry point
│   ├── cli.rs           # Command-line interface and user interaction
│   ├── crypto.rs        # Encryption/decryption (AES-256-GCM, Argon2id)
│   ├── identity.rs      # Data structures for identities
│   └── storage.rs       # Vault management and file I/O
│
├── Cargo.toml           # Rust dependencies and project config
├── Cargo.lock           # Locked dependency versions
│
├── README.md            # Main documentation
├── QUICKSTART.md        # Quick start guide with examples
├── SECURITY.md          # Security policy and practices
├── SECURITY_COMPARISON.md # Detailed security analysis
├── LICENSE              # GPL License
├── .gitignore           # Git ignore rules
│
├── demo.sh              # Interactive demo script
│
└── target/
    └── release/
        └── aliaser      # Compiled binary (after build)
```

## 🔧 Technical Stack

### Core Technologies
- **Language:** Rust 2021 Edition
- **Build System:** Cargo

### Security Dependencies
- `aes-gcm` v0.10 - AES-256-GCM encryption
- `argon2` v0.5 - Key derivation and password hashing
- `rand` v0.8 - Cryptographically secure random numbers
- `zeroize` v1.7 - Secure memory clearing

### CLI & Utilities
- `clap` v4.5 - Command-line parsing
- `rpassword` v7.3 - Secure password input
- `colored` v2.1 - Terminal colors
- `serde` v1.0 - Serialization
- `serde_json` v1.0 - JSON handling
- `chrono` v0.4 - Date/time
- `dirs` v5.0 - Home directory detection
- `anyhow` v1.0 - Error handling

## 🔐 Security Features

### Encryption
- **Algorithm:** AES-256-GCM (Authenticated Encryption)
- **Key Size:** 256 bits
- **Nonce:** 96 bits (random per operation)
- **Authentication:** Built-in via GCM mode

### Key Derivation
- **Algorithm:** Argon2id
- **Salt:** 256 bits (random per vault)
- **Output:** 256-bit encryption key
- **Resistance:** GPU, ASIC, side-channel attacks

### Password Security
- Master password never stored (only hash)
- Minimum 8 character requirement
- Confirmation required on setup
- Zeroized from memory after use

### Memory Safety
- Rust's guaranteed memory safety
- No buffer overflows
- No use-after-free
- Zeroization of sensitive data structures

### Data Protection
- All credentials encrypted at rest
- Personal information encrypted
- Vault file integrity verified (AEAD)
- Configuration separate from vault

### Privacy
- Zero network code
- No telemetry or analytics
- No data collection
- Completely offline
- Open source for auditing

## 📝 Supported Data Types

### Credentials
- Username
- Password (with auto-generation)
- Email
- Alias names

### Personal Information (Optional)
- First Name
- Last Name
- Birthdate
- Address
- Phone Number
- Custom key-value fields

### Metadata
- Service name
- Creation timestamp
- Last updated timestamp
- Notes/comments

## 🚀 Commands

| Command | Description |
|---------|-------------|
| `init` | Initialize a new vault with master password |
| `add` | Add a new identity |
| `list` | List all stored services |
| `get <service>` | Retrieve and display an identity |
| `update <service>` | Update an existing identity |
| `delete <service>` | Delete an identity |
| `export <path>` | Export encrypted vault to file |
| `import <path>` | Import vault from file |
| `change-master` | Change master password |

## 💾 Data Storage

### Files Created
- `~/.aliaser.vault` - Encrypted vault data
- `~/.aliaser.config` - Configuration (hash and salt)

### Backup Format
- Exported files are encrypted vaults
- Require master password to decrypt
- Can be stored anywhere securely

## 🏗️ Build & Installation

### Development Build
```bash
cargo build
./target/debug/aliaser --help
```

### Release Build (Optimized)
```bash
cargo build --release
./target/release/aliaser --help
```

### System Installation
```bash
cargo install --path .
# Or manually:
sudo cp target/release/aliaser /usr/local/bin/
```

## 🧪 Testing

### Run Tests
```bash
cargo test
```

### Check for Issues
```bash
cargo clippy
```

### Security Audit
```bash
cargo audit
```

## 📖 Documentation Files

1. **README.md** - Main documentation
   - Features overview
   - Installation instructions
   - Usage examples
   - Security summary

2. **QUICKSTART.md** - Getting started guide
   - Step-by-step tutorial
   - Common workflows
   - Tips and best practices
   - Troubleshooting

3. **SECURITY.md** - Security policy
   - Reporting vulnerabilities
   - Security features details
   - Best practices for users
   - Known limitations
   - Threat model

4. **SECURITY_COMPARISON.md** - Detailed security analysis
   - Comparison with other password managers
   - Attack scenario analysis
   - Cryptographic details
   - When to use/not use

5. **LICENSE** - GPL License

## 🎯 Use Cases

### Ideal For
✅ Security professionals  
✅ Privacy advocates  
✅ Developers comfortable with CLI  
✅ Users needing local-only storage  
✅ Air-gapped systems  
✅ Identity management  
✅ Users who distrust cloud storage  

### Not Ideal For
❌ Users needing mobile apps  
❌ Teams needing shared access  
❌ Users wanting browser integration  
❌ Multi-device automatic sync  
❌ Users preferring GUI  

## 🔄 Typical Workflow

1. **Setup**
   ```bash
   aliaser init
   # Enter master password
   ```

2. **Add Credentials**
   ```bash
   aliaser add
   # Follow prompts
   ```

3. **Daily Use**
   ```bash
   aliaser list
   aliaser get GitHub
   ```

4. **Maintenance**
   ```bash
   # Update password
   aliaser update GitHub
   
   # Backup
   aliaser export backup.vault
   
   # Change master password
   aliaser change-master
   ```

## 🔮 Future Enhancements (Ideas)

- [ ] GUI interface (using egui or iced)
- [ ] TOTP/2FA code storage
- [ ] Password strength checker
- [ ] Browser integration
- [ ] Hardware security module support
- [ ] Post-quantum cryptography
- [ ] Secure notes feature
- [ ] File attachment encryption
- [ ] Password history
- [ ] Compromised password checking

## 📊 Performance

### Binary Size
- Debug: ~15 MB
- Release (unstripped): ~8 MB
- Release (stripped): ~5 MB

### Speed
- Init: < 1 second
- Add/Get/Update: < 100ms
- Unlock: ~1 second (Argon2id intentional delay)

### Resource Usage
- Memory: < 50 MB
- Disk: Vault scales with entries (~1-10 KB per identity)

## 🐛 Known Issues / Limitations

1. **CLI Only** - No GUI (yet)
2. **No Sync** - Manual export/import required
3. **No Mobile** - Desktop only currently
4. **Single User** - No multi-user support
5. **No Browser Integration** - Copy/paste required

## 🤝 Contributing

This is an open-source project. Contributions welcome:

### Areas for Contribution
- GUI frontend
- Mobile apps
- Browser extensions
- Additional export formats
- Password strength analysis
- Security audits
- Documentation improvements
- Bug fixes

### Development Setup
```bash
# Clone repository
git clone <repo-url>
cd Aliaser

# Build and test
cargo build
cargo test
cargo clippy

# Run
./target/debug/aliaser init
```

## 📜 License

GPL License - See LICENSE file for details.

## 🙏 Acknowledgments

### Cryptography Libraries
- RustCrypto team (aes-gcm, argon2)
- Rust Random project (rand)

### Inspiration
- KeePass/KeePassXC
- Bitwarden
- Pass (Unix password manager)

## 📞 Support

- Documentation: See README.md and QUICKSTART.md
- Security Issues: See SECURITY.md
- General Questions: Open GitHub issue

## 🎓 Learning Resources

### Understanding the Security
1. Read SECURITY_COMPARISON.md
2. Review crypto.rs source code
3. Study AES-GCM and Argon2id papers

### For Developers
1. Review source code (well-commented)
2. Read Rust documentation
3. Study dependencies documentation

## ⚖️ Threat Model

### Protected Against
✅ Disk theft  
✅ Network attacks  
✅ Brute force (with strong password)  
✅ Data tampering  
✅ Unauthorized access  

### NOT Protected Against
❌ Compromised OS  
❌ Keyloggers  
❌ Weak master passwords  
❌ Physical coercion  
❌ Memory attacks while unlocked  

## 🎯 Design Principles

1. **Security First** - No compromise on encryption
2. **Privacy by Default** - No telemetry, no network
3. **Simplicity** - Do one thing well
4. **Transparency** - Open source, auditable
5. **User Control** - All data local, user owned

## 📈 Project Status

**Version:** 0.1.0  
**Status:** Alpha/Beta  
**Stability:** Core features stable  
**Security:** Cryptography sound, needs formal audit  
**Maintenance:** Active development  

## 🚨 Important Reminders

⚠️ **Master password cannot be recovered**  
⚠️ **Keep backups in secure locations**  
⚠️ **Use strong passwords (16+ characters)**  
⚠️ **Regular exports recommended**  
⚠️ **Not yet professionally audited**  

---

**Built with ❤️ and Rust 🦀**

*Your security is your responsibility. Use wisely!* 🔐
