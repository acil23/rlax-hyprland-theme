#!/usr/bin/env bash
set -Eeuo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/install.sh [--all | --component NAME ...] [--dry-run] [--prefix PATH]

Components: launcher, power-menu, osd, waybar, mako, hyprlock, sddm, font
Default:   launcher, power-menu, osd, waybar, mako, hyprlock, font
Options:
  --all                 Include optional SDDM greeter theme
  --component NAME      Install one or more components (repeatable)
  --prefix PATH         Install under PATH (default: ~/.local)
  --dry-run             Print planned file operations; do not build or write
  -h, --help            Show this help

The installer never edits Hyprland's main configuration. Optional integration
snippets are installed under PREFIX/share/rlax-theme/hypr/ for manual inclusion.
EOF
}

ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
PREFIX=${HOME}/.local
DRY_RUN=0
COMPONENTS=()
DEFAULT_COMPONENTS=(launcher power-menu osd waybar mako hyprlock font)

while (($#)); do
  case "$1" in
    --all) COMPONENTS=("${DEFAULT_COMPONENTS[@]}" sddm); shift ;;
    --component) (($# >= 2)) || { usage >&2; exit 2; }; COMPONENTS+=("$2"); shift 2 ;;
    --prefix) (($# >= 2)) || { usage >&2; exit 2; }; PREFIX=$2; shift 2 ;;
    --dry-run) DRY_RUN=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown option: $1" >&2; usage >&2; exit 2 ;;
  esac
done
((${#COMPONENTS[@]})) || COMPONENTS=("${DEFAULT_COMPONENTS[@]}")

declare -A seen=()
for component in "${COMPONENTS[@]}"; do
  case "$component" in launcher|power-menu|osd|waybar|mako|hyprlock|sddm|font) ;;
    *) echo "Unknown component: $component" >&2; exit 2;;
  esac
  [[ ${seen[$component]+yes} ]] && { echo "Duplicate component: $component" >&2; exit 2; }
  seen[$component]=1
done
# The themed Waybar power button depends on the matching power-menu app.
if [[ ${seen[waybar]+yes} ]]; then seen[power-menu]=1; fi

DATA_HOME=${XDG_DATA_HOME:-$HOME/.local/share}
CONFIG_HOME=${XDG_CONFIG_HOME:-$HOME/.config}
BIN_DIR=$PREFIX/bin
SHARE_DIR=$PREFIX/share/rlax-theme
BACKUP_ROOT=$DATA_HOME/rlax-theme/backups/$(date +%Y%m%d-%H%M%S)-$$
MANIFEST=$DATA_HOME/rlax-theme/installed-files
BACKUP_INDEX=$DATA_HOME/rlax-theme/backups.tsv

log() { printf '[rlax-theme] %s\n' "$*"; }
run() { if ((DRY_RUN)); then printf 'DRY RUN:'; printf ' %q' "$@"; printf '\n'; else "$@"; fi; }
install_file() {
  local source=$1 destination=$2 mode=${3:-0644}
  if ((DRY_RUN)); then log "would install $source -> $destination"; return; fi
  mkdir -p "$(dirname -- "$destination")"
  if [[ -f $MANIFEST ]] && grep -Fqx -- "$destination" "$MANIFEST"; then
    : # Already owned by this installation; keep the original pre-install backup.
  elif [[ -e $destination || -L $destination ]]; then
    local backup="$BACKUP_ROOT/${destination#/}"
    mkdir -p "$(dirname -- "$backup")"
    cp -a -- "$destination" "$backup"
    mkdir -p "$(dirname -- "$BACKUP_INDEX")"
    printf '%s\t%s\n' "$destination" "$backup" >> "$BACKUP_INDEX"
    log "backup: $destination -> $backup"
  fi
  install -m "$mode" -- "$source" "$destination"
  mkdir -p "$(dirname -- "$MANIFEST")"
  touch "$MANIFEST"
  grep -Fqx -- "$destination" "$MANIFEST" || printf '%s\n' "$destination" >> "$MANIFEST"
}
need_command() { command -v "$1" >/dev/null || { echo "Missing required command: $1" >&2; exit 1; }; }

if ((DRY_RUN)); then log "dry-run: prefix=$PREFIX components=${COMPONENTS[*]}"; fi

needs_build=0
[[ ${seen[launcher]+yes} || ${seen[power-menu]+yes} ]] && needs_build=1
if ((needs_build)); then
  need_command cargo
  packages=()
  [[ ${seen[launcher]+yes} ]] && packages+=(rlax-launcher)
  [[ ${seen[power-menu]+yes} ]] && packages+=(rlax-power-menu)
  if ((!DRY_RUN)); then
    log "building release binaries: ${packages[*]}"
    for package in "${packages[@]}"; do
      (cd "$ROOT" && cargo build --release --locked -p "$package")
    done
  fi
fi

if [[ ${seen[launcher]+yes} ]]; then
  install_file "$ROOT/target/release/rlax-launcher" "$BIN_DIR/rlax-launcher" 0755
  install_file "$ROOT/assets/exaid.png" "$SHARE_DIR/assets/exaid.png"
fi
if [[ ${seen[power-menu]+yes} ]]; then
  install_file "$ROOT/target/release/rlax-power-menu" "$BIN_DIR/rlax-power-menu" 0755
  install_file "$ROOT/assets/power_menu.jpeg" "$SHARE_DIR/assets/power_menu.jpeg"
fi
if [[ ${seen[osd]+yes} ]]; then
  need_command python3
  python3 -c 'from PIL import Image' 2>/dev/null || { echo 'OSD requires Python Pillow (python-pillow on Arch).' >&2; exit 1; }
  for file in osd-volume.sh osd-brightness.sh render-gashat-osd.py; do
    install_file "$ROOT/scripts/$file" "$BIN_DIR/$file" 0755
  done
  install_file "$ROOT/assets/gashatExAid-removebg-preview.png" "$SHARE_DIR/assets/gashatExAid-removebg-preview.png"
  install_file "$ROOT/assets/gashatGemn-removebg-preview.png" "$SHARE_DIR/assets/gashatGemn-removebg-preview.png"
fi
if [[ ${seen[waybar]+yes} ]]; then
  install_file "$ROOT/configs/waybar/style.css" "$CONFIG_HOME/waybar/style.css"
  local_config=$(mktemp)
  python3 - "$ROOT/configs/waybar/config" "$local_config" "$BIN_DIR" <<'PY'
import json, pathlib, sys
source, output, bindir = sys.argv[1:]
config = json.loads(pathlib.Path(source).read_text())
config['temperature'].pop('hwmon-path', None)
for module in ('cpu', 'memory'):
    config[module]['on-click'] = 'foot -e btop || kitty -e btop || alacritty -e btop'
config['network']['on-click'] = 'foot -e nmtui || kitty -e nmtui || nm-connection-editor'
config['custom/power']['on-click'] = f'{bindir}/rlax-power-menu'
config['custom/power']['on-click-right'] = f'{bindir}/rlax-power-menu'
pathlib.Path(output).write_text(json.dumps(config, indent=4) + '\n')
PY
  install_file "$local_config" "$CONFIG_HOME/waybar/config" 0644
  rm -f "$local_config"
fi
if [[ ${seen[mako]+yes} ]]; then install_file "$ROOT/configs/mako/config" "$CONFIG_HOME/mako/config"; fi
if [[ ${seen[hyprlock]+yes} ]]; then
  install_file "$ROOT/assets/login_background.jpeg" "$SHARE_DIR/assets/login_background.jpeg"
  install_file "$ROOT/configs/hypr/rlax-theme.conf" "$SHARE_DIR/hypr/rlax-theme.conf"
  local_config=$(mktemp)
  sed "s|@RLAX_THEME_ASSETS@|$SHARE_DIR/assets|g" "$ROOT/configs/hypr/hyprlock.conf" > "$local_config"
  install_file "$local_config" "$CONFIG_HOME/hypr/rlax-hyprlock.conf"
  rm -f "$local_config"
  log "hyprlock config installed separately at $CONFIG_HOME/hypr/rlax-hyprlock.conf; select it in your lock command if desired"
fi
if [[ ${seen[font]+yes} ]]; then
  install_file "$ROOT/assets/fonts/PressStart2P-Regular.ttf" "$DATA_HOME/fonts/PressStart2P-Regular.ttf"
  if command -v fc-cache >/dev/null && ((!DRY_RUN)); then fc-cache -f "$DATA_HOME/fonts" >/dev/null; fi
fi
if [[ ${seen[sddm]+yes} ]]; then
  log "SDDM theme files are installed only in user data; system SDDM settings are not changed."
  install_file "$ROOT/configs/sddm/rlax-gashat/Main.qml" "$SHARE_DIR/sddm/rlax-gashat/Main.qml"
  install_file "$ROOT/configs/sddm/rlax-gashat/metadata.desktop" "$SHARE_DIR/sddm/rlax-gashat/metadata.desktop"
  install_file "$ROOT/configs/sddm/rlax-gashat/login_background.jpeg" "$SHARE_DIR/sddm/rlax-gashat/login_background.jpeg"
fi

# Repair only known Rlax command references in the user's Lua config. Keep all
# other bindings and settings intact, and backup each touched file once.
if [[ -d "$CONFIG_HOME/hypr" && ( ${seen[launcher]+yes} || ${seen[power-menu]+yes} || ${seen[osd]+yes} || ${seen[hyprlock]+yes} ) ]]; then
  for file in programs.lua keybinds.lua hyprlock.conf; do
    config_file="$CONFIG_HOME/hypr/$file"
    [[ -f $config_file ]] || continue
    if grep -qE 'customApp|rlax-(launcher|power-menu)|osd-(volume|brightness)|login_background\.jpeg' "$config_file"; then
      if ((DRY_RUN)); then
          log "would update Rlax command references in $config_file (backup first)"
      else
        if ! grep -Fqx -- "$config_file" "$MANIFEST" 2>/dev/null; then
          backup="$BACKUP_ROOT/${config_file#/}"
          mkdir -p "$(dirname -- "$backup")" "$(dirname -- "$BACKUP_INDEX")"
          cp -a -- "$config_file" "$backup"
          printf '%s\t%s\n' "$config_file" "$backup" >> "$BACKUP_INDEX"
          mkdir -p "$(dirname -- "$MANIFEST")"
          touch "$MANIFEST"
          printf '%s\n' "$config_file" >> "$MANIFEST"
          log "backup: $config_file -> $backup"
        fi
        python3 - "$config_file" "$BIN_DIR" "$DATA_HOME" <<'PY'
import pathlib
import re
import sys

path = pathlib.Path(sys.argv[1])
bin_dir = sys.argv[2]
data_home = sys.argv[3]
text = path.read_text()
replacements = (
    (r"[^\s\"']*/target/(?:debug|release)/rlax-launcher", f'{bin_dir}/rlax-launcher'),
    (r"[^\s\"']*/target/(?:debug|release)/rlax-power-menu", f'{bin_dir}/rlax-power-menu'),
    (r"[^\s\"']*/scripts/osd-volume\.sh", f'{bin_dir}/osd-volume.sh'),
    (r"[^\s\"']*/scripts/osd-brightness\.sh", f'{bin_dir}/osd-brightness.sh'),
    (r"[^\s\"']*/assets/login_background\.jpeg", f'{data_home}/rlax-theme/assets/login_background.jpeg'),
)
for pattern, replacement in replacements:
    text = re.sub(pattern, lambda _: replacement, text)
path.write_text(text)
PY
      fi
    fi
  done
  if [[ ${seen[hyprlock]+yes} && -f "$CONFIG_HOME/hypr/hyprlock.conf" ]] && grep -q 'login_background.jpeg' "$CONFIG_HOME/hypr/hyprlock.conf"; then
    log "updated hyprlock artwork reference in $CONFIG_HOME/hypr/hyprlock.conf"
  fi
  if [[ ${seen[launcher]+yes} && -f "$CONFIG_HOME/hypr/programs.lua" ]] && ! grep -q 'menu = .*rlax-launcher' "$CONFIG_HOME/hypr/programs.lua"; then
          log "launcher command may need manual setting in $CONFIG_HOME/hypr/programs.lua"
  fi
fi

if ((!DRY_RUN)); then
  [[ ${seen[hyprlock]+yes} ]] || install_file "$ROOT/configs/hypr/rlax-theme.conf" "$SHARE_DIR/hypr/rlax-theme.conf"
  log "installation complete; optional Hyprland snippet: $SHARE_DIR/hypr/rlax-theme.conf"
  log "backup directory: $BACKUP_ROOT"
  if [[ -n ${HYPRLAND_INSTANCE_SIGNATURE:-} ]] && command -v hyprctl >/dev/null; then
    log "reloading Hyprland configuration to activate repaired keybindings"
    hyprctl reload || log "reload failed; run 'hyprctl reload' manually after resolving config errors"
  else
    log "run 'hyprctl reload' to activate repaired keybindings"
  fi
else
  log 'dry-run complete; no files changed'
fi