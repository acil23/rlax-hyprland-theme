#!/usr/bin/env bash
# Helper to add rlax-launcher bind to Hyprland
set -e
HYPR_CONF="$HOME/.config/hypr/hyprland.conf"
BIN="/home/aklein/Data/Rlax/customApp/target/debug/rlax-launcher"
RELEASE="/home/aklein/Data/Rlax/customApp/target/release/rlax-launcher"

USE_BIN="$BIN"
[ -f "$RELEASE" ] && USE_BIN="$RELEASE"

echo "Will bind: SUPER + SPACE -> $USE_BIN"
echo ""
echo "Tambahkan baris ini ke $HYPR_CONF:"
echo ""
echo "  bind = SUPER, SPACE, exec, $USE_BIN"
echo ""
echo "Lalu reload: hyprctl reload"
if grep -q "rlax-launcher" "$HYPR_CONF" 2>/dev/null; then
    echo "[OK] sudah ada bind rlax-launcher di hyprland.conf"
else
    read -p "Mau auto-append ke hyprland.conf? (y/N) " ans
    if [[ "$ans" == "y" || "$ans" == "Y" ]]; then
        echo "" >> "$HYPR_CONF"
        echo "# Rlax launcher" >> "$HYPR_CONF"
        echo "bind = SUPER, SPACE, exec, $USE_BIN" >> "$HYPR_CONF"
        echo "Appended. Jalankan: hyprctl reload"
    fi
fi
