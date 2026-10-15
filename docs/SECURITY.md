# SECURITY.md — Rlax Custom Apps

Prinsip: **Secure by default, least privilege**.

1. Memory safety: Rust + `#![deny(unsafe_code)]` di crates sebisa mungkin.
2. Input validation: semua input user lewat `rlax-security::sanitize_query`.
3. No shell: launch app via `hyprctl dispatch exec -- <cmd>` tanpa shell.
4. Desktop file: `Exec` di-validate `is_safe_exec()` dan `strip_field_codes()`.
5. Wayland isolation: tidak pakai X11, manfaatkan isolasi Wayland.
6. Secrets: nanti pakai `libsecret` / `oo7` (gnome-keyring), jangan plaintext.
7. Audit: `cargo audit` dan `cargo deny` di CI.

## Checklist tiap app baru
- [ ] Validasi semua input luar
- [ ] Tidak ada `unsafe` tanpa review
- [ ] Tidak jalan sebagai root
- [ ] Dependency minimal
