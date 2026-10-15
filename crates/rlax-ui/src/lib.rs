//! rlax-ui EPIC FULLSCREEN borderless
use gtk4::CssProvider;
use gtk4::gdk::Display;
pub fn apply_rlax_theme() {
 let css = r#"
  window { background: transparent; border: none; }
  box.root { background: transparent; }
  box.stage {
    background:
      radial-gradient(ellipse at 50% 34%, rgba(160,32,255,0.20) 0%, transparent 55%),
      radial-gradient(ellipse at 50% 82%, rgba(255,20,147,0.15) 0%, transparent 60%),
      linear-gradient(180deg, rgba(0,255,255,0.03) 0%, transparent 45%),
      rgba(5,0,14,0.96);
    border-radius: 18px;
    border: 1px solid rgba(255,20,147,0.22);
    box-shadow: 0 22px 64px rgba(0,0,0,0.78), 0 0 0 1px rgba(168,80,255,0.10), 0 0 40px rgba(168,80,255,0.16);
    margin: 10px;
  }
  label.henshin-title { font-family: "JetBrainsMono Nerd Font", monospace; font-weight: 900; font-size: 18px; color: #ffffff; letter-spacing: 4px; }
  label.henshin-sub { font-family: "JetBrainsMono Nerd Font", monospace; color: #7fa0b8; font-size: 9px; letter-spacing: 3px; }
  entry.search-entry {
    background: rgba(14,0,28,0.92);
    border: 2px solid rgba(0,255,255,0.9);
    border-radius: 6px;
    padding: 12px 16px;
    font-family: "JetBrainsMono Nerd Font", monospace;
    font-size: 13px; font-weight: 800; color: #ffffff; caret-color: #39FF14;
    box-shadow: 0 0 14px rgba(0,255,255,0.22), inset 0 0 10px rgba(0,255,255,0.06);
  }
  entry.search-entry:focus { border-color: #FF1493; box-shadow: 0 0 18px rgba(255,20,147,0.55), inset 0 0 12px rgba(255,20,147,0.08); }
  entry.search-entry placeholder { color: #6b6b88; }
  box.neon-ring {
    background: transparent;
    border: 3px solid rgba(168,80,255,0.95);
    border-radius: 50%;
    box-shadow: 0 0 22px rgba(168,80,255,0.95), 0 0 48px rgba(168,80,255,0.45), inset 0 0 24px rgba(168,80,255,0.35);
  }
  box.neon-ring-inner {
    background: transparent;
    border: 2px solid rgba(0,255,255,0.55);
    border-radius: 50%;
  }
  button.ring-arrow {
    background: rgba(14,0,28,0.88);
    border: 2px solid rgba(255,20,147,0.7);
    border-radius: 8px;
    color: #FF1493; font-size: 18px; font-weight: 900;
    min-width: 42px; min-height: 72px; padding: 0;
    box-shadow: 0 0 12px rgba(255,20,147,0.35);
  }
  button.ring-arrow:hover { background: #FF1493; color: #000; box-shadow: 0 0 18px rgba(255,20,147,0.85); }
  button.gashat-card {
    background: linear-gradient(180deg, rgba(14,14,42,0.72) 0%, rgba(8,8,28,0.78) 100%);
    border: 2px solid rgba(0,255,255,0.88);
    border-radius: 14px;
    padding: 12px 8px;
    box-shadow: 0 0 16px rgba(0,255,255,0.30), inset 0 0 14px rgba(0,255,255,0.07), 0 10px 28px rgba(0,0,0,0.55);
  }
  button.gashat-card:hover { border-color: #00FFFF; background: linear-gradient(180deg, #14144a 0%, #0c0c2e 100%); box-shadow: 0 0 20px rgba(0,255,255,0.5), 0 10px 28px rgba(0,0,0,0.7); }
  button.gashat-card.selected {
    background: linear-gradient(180deg, rgba(28,10,52,0.88) 0%, rgba(18,6,36,0.92) 100%);
    border: 3px solid #FF1493;
    border-radius: 14px;
    box-shadow: 0 0 0 2px rgba(0,255,255,0.75), 0 0 30px rgba(255,20,147,0.95), 0 0 48px rgba(255,20,147,0.45), 0 14px 36px rgba(0,0,0,0.75), inset 0 0 18px rgba(255,20,147,0.12);
  }
  /* blur + desaturate yang di belakang Ex-Aid — makin belakang makin kabur */
  button.gashat-card.gashat-back  { opacity: 0.38; filter: blur(2.5px) saturate(0.60) brightness(0.78); }
  button.gashat-card.gashat-mid   { opacity: 0.62; filter: blur(1.0px) saturate(0.82) brightness(0.88); }
  button.gashat-card.gashat-near  { opacity: 0.92; filter: blur(0px); }
  /* keep old names as aliases */
  button.gashat-card.dimmed { opacity: 0.52; }
  button.gashat-card.semi { opacity: 0.82; }
  button.gashat-card.ghost { opacity: 0.38; filter: blur(2.5px) saturate(0.60) brightness(0.78); }
  label.card-name { font-family: "JetBrainsMono Nerd Font", monospace; font-weight: 900; font-size: 11px; color: #eef0ff; }
  button.gashat-card.selected label.card-name { color: #ffffff; font-size: 12px; }
  label.card-exec { font-family: "JetBrainsMono Nerd Font", monospace; font-size: 7px; color: #8a8ab0; }
  label.player-label { color: #FF6bb3; font-size: 11px; font-weight: 900; letter-spacing: 3px; text-shadow: 0 0 8px rgba(255,20,147,0.7); }
  label.choose-label { color: #00FFFF; font-size: 10px; font-weight: 900; letter-spacing: 2px; background: rgba(0,0,0,0.62); padding: 6px 12px; border: 1px solid rgba(0,255,255,0.45); border-radius: 6px; }
  label.dot { color: rgba(255,255,255,0.22); font-size: 9px; }
  label.dot.active { color: #FF1493; }
  box.select-popup {
    background: rgba(6,0,14,0.94);
    border: 3px solid #FFFF00;
    border-radius: 12px;
    padding: 28px 40px;
    box-shadow: 0 0 0 2px rgba(255,20,147,0.45), 0 0 36px rgba(255,255,0,0.55), 0 20px 40px rgba(0,0,0,0.7);
  }
  label.select-label { font-family: "JetBrainsMono Nerd Font", monospace; font-weight: 900; font-size: 48px; color: #FFFF00; letter-spacing: 3px; }
  label.select-sub { font-size: 12px; color: #00FFFF; letter-spacing: 4px; font-weight: 800; }
  image.select-icon { -gtk-icon-size: 64px; }
  image.app-icon { -gtk-icon-size: 36px; }
  label.icon-badge { min-width: 36px; min-height: 36px; background: #FF1493; color: #000; font-weight: 900; font-size: 15px; border-radius: 4px; }
 "#;
 let pr = CssProvider::new(); pr.load_from_string(css);
 if let Some(d) = Display::default() { gtk4::style_context_add_provider_for_display(&d, &pr, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION); }
}
pub fn mighty_colors() -> (&'static str,&'static str,&'static str) { ("#FF1493","#00FFFF","#1A002A") }
