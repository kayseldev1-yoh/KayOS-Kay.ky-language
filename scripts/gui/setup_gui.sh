#!/bin/bash
# KayOS Desktop Setup - Selection Menu
echo "Choose your preferred Desktop Environment:"
echo "1) Xfce (Lightweight & Fast - Recommended)"
echo "2) GNOME (Simple & Modern)"
echo "3) KDE Plasma (Powerful & Customizable)"
read -p "Select [1-3]: " choice

case $choice in
    1) sudo apt install -y xfce4 xfce4-goodies ;;
    2) sudo apt install -y gnome-shell gnome-session ;;
    3) sudo apt install -y kde-plasma-desktop ;;
    *) echo "Invalid option, defaulting to Xfce"; sudo apt install -y xfce4 ;;
esac

sudo apt install -y lightdm
echo "[Desktop Entry]
Name=KayOS
Exec=/home/kaysel/Kay.ky/target/release/kayky
Icon=/usr/share/kayos/branding/icon.png
Type=Application" | sudo tee /usr/share/xsessions/kayos.desktop
