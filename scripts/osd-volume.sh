#!/usr/bin/env bash
set -euo pipefail

arg=${1:-}
case "$arg" in
  '5%+') wpctl set-volume -l 1.0 @DEFAULT_AUDIO_SINK@ 5%+ ;;
  '5%-') wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%- ;;
  toggle) wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle ;;
  *) echo "usage: $0 {5%+|5%-|toggle}" >&2; exit 2 ;;
esac

volume_info=$(wpctl get-volume @DEFAULT_AUDIO_SINK@)
if [[ "$volume_info" == *"[MUTED]"* ]]; then
  percent=0
  body='MUTED'
else
  volume=$(awk '{print $2}' <<<"$volume_info")
  percent=$(awk -v value="$volume" 'BEGIN { n=int(value * 100 + 0.5); if (n > 100) n=100; if (n < 0) n=0; print n }')
  body="${percent}%"
fi

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo=$(cd -- "$script_dir/.." && pwd)
icon="${XDG_RUNTIME_DIR:-/tmp}/rlax-volume-gashat.png"
render_args=()
if [[ "$volume_info" == *"[MUTED]"* ]]; then
  render_args+=(--muted)
fi
python3 "$script_dir/render-gashat-osd.py" \
  --asset "$repo/assets/gashatExAid-removebg-preview.png" \
  --output "$icon" --percent "$percent" --label VOL \
  --bar '#AFFF35' "${render_args[@]}"

notify-send --app-name="Rlax Volume OSD" \
  --icon="$icon" \
  --expire-time=1800 \
  --hint=string:x-canonical-private-synchronous:rlax-volume \
  --urgency=low \
  'VOLUME // EX-AID GASHAT' "$body"