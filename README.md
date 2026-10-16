# Rlax Ex-Aid Hyprland Theme

A modular Kamen Rider Ex-Aid / Mighty Action X desktop theme for Hyprland. It includes GTK4 layer-shell launcher and power menu applications, Waybar and Mako styling, Gashat volume/brightness OSD scripts, an optional hyprlock theme, and an optional SDDM greeter theme.

The installer is user-scoped by default (`~/.local` and `~/.config`) and creates backups before replacing files. It does not replace your Hyprland configuration or personal settings. When an existing Hyprland Lua `programs.lua`/`keybinds.lua` contains known old Rlax paths, it backs up that file and updates only those Rlax command paths so the already-defined shortcuts keep working.

## Components

| Component | What it installs | Default? |
|---|---|---:|
| `launcher` | `rlax-launcher` and Ex-Aid artwork | Yes |
| `power-menu` | `rlax-power-menu` and background artwork | Yes |
| `osd` | Volume/brightness scripts, renderer and Gashat assets | Yes |
| `waybar` | Ex-Aid Waybar config and CSS, adapted to the current user | Yes |
| `mako` | Theme notification configuration | Yes |
| `hyprlock` | Separate optional lock-screen config and background | Yes |
| `font` | Press Start 2P user font | Yes |
| `sddm` | Greeter theme files in user data only; does not change system SDDM | Only with `--all` |

## Requirements

The app build needs Rust/Cargo, a C toolchain, GTK4 development files, and gtk4-layer-shell development files. On Arch Linux:

```bash
sudo pacman -S --needed base-devel rust cargo gtk4 gtk4-layer-shell pkgconf
```

Runtime components used by the optional scripts/configuration:

```bash
sudo pacman -S --needed hyprland hyprlock waybar mako pipewire wireplumber brightnessctl libnotify python python-pillow
```

For icons, install a Nerd Font of your choice (the example configuration uses Nerd Font glyphs). OSD volume requires PipeWire/WirePlumber (`wpctl`); brightness OSD requires `brightnessctl`, a supported brightness device, `notify-send`, and Python Pillow. The launcher and power menu require a Wayland Hyprland session and GTK4 layer-shell support.

Other distributions: install equivalent packages for GTK4, gtk4-layer-shell, Rust/Cargo, Waybar, Mako, hyprlock, PipeWire/WirePlumber, brightnessctl, libnotify and Pillow. Package names vary by distribution.

## Install

Clone the repository, then run the installer from its root:

```bash
git clone https://github.com/acil23/rlax-hyprland-theme.git
cd rlax-hyprland-theme
./scripts/install.sh
```

The default install builds the two Rust applications in release mode and installs them to `~/.local/bin`. Ensure that directory is on `PATH` (for example, add `export PATH="$HOME/.local/bin:$PATH"` to your shell profile and restart the session). User assets go to `~/.local/share/rlax-theme`; user configuration goes to `~/.config/waybar`, `~/.config/mako`, and as the separate `~/.config/hypr/rlax-hyprlock.conf` file.

Use `--component` to select components. Components can be repeated:

```bash
./scripts/install.sh --component launcher --component font
./scripts/install.sh --component osd
./scripts/install.sh --all                    # includes SDDM files in user data
./scripts/install.sh --dry-run                # show requested operations only
./scripts/install.sh --prefix "$HOME/.local"  # explicit user install prefix
```

The `--all` SDDM option **does not** install to `/usr/share/sddm/themes`, select the theme, or alter your display manager. SDDM is system-wide and distribution-specific: manually copy the installed `share/rlax-theme/sddm/rlax-gashat` directory to the SDDM themes directory with administrator privileges, then select it in the SDDM configuration. Keep a recovery route and a known-working greeter before changing login-manager settings; a broken greeter can prevent graphical login.

The installer backs up existing destination files under `$XDG_DATA_HOME/rlax-theme/backups/` (normally `~/.local/share/rlax-theme/backups/`) and records installed paths in a manifest. Re-running updates the files and keeps prior backups. It does not invoke `sudo`.

