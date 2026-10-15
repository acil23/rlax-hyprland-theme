# Rlax Custom Apps — Arch + Hyprland (Kamen Rider Ex-Aid Theme)

Monorepo aplikasi custom ringan, aman, dan Hyprland-native. Dibangun dengan **Rust + GTK4 + gtk4-layer-shell** — terinspirasi *Kamen Rider Ex-Aid Mighty Action X Gashat Ring*.

![Ex-Aid](assets/exaid.png)

## ✨ Apa yang sudah dikerjakan sejauh ini

### Phase 0 — Foundation
- Workspace Cargo (`resolver = "2"`, Rust 1.75, edition 2021, MIT)
- `crates/rlax-ui` — design system shared (CSS provider, warna mighty: #FF1493 / #00FFFF / #1A002A)
- `crates/rlax-hypr` — wrapper `hyprctl dispatch exec --` anti shell-injection
- `crates/rlax-security` — `sanitize_query` (max 128 char, strip control char), `strip_field_codes`, `is_safe_exec`
- `docs/SECURITY.md` — prinsip secure-by-default, memory-safety, Wayland isolation

### Phase 1 — rlax-launcher (DONE, epic!)
Launcher pengganti rofi/wofi, Wayland-native layer-shell:
- Scan `*.desktop` via `load_apps()` + fuzzy filter `AppEntry::matches`
- **UI Gashat Ring**: 9 kartu melingkar elips mengitari Ex-Aid (bukan bar lurus), perspektif 3D — depan `182×232` paling besar & tajam, belakang `122×154` kecil + `blur(2.5px) + desaturate` (depth-of-field)
- **Ex-Aid center** `assets/exaid.png` (432×578 RGBA transparan) 410×540 Contain — `Picture` di `Fixed`, bukan background
- Neon horizon ring `720×130` + `560×90` di kaki Ex-Aid (glow ungu/cyan)
- Search `860px` centered `"🎮 GASHAT SELECT > Type to Henshin..."` + header `CHOOSE APPLICATION >`
- Navigasi `←/→/↑/↓/Tab`, scroll, klik card (offset `rem_euclid`), `Enter` → animasi `Select! 620ms` → `rlax_hypr::launch_app` (tanpa shell)
- **Iterasi desain epic** (dari feedback): hapus border kotak → fullscreen → centered `1460×740` overlay (layer-shell `anchor=false`), window rounded `18px` + glow, kartu transparan `0.38–0.92`, naik sejajar panah `◀ ▶` (Y 210–312), stage `rgba(5,0,14,0.96)`
- Build `56M`, `cargo check 0.2s`, `153 apps loaded`

## Stack
- Rust + GTK4 0.11 + gtk4-layer-shell 0.8 + gdk4/pango/glib/gio
- Hyprland (Wayland), Arch Linux

## Struktur
```
apps/rlax-launcher    # launcher — DONE
crates/rlax-ui        # theme epic fullscreen borderless
crates/rlax-hypr      # hyprctl aman
crates/rlax-security  # sanitasi
assets/exaid.png      # 432×578 Ex-Aid transparan (center)
```

## Cara Jalan
```bash
sudo pacman -S gtk4 gtk4-layer-shell
cargo run -p rlax-launcher      # dev
./target/debug/rlax-launcher    # release
```

### Hyprland bind (SUPER+R)
```ini
bind = SUPER, R, exec, /home/aklein/Data/Rlax/customApp/target/debug/rlax-launcher
```

## Security
- `#![deny(unsafe_code)]` sebisa mungkin, `cargo audit`/`cargo deny` di CI (roadmap)
- Semua `Exec` desktop divalidasi, field code `%f/%F/%u/%U` di-strip
- Launch tanpa shell: `hyprctl dispatch exec -- <sanitized>`

## Roadmap
- [x] rlax-launcher Gashat Ring epic
- [ ] rlax-clip (clipboard + grim/slurp)
- [ ] rlax-files
- [ ] rlax-monitor

---
*Gashapon! Henshin! — Level Up!*
