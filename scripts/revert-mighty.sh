#!/bin/bash
cp -r /tmp/mighty-backup-20261015/hypr ~/.config/ 2>/dev/null
cp -r /tmp/mighty-backup-20261015/waybar ~/.config/ 2>/dev/null
cp -r /tmp/mighty-backup-20261015/kitty ~/.config/ 2>/dev/null
cp -r /tmp/mighty-backup-20261015/mako ~/.config/ 2>/dev/null
echo "Reverted. Run: hyprctl reload; pkill waybar; waybar &"
