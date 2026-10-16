#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <brightnessctl adjustment>" >&2
  exit 2
fi

brightnessctl set "$1" >/dev/null
brightness_percent=$(brightnessctl -m | awk -F, 'NR == 1 {gsub(/%/, "", $4); print $4}')
[[ "$brightness_percent" =~ ^[0-9]+$ ]] || {
  echo "could not read brightness percentage" >&2
  exit 1
}

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
asset_dir="${XDG_DATA_HOME:-$HOME/.local/share}/rlax-theme/assets"
icon="${XDG_RUNTIME_DIR:-/tmp}/rlax-brightness-gashat.png"
python3 "$script_dir/render-gashat-osd.py" \
  --asset "$asset_dir/gashatGemn-removebg-preview.png" \
  --output "$icon" --percent "$brightness_percent" --label BRI \
  --bar '#D9C7FF'
notify-send --app-name="Rlax Brightness OSD" \
  --icon="$icon" \
  --expire-time=1800 \
  --hint=string:x-canonical-private-synchronous:rlax-brightness \
  --urgency=low \
  'BRIGHTNESS // GEMN GASHAT' "${brightness_percent}%"