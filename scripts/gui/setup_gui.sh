#!/bin/bash
# KayOS Desktop Setup (XFCE)
sudo apt update && sudo apt install -y xfce4 xfce4-goodies lightdm
sudo mkdir -p /usr/share/kayos/branding
sudo cp /home/kaysel/Kay.ky/assets/branding/* /usr/share/kayos/branding/
echo '[Desktop Entry]
Name=KayOS
Comment=KayOS Cybersecurity Desktop
Exec=/home/kaysel/Kay.ky/target/release/kayky
Icon=/usr/share/kayos/branding/icon.png
Type=Application' | sudo tee /usr/share/xsessions/kayos.desktop
# Set wallpaper (XFCE specific)
xfconf-query -c xfce4-desktop -p /backdrop/screen0/monitor0/workspace0/last-image -s /usr/share/kayos/branding/wallpaper.png --create -t string
