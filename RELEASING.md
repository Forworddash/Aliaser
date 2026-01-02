# 🚀 Creating GitHub Releases

This guide explains how to create releases for Aliaser on GitHub with pre-built binaries for Windows, Linux, and macOS.

## 📋 Prerequisites

1. **GitHub Repository**: Push your code to GitHub
2. **GitHub Actions Enabled**: Should be enabled by default
3. **Git Tags**: Used to trigger releases

## 🎯 Quick Release Process

### Step 1: Commit and Push Your Code

```bash
cd /home/deneuve/Documents/Aliaser

# Make sure everything is committed
git add .
git commit -m "Prepare for v0.1.0 release"
git push origin main
```

### Step 2: Create and Push a Version Tag

```bash
# Create a tag for version 0.1.0
git tag -a v0.1.0 -m "Release version 0.1.0"

# Push the tag to GitHub
git push origin v0.1.0
```

### Step 3: Automatic Build and Release

GitHub Actions will automatically:
1. Build binaries for all platforms (5 targets)
2. Create a GitHub Release
3. Upload all binaries to the release
4. Add release notes

**Build Time:** ~10-15 minutes for all platforms

## 📦 What Gets Built

The release workflow builds binaries for:

| Platform | Architecture | File Name | Format |
|----------|-------------|-----------|--------|
| **Linux** | x86_64 | `aliaser-linux-x86_64.tar.gz` | tar.gz |
| **Linux** | ARM64 | `aliaser-linux-aarch64.tar.gz` | tar.gz |
| **macOS** | Intel (x86_64) | `aliaser-macos-x86_64.tar.gz` | tar.gz |
| **macOS** | Apple Silicon (ARM64) | `aliaser-macos-aarch64.tar.gz` | tar.gz |
| **Windows** | x86_64 | `aliaser-windows-x86_64.exe.zip` | zip |

## 🔧 Release Workflow

The `.github/workflows/release.yml` file handles:

### Build Phase
- Compiles optimized release binaries
- Strips debug symbols (Linux/macOS)
- Cross-compiles for all platforms
- Creates compressed archives

### Release Phase
- Creates a GitHub Release
- Uploads all binaries
- Adds formatted release notes
- Tags the release properly

## 📝 Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- `v0.1.0` - Initial release
- `v0.1.1` - Bug fixes (patch)
- `v0.2.0` - New features (minor)
- `v1.0.0` - Stable release (major)

## 🎯 Complete Example

```bash
# 1. Make your changes
git add .
git commit -m "Add new feature"
git push

# 2. Create a release tag
git tag -a v0.1.0 -m "Initial release

Features:
- AES-256-GCM encryption
- Identity management
- Password generation
- Backup/restore
"

# 3. Push the tag
git push origin v0.1.0

# 4. Watch the build on GitHub
# Go to: https://github.com/YOUR_USERNAME/Aliaser/actions
```

## 🌐 Where Users Download

After the release is created:

1. **Release Page**: `https://github.com/YOUR_USERNAME/Aliaser/releases`
2. **Latest Release**: `https://github.com/YOUR_USERNAME/Aliaser/releases/latest`
3. **Direct Downloads**: Users can download specific binaries

## 👥 User Installation Instructions

### For Linux/macOS Users

```bash
# Download (example for Linux x86_64)
wget https://github.com/YOUR_USERNAME/Aliaser/releases/download/v0.1.0/aliaser-linux-x86_64.tar.gz

# Extract
tar xzf aliaser-linux-x86_64.tar.gz

# Make executable
chmod +x aliaser

# Install system-wide (optional)
sudo mv aliaser /usr/local/bin/

# Run
aliaser init
```

### For Windows Users

1. Download `aliaser-windows-x86_64.exe.zip`
2. Extract the zip file
3. Run `aliaser.exe` in PowerShell or Command Prompt
4. Optionally add to PATH

### For macOS Users

**Intel Macs:**
```bash
wget https://github.com/YOUR_USERNAME/Aliaser/releases/download/v0.1.0/aliaser-macos-x86_64.tar.gz
tar xzf aliaser-macos-x86_64.tar.gz
chmod +x aliaser
sudo mv aliaser /usr/local/bin/
```

