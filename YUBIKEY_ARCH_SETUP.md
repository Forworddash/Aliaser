# YubiKey Setup on Arch Linux

## 📝 Prerequisites for YubiKey on Arch Linux

The error you encountered indicates that PC/SC (Personal Computer/Smart Card) support is not available:

```
WARNING: PC/SC not available. Smart card (CCID) protocols will not function.
ERROR: Unable to list devices for connection
ERROR: Failed to connect to YubiKey.
```

## 🔧 Installation Steps

### 1. Install Required Packages

```bash
# Install pcsclite (PC/SC Smart Card Daemon)
sudo pacman -S pcsclite

# Install YubiKey Manager
sudo pacman -S yubikey-manager

# Install CCID driver (for smart card readers)
sudo pacman -S ccid

# Optional: Install YubiKey personalization tools
sudo pacman -S yubikey-personalization-gui
```

### 2. Enable and Start pcscd Service

```bash
# Enable the PC/SC daemon to start on boot
sudo systemctl enable pcscd.service

# Start the PC/SC daemon now
sudo systemctl start pcscd.service

# Check the status
sudo systemctl status pcscd.service
```

### 3. Add Your User to pcscd Group (Optional)

```bash
# Add your user to the pcscd group
sudo usermod -a -G pcscd $USER

# Log out and log back in for changes to take effect
```

### 4. Test YubiKey Detection

```bash
# Check if YubiKey is detected
ykman list

# Get YubiKey info
ykman info

# Test USB connection
lsusb | grep Yubico
```

## ⚙️ Configure YubiKey for HMAC-SHA1

Once the YubiKey is detected, configure slot 2 for challenge-response:

```bash
# Configure slot 2 with HMAC-SHA1 challenge-response
# The --touch flag requires physical touch for each use (recommended for security)
ykman otp chalresp --touch --generate 2

# Or without touch requirement (less secure):
ykman otp chalresp --generate 2

# Verify the configuration
ykman otp info
```

## 🧪 Test the Setup

### Test YubiKey with ykman

```bash
# Send a test challenge
echo "test challenge" | ykman otp calculate 2 -

# This should prompt you to touch the YubiKey and return a response
```

### Test with Aliaser

```bash
cd /home/deneuve/Documents/Aliaser

# Run aliaser with YubiKey support
./target/release/aliaser init

# When prompted, enable YubiKey authentication
# Touch your YubiKey when requested
```

## 🚨 Troubleshooting

### YubiKey Not Detected

```bash
# Check USB devices
lsusb | grep Yubico

# Check pcscd status
sudo systemctl status pcscd

# Restart pcscd
sudo systemctl restart pcscd

# Check for conflicting services
ps aux | grep pcscd
```

### Permission Issues

```bash
# Check udev rules
ls -la /etc/udev/rules.d/*yubi*

# If missing, create udev rule:
sudo tee /etc/udev/rules.d/70-yubikey.rules << 'EOF'
# Udev rules for YubiKey
ACTION!="add|change", GOTO="yubico_end"

# Yubico YubiKey
SUBSYSTEM=="usb", ATTRS{idVendor}=="1050", GROUP="users", MODE="0660"

LABEL="yubico_end"
EOF

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Unplug and replug YubiKey
```

### PC/SC Still Not Working

```bash
# Check if pcscd is running
ps aux | grep pcscd

# Try running pcscd manually to see errors
sudo pcscd -f -d

# Check for conflicting smart card services
systemctl list-units | grep -i card

# Make sure no other service is blocking pcscd
sudo lsof /var/run/pcscd/pcscd.comm
```

## 📋 Quick Reference

### Essential Commands

```bash
# List YubiKeys
ykman list

# Get YubiKey info
ykman info

# Configure slot 2 for challenge-response
ykman otp chalresp --touch --generate 2

# Test challenge-response
ykman otp calculate 2 "test"

# Reset a slot (WARNING: destructive!)
ykman otp delete 2
```

### Service Management

```bash
# Start pcscd
sudo systemctl start pcscd

# Stop pcscd
sudo systemctl stop pcscd

# Restart pcscd
sudo systemctl restart pcscd

# Check status
sudo systemctl status pcscd

# View logs
sudo journalctl -u pcscd -f
```

## ✅ Complete Setup Script

Run this script to set up everything:

```bash
#!/bin/bash

echo "Installing YubiKey support on Arch Linux..."

# Install packages
sudo pacman -S --needed pcsclite yubikey-manager ccid

# Enable and start pcscd
sudo systemctl enable pcscd.service
sudo systemctl start pcscd.service

# Create udev rules
sudo tee /etc/udev/rules.d/70-yubikey.rules << 'EOF'
ACTION!="add|change", GOTO="yubico_end"
SUBSYSTEM=="usb", ATTRS{idVendor}=="1050", GROUP="users", MODE="0660"
LABEL="yubico_end"
EOF

# Reload udev
sudo udevadm control --reload-rules
sudo udevadm trigger

echo "Setup complete! Please unplug and replug your YubiKey."
echo "Then run: ykman list"
```

## 🎯 Next Steps

After successful setup:

1. **Configure YubiKey**: `ykman otp chalresp --touch --generate 2`
2. **Test Aliaser**: `./target/release/aliaser init`
3. **Enable YubiKey** when prompted
4. **Touch YubiKey** when the LED blinks

## 📚 Additional Resources

- [YubiKey Manager Documentation](https://docs.yubico.com/software/yubikey/tools/ykman/)
- [Arch Wiki - YubiKey](https://wiki.archlinux.org/title/YubiKey)
- [PC/SC-Lite Documentation](https://pcsclite.apdu.fr/)

---

**Note**: After installing pcsclite and starting the daemon, you should be able to use your YubiKey with Aliaser!
