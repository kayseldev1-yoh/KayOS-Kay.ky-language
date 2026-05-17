#!/bin/bash
# KayOS Security Toolkit - Automatic Installer
# Script ini akan menginstal seluruh daftar security tools yang diminta.

echo "[*] Updating package list..."
sudo apt update -y

echo "[*] Installing Security Tools..."

# Network & Scanning
sudo apt install -y nmap zenmap netcat-traditional hping3 netdiscover arp-scan dmitry

# Cracking
sudo apt install -y hashcat hydra john crunch rainbowcrack

# Exploitation & Pentesting
sudo apt install -y metasploit-framework sqlmap crackmapexec netexec armitage beef-xss evil-winrm

# Wi-Fi & Wireless
sudo apt install -y aircrack-ng wifite reaver wifiphisher airgeddon kismet fern-wifi-cracker

# Web & Recon
sudo apt install -y gobuster whatweb amass dirb nikto sublist3r recon-ng ffuf feroxbuster arjun sstimap spiderfoot smtp-user-enum dirbuster wpscan parsero metagoofil

# Analysis & Forensics
sudo apt install -y wireshark binwalk yara autopsy capstone bulk-extractor autorecon assetfinder jadx hashid

# Miscellaneous & System
sudo apt install -y powershell sherlock impacket-scripts ettercap mimikatz steghide yersinia tiger tcpdump ligolo-ng scapy caido pspy ollydbg hoaxshell ghidra dsniff cowpatty cewl cadaver burpsuite apktool

echo "[+] Installation complete. All tools are ready for use in KayOS."
