# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in Aliaser, please report it responsibly:

1. **DO NOT** open a public GitHub issue
2. Email the maintainers with details
3. Allow reasonable time for a fix before public disclosure

## Security Features

### Encryption
- AES-256-GCM (NIST-approved)
- Authenticated encryption with additional data (AEAD)
- Random nonces for each encryption operation
- No IV/nonce reuse

### Key Derivation
- Argon2id (winner of Password Hashing Competition)
- Random 256-bit salt per vault
- Memory-hard function resistant to GPU/ASIC attacks
- Time and memory cost parameters follow OWASP recommendations

### Password Requirements
- Minimum 8 characters (recommended 16+)
- Password confirmation during setup
- Master password never stored (only hash)
- Separate hash for verification vs key derivation

### Memory Safety
- Sensitive data structures implement Zeroize
- Passwords cleared from memory after use
- Rust's memory safety guarantees prevent buffer overflows

### Data Protection
- All vault data encrypted at rest
- Configuration file stores only hash and salt (no secrets)
- No network connectivity (offline-only)
- No telemetry or analytics

## Best Practices for Users

1. **Strong Master Password**
   - Use 16+ characters
   - Mix uppercase, lowercase, numbers, symbols
   - Avoid common words or patterns
   - Use a passphrase if easier to remember

2. **Physical Security**
   - Encrypt your disk (LUKS, FileVault, BitLocker)
   - Lock your computer when away
   - Use secure boot if available

3. **Backups**
   - Regularly export vault to secure location
   - Store backups encrypted
   - Test restoration periodically
   - Keep backups offline or in encrypted cloud storage

4. **Updates**
   - Keep Aliaser updated
   - Monitor security advisories
   - Rebuild from source for critical updates

5. **Operational Security**
   - Run on trusted systems only
   - Avoid compromised/malware-infected machines
   - Use in private (shoulder surfing risk)
   - Clear terminal history if sensitive

## Known Limitations

1. **No Cloud Sync**: Manual export/import required for multi-device
2. **CLI Only**: Terminal access required (GUI planned)
3. **Single User**: No multi-user support
4. **No 2FA Storage**: TOTP/U2F storage not implemented yet

## Audit Status

This is open-source software. Professional security audit status: **Not yet audited**

You are encouraged to:
- Review the source code
- Run security analysis tools
- Report findings responsibly

## Dependencies Security

All dependencies are regularly updated. Key security dependencies:

- `aes-gcm` - Maintained by RustCrypto team
- `argon2` - Reference implementation
- `rand` - RustCrypto random number generation
- `zeroize` - Secure memory clearing

Run `cargo audit` to check for known vulnerabilities in dependencies.

## Cryptographic Design Principles

1. **No Custom Crypto**: Use well-tested, standard libraries
2. **Defense in Depth**: Multiple layers of protection
3. **Fail Securely**: Errors don't leak information
4. **Minimal Attack Surface**: Offline-only, no network code
5. **Auditability**: Open source, readable code

## Threat Model

### Protected Against
- Disk theft (encryption at rest)
- Memory dumps (zeroization)
- Brute force (Argon2id)
- Unauthorized access (master password)
- Data tampering (AEAD authentication)

### NOT Protected Against
- Compromised operating system
- Keyloggers/malware on host
- Physical torture (XKCD 538)
- Quantum computers (future threat)
- Side-channel attacks (timing, power analysis)

### Out of Scope
- Network attacks (no network code)
- Social engineering
- Backup security (user responsibility)
- Hardware security modules

## Future Security Improvements

Planned enhancements:
- [ ] Post-quantum cryptography readiness
- [ ] Hardware security module support
- [ ] Biometric authentication integration
- [ ] Secure enclave support (SGX, SEV)
- [ ] Formal security audit
- [ ] Security-focused GUI implementation

## Responsible Disclosure Timeline

1. Report received → 24 hours: Acknowledgment
2. 7 days: Initial assessment
3. 30 days: Fix developed and tested
4. 60 days: Public disclosure (if not fixed, coordinated disclosure)

## Security Contact

For security issues: [Add your security contact email]

---

**Remember**: No software is 100% secure. Security is a process, not a product.