## Hyprland integration (manual, non-destructive)

The installer intentionally does not edit `hyprland.conf`, Lua modules, keybinds, autostart, monitor, or input configuration. Your `~/.local/bin` must be on Hyprland's `PATH` for commands below.

### Optional visual settings

Review `~/.local/share/rlax-theme/hypr/rlax-theme.conf`, then include it from your Hyprland configuration if you want its gaps, borders, shadow, blur, and animation defaults:

```ini
source = ~/.local/share/rlax-theme/hypr/rlax-theme.conf
```

This is a sample appearance block; Hyprland may report duplicate settings if your existing config defines the same options. In that case, merge only the individual desired settings into your own appearance section rather than including the entire snippet.

### Launcher, power menu, and OSD keybinds

Choose free shortcuts and add only the corresponding commands to your existing keybind configuration. Do not replace your keybind file. Example Hyprland `hyprland.conf` syntax:

```ini
bind = SUPER, R, exec, rlax-launcher
bind = SUPER SHIFT, Q, exec, rlax-power-menu
bindel = , XF86AudioRaiseVolume, exec, osd-volume.sh 5%+
bindel = , XF86AudioLowerVolume, exec, osd-volume.sh 5%-
bindl = , XF86AudioMute, exec, osd-volume.sh toggle
bindel = , XF86MonBrightnessUp, exec, osd-brightness.sh 3%+
bindel = , XF86MonBrightnessDown, exec, osd-brightness.sh 3%-
```

If your Hyprland configuration is Lua-based, add equivalent `hl.bind(...)` entries to your own keybind module. Check that shortcuts are unused first. You can also test the applications directly from a terminal before binding them.

### Waybar, Mako and lock screen

The Waybar install adapts the repository example: it removes the machine-specific extra disk and hwmon sensor path, and uses the installed power-menu executable. Review the config for your machine (terminal names, network tools, audio controls and modules vary), then restart Waybar. Restart Mako after its configuration changes:

```bash
pkill waybar; waybar &
makoctl reload
```

The hyprlock theme is installed as `~/.config/hypr/rlax-hyprlock.conf`. Test it manually with `hyprlock --config ~/.config/hypr/rlax-hyprlock.conf` before changing any lock key or idle handler. To make it the default, adjust only your existing lock command after confirming the test works. Keep a second terminal/session available when testing lock-screen changes.

## Uninstall and rollback

Run from the clone:

```bash
./scripts/uninstall.sh
```

The uninstaller removes files listed in the installer manifest and restores their pre-install copies where available. Backups are retained under `~/.local/share/rlax-theme/backups/` for manual recovery. It does not remove anything manually copied into system SDDM directories. Review the manifest before uninstalling if you have since customized installed Waybar/Mako or Hyprland Lua files; restoring a backup replaces those files.

## Build, test, and run from the clone

```bash
cargo build --release --locked -p rlax-launcher
cargo build --release --locked -p rlax-power-menu
cargo test --workspace --locked
cargo run -p rlax-launcher
cargo run -p rlax-power-menu
```

## Repository layout

```text
apps/                 GTK4 layer-shell launcher and power menu
assets/               Artwork and Press Start 2P font/license
configs/hypr/         Optional theme and hyprlock snippets
configs/mako/         Mako theme
configs/sddm/         Optional SDDM greeter theme
configs/waybar/       Waybar example theme
crates/               Shared UI, Hyprland and security helpers
scripts/              Installer, uninstaller and OSD helpers
docs/SECURITY.md      Security notes
```

## Notes

- OSD keybindings explicitly adjust the setting and then render/send a notification. They do not install a background service.
- The power-menu countdown closes the menu at zero; it does not trigger a power action automatically.
- SDDM installation is intentionally not automated because it is system-wide and can affect the ability to log in.
- `scripts/revert-mighty.sh` and the old `scripts/hypr-bind.sh` were removed: they depended on a dated backup path or repository-specific absolute paths.