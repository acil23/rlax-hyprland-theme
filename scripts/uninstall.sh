#!/usr/bin/env bash
set -euo pipefail
DATA_HOME=${XDG_DATA_HOME:-$HOME/.local/share}
STATE=$DATA_HOME/rlax-theme
MANIFEST=$STATE/installed-files
BACKUP_INDEX=$STATE/backups.tsv
if [[ ! -f $MANIFEST ]]; then
  echo "No Rlax theme installation manifest found at $MANIFEST" >&2
  exit 1
fi
while IFS= read -r destination; do
  [[ -n $destination ]] || continue
  backup=
  if [[ -f $BACKUP_INDEX ]]; then
    backup=$(awk -F '\t' -v dst="$destination" '$1 == dst { value=$2 } END { print value }' "$BACKUP_INDEX")
  fi
  if [[ -n $backup && -e $backup ]]; then
    mkdir -p "$(dirname -- "$destination")"
    rm -rf -- "$destination"
    cp -a -- "$backup" "$destination"
    echo "restored $destination from $backup"
  elif [[ -e $destination || -L $destination ]]; then
    rm -rf -- "$destination"
    echo "removed $destination"
  fi
done < "$MANIFEST"
rm -f -- "$MANIFEST"
rm -f -- "$BACKUP_INDEX"
echo "Rlax theme files removed/restored. Backups retained at $STATE/backups for manual recovery."