**Apple Silicon Macs (M1/M2/M3):**
```bash
wget https://github.com/YOUR_USERNAME/Aliaser/releases/download/v0.1.0/aliaser-macos-aarch64.tar.gz
tar xzf aliaser-macos-aarch64.tar.gz
chmod +x aliaser
sudo mv aliaser /usr/local/bin/
```

## 🔍 Verifying Releases

Users can verify the build by:

1. **Checking the workflow**: View the GitHub Actions build logs
2. **Building from source**: Compare with their own build
3. **Hash verification**: Compare file hashes (add checksums in future)

## 🚨 Important Notes

### Security Considerations

1. **Signed Releases**: Consider code signing for future releases
2. **Checksums**: Add SHA256 checksums for verification
3. **GPG Signatures**: Sign releases with GPG key
4. **Reproducible Builds**: Ensure builds are reproducible

### Platform-Specific Issues

**macOS:**
- Users may need to allow the app in Security & Privacy settings
- Consider notarization for macOS in the future

**Windows:**
- Users may see SmartScreen warnings
- Consider code signing certificate

**Linux:**
- Works on most distributions
- May need `libc` compatibility checks

## 📈 Release Checklist

Before creating a release:

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG updated (create one)
- [ ] Version number bumped in `Cargo.toml`
- [ ] Security audit passed (`cargo audit`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] README includes version info
- [ ] License file present (GPL-3.0)

## 🔄 Continuous Integration

The `.github/workflows/build.yml` runs on every push:

- Runs tests on Linux, Windows, macOS
- Checks code formatting
- Runs clippy linter
- Performs security audit
- Builds release binaries

## 🛠️ Troubleshooting

### Build Fails on GitHub Actions

1. Check the Actions tab for error logs
2. Test locally: `cargo build --release`
3. Ensure dependencies are available
4. Check platform-specific issues

### Tag Already Exists

```bash
# Delete local tag
git tag -d v0.1.0

# Delete remote tag
git push origin :refs/tags/v0.1.0

# Recreate tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### Release Draft Not Creating

1. Check repository permissions
2. Ensure GITHUB_TOKEN has write permissions
3. Verify workflow YAML syntax

## 📊 Release Analytics

Track your releases:

1. **Download counts**: GitHub shows download statistics
2. **User feedback**: Monitor issues and discussions
3. **Stars/forks**: Track repository growth

## 🎯 Future Enhancements

Consider adding:

- [ ] Checksums file (SHA256SUMS)
- [ ] GPG signatures
- [ ] Code signing (Windows/macOS)
- [ ] Homebrew formula (macOS)
- [ ] Debian package (.deb)
- [ ] RPM package
- [ ] AUR package (Arch Linux)
- [ ] Chocolatey package (Windows)
- [ ] Snap package (Linux)
- [ ] Docker image

## 📚 Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Creating Releases](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)
- [Semantic Versioning](https://semver.org/)
- [Rust Cross-Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)

## 💡 Pro Tips

1. **Test locally first**: Build for your platform before releasing
2. **Use pre-releases**: Tag as `v0.1.0-beta` for testing
3. **Write good release notes**: Explain what's new and what changed
4. **Keep a CHANGELOG**: Track all changes between versions
5. **Automate testing**: Let CI catch issues before release

## ✅ Your First Release

Run these commands to create your first release:

```bash
# From the project root
cd /home/deneuve/Documents/Aliaser

# Ensure everything is committed
git status
git add .
git commit -m "Prepare v0.1.0 release"

# Create and push the tag
git tag -a v0.1.0 -m "Initial release of Aliaser

Features:
- Secure password and identity management
- AES-256-GCM encryption
- Argon2id key derivation
- Local-only storage
- Cross-platform support
- Comprehensive documentation
"

# Push commits and tag
git push origin main
git push origin v0.1.0

# Watch the build at:
# https://github.com/YOUR_USERNAME/Aliaser/actions
```

---

**Congratulations!** Your users can now download and use Aliaser on any platform! 🎉

For questions or issues with releases, check the [GitHub Actions documentation](https://docs.github.com/en/actions) or open an issue.
