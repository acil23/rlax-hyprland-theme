mod desktop;
use desktop::{AppEntry, UsageStore, load_apps};
use gdk4::Key;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Fixed, Image, Label, Orientation, Overlay, Picture, SearchEntry};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use rlax_security::{sanitize_query, strip_field_codes};
const APP_ID: &str = "tech.aklein.rlax-launcher";
const ASSET: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/exaid.png");
fn installed_asset() -> String {
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(".local/share")));
    data_home.map(|p| p.join("rlax-theme/assets/exaid.png"))
        .filter(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| ASSET.to_owned())
}
fn main(){ let app=Application::builder().application_id(APP_ID).build(); app.connect_activate(build_ui); app.run_with_args::<String>(&[]); }
fn exists(p:&str)->bool{ std::path::Path::new(p).exists() }
fn build_ui(app:&Application){
 let usage_store=UsageStore::load_default();
 let mut apps:Vec<AppEntry>=load_apps();
 usage_store.sort_apps(&mut apps);
 eprintln!("[rlax-launcher] loaded {} apps (ranked by launch frequency)", apps.len()); rlax_ui::apply_rlax_theme();
 let window=ApplicationWindow::builder().application(app).title("RLAX HENSHIN").decorated(false).resizable(false).build();
 // Centered epic — NOT fullscreen (user: terlalu fullscreen). Fixed 1460x740 centered overlay
 window.set_default_size(1460, 740); window.set_size_request(1460, 740);
 window.init_layer_shell(); window.set_layer(Layer::Overlay); window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
 window.set_namespace(Some("rlax-launcher"));
 // No edge anchor = layer-shell centers the window (fixes pojok kiri atas bug)
 window.set_anchor(Edge::Top,false); window.set_anchor(Edge::Bottom,false); window.set_anchor(Edge::Left,false); window.set_anchor(Edge::Right,false);
 window.set_exclusive_zone(0); window.set_margin(Edge::Top,0); window.set_margin(Edge::Bottom,0); window.set_margin(Edge::Left,0); window.set_margin(Edge::Right,0);
 let overlay=Overlay::new(); overlay.set_vexpand(true); overlay.set_hexpand(true);
 let root=GtkBox::new(Orientation::Vertical,0); root.add_css_class("root"); root.set_vexpand(true); root.set_hexpand(true);
 let stage=GtkBox::new(Orientation::Vertical,0); stage.add_css_class("stage"); stage.set_vexpand(true); stage.set_hexpand(true);
 let header=GtkBox::new(Orientation::Vertical,4); header.set_halign(gtk4::Align::Center); header.set_margin_top(18);
 let title=Label::new(Some("CHOOSE APPLICATION  >")); title.add_css_class("henshin-title"); header.append(&title);
 let sub=Label::new(Some("GASHAT RING  //  TYPE TO SEARCH  //  \u{25C0} \u{25B6} TO SELECT")); sub.add_css_class("henshin-sub"); header.append(&sub);
 let search_wrap=GtkBox::new(Orientation::Horizontal,0); search_wrap.set_halign(gtk4::Align::Center); search_wrap.set_margin_top(14); search_wrap.set_margin_bottom(6);
 let search=SearchEntry::builder().placeholder_text("\u{1F3AE}  GASHAT SELECT  >  Type to Henshin...").hexpand(false).build(); search.add_css_class("search-entry"); search.set_size_request(860, -1);
 search_wrap.append(&search);
 let center_wrap=GtkBox::new(Orientation::Horizontal,0); center_wrap.set_halign(gtk4::Align::Center); center_wrap.set_vexpand(true); center_wrap.set_hexpand(true);
 let fixed=Fixed::new(); fixed.set_size_request(1400, 620); fixed.set_hexpand(false); fixed.set_vexpand(false); fixed.set_halign(gtk4::Align::Center); fixed.set_valign(gtk4::Align::Center);
 // neon horizon tetap di kaki Ex-Aid (jangan ikut naik)
 let ring_outer=GtkBox::new(Orientation::Vertical,0); ring_outer.add_css_class("neon-ring"); ring_outer.set_size_request(720, 130); fixed.put(&ring_outer,340.0,415.0);
 let ring_inner=GtkBox::new(Orientation::Vertical,0); ring_inner.add_css_class("neon-ring-inner"); ring_inner.set_size_request(560, 90); fixed.put(&ring_inner,420.0,435.0);
 let asset = installed_asset();
 let pic: gtk4::Widget = if exists(&asset) { let pp=Picture::for_filename(&asset); pp.set_content_fit(gtk4::ContentFit::Contain); pp.set_size_request(410, 540); pp.set_can_shrink(true); pp.upcast() } else { let l=Label::new(Some("EX-AID")); l.add_css_class("henshin-title"); l.upcast() };
 fixed.put(&pic,495.0,-10.0);
 let btn_left=Button::with_label("\u{25C0}"); btn_left.add_css_class("ring-arrow"); btn_left.set_focusable(false); btn_left.set_size_request(44,78); fixed.put(&btn_left,8.0,232.0);
 let btn_right=Button::with_label("\u{25B6}"); btn_right.add_css_class("ring-arrow"); btn_right.set_focusable(false); btn_right.set_size_request(44,78); fixed.put(&btn_right,1348.0,232.0);
 // 9-card GASHAT RING — ELIPSE DI PINGGANG, SEJAJAR PANAH (bukan di bawah kaki). Depan besar, belakang kecil+blur
 let positions:[(f64,f64,i32,i32);9]=[(502.0,210.0,122,154),(104.0,228.0,134,170),(-8.0,262.0,152,190),(202.0,295.0,170,212),(609.0,312.0,182,232),(1023.0,295.0,170,212),(1258.0,262.0,152,190),(1154.0,228.0,134,170),(770.0,210.0,122,154)];
 let mut cards:Vec<Button>=Vec::new();
 for (x,y,w,h) in positions { let b=Button::new(); b.add_css_class("gashat-card"); b.set_focusable(false); b.set_size_request(w,h); fixed.put(&b,x,y); cards.push(b); }
 let player_lbl=Label::new(Some("PLAYER 1: SELECT")); player_lbl.add_css_class("player-label"); fixed.put(&player_lbl,626.0,552.0);
 let choose_lbl=Label::new(Some("\u{25B6}  CHOOSE APP  \u{25C0}")); choose_lbl.add_css_class("choose-label"); fixed.put(&choose_lbl,14.0,340.0);
 center_wrap.append(&fixed);
 let dots_bar=GtkBox::new(Orientation::Horizontal,10); dots_bar.set_halign(gtk4::Align::Center); dots_bar.set_margin_bottom(14);
 let dots_label=Label::new(Some("\u{25CF} \u{25CB} \u{25CB} \u{25CB} \u{25CB}")); dots_label.add_css_class("dot");
 let count_label=Label::new(Some("")); count_label.add_css_class("card-exec"); dots_bar.append(&dots_label); dots_bar.append(&count_label);
 stage.append(&header); stage.append(&search_wrap); stage.append(&center_wrap); stage.append(&dots_bar);
 root.append(&stage);

 let select_box=GtkBox::new(Orientation::Vertical,8); select_box.add_css_class("select-popup"); select_box.set_halign(gtk4::Align::Center); select_box.set_valign(gtk4::Align::Center); select_box.set_visible(false);
 let select_icon=Image::from_icon_name("emblem-ok-symbolic"); select_icon.add_css_class("select-icon"); select_icon.set_pixel_size(64);
 let select_label=Label::new(Some("Select!")); select_label.add_css_class("select-label");
 let select_sub=Label::new(Some("\u{25B6}  GASHAT HENSHIN!  \u{25C0}")); select_sub.add_css_class("select-sub");
 let select_app=Label::new(Some("")); select_app.add_css_class("card-exec");
 select_box.append(&select_icon); select_box.append(&select_label); select_box.append(&select_sub); select_box.append(&select_app); overlay.add_overlay(&select_box);
 window.set_child(Some(&overlay));
 use std::cell::{Cell,RefCell}; use std::rc::Rc;
 let apps_rc=Rc::new(apps); let usage=Rc::new(RefCell::new(usage_store)); let filtered:Rc<RefCell<Vec<AppEntry>>>=Rc::new(RefCell::new(Vec::new())); let selected:Rc<Cell<usize>>=Rc::new(Cell::new(0)); let selecting:Rc<Cell<bool>>=Rc::new(Cell::new(false));
 let build_card=|btn:&Button, entry:Option<&AppEntry>, is_sel:bool, dim:&str|{
  if let Some(ch)=btn.child(){ btn.set_child(None::<&gtk4::Widget>); let _=ch; }
  btn.remove_css_class("selected"); btn.remove_css_class("dimmed"); btn.remove_css_class("semi"); btn.remove_css_class("ghost");
  btn.remove_css_class("gashat-back"); btn.remove_css_class("gashat-mid"); btn.remove_css_class("gashat-near");
  if !dim.is_empty(){ btn.add_css_class(dim); } if is_sel{ btn.add_css_class("selected"); }
  if let Some(app)=entry{
   let inner=GtkBox::new(Orientation::Vertical,6); inner.set_halign(gtk4::Align::Center); inner.set_valign(gtk4::Align::Center);
   let icon:gtk4::Widget=if let Some(n)=&app.icon{ let im=Image::from_icon_name(n.as_str()); im.set_pixel_size(if is_sel{40}else{32}); im.add_css_class("app-icon"); im.upcast()} else { let l=Label::new(Some(&app.name.chars().next().unwrap_or('?').to_string())); l.add_css_class("icon-badge"); l.upcast()};
   let name_lbl=Label::new(Some(&app.name)); name_lbl.add_css_class("card-name"); name_lbl.set_wrap(true); name_lbl.set_justify(gtk4::Justification::Center); name_lbl.set_max_width_chars(12); name_lbl.set_ellipsize(pango::EllipsizeMode::End); name_lbl.set_lines(2);
   let c=app.comment.as_deref().unwrap_or("").to_string(); let s=if c.len()>26{format!("{}..",&c[..24])}else{c};
   let exec_lbl=Label::new(Some(&s)); exec_lbl.add_css_class("card-exec"); exec_lbl.set_max_width_chars(14); exec_lbl.set_ellipsize(pango::EllipsizeMode::End);
   let ib=GtkBox::new(Orientation::Vertical,0); ib.set_halign(gtk4::Align::Center); ib.append(&icon);
   inner.append(&ib); inner.append(&name_lbl); if !exec_lbl.text().is_empty(){ inner.append(&exec_lbl); }
   btn.set_child(Some(&inner)); btn.set_sensitive(true); btn.set_tooltip_text(Some(&app.name));
  } else { let l=Label::new(Some("\u{2014}")); l.add_css_class("card-exec"); btn.set_child(Some(&l)); btn.set_sensitive(false); }
 };


 let render_ring={
  let filtered=filtered.clone(); let selected=selected.clone(); let cards=cards.clone(); let dots_label=dots_label.clone(); let count_label=count_label.clone(); let build_card=build_card.clone();
  Rc::new(move||{
   let list=filtered.borrow(); let n=list.len();
   if n==0{ for(i,b) in cards.iter().enumerate(){ build_card(b,None,false,if i==4{"semi"}else{"ghost"});} dots_label.set_text("\u{2014} NO GASHAT \u{2014}"); dots_label.remove_css_class("active"); count_label.set_text("0 apps"); return; }
   let sel=selected.get().min(n-1);
   for(slot,btn) in cards.iter().enumerate(){ let off=slot as i32 -4; let idx=(sel as i32+off).rem_euclid(n as i32) as usize; let is=slot==4; let dim=if is{""}else if slot==3||slot==5{"gashat-near"}else if slot==2||slot==6{"gashat-mid"}else{"gashat-back"}; build_card(btn,list.get(idx),is,dim); }
   let mut dots=String::new(); for i in 0..n.min(10){ if i==(sel%10){dots.push_str("\u{25CF} ");} else {dots.push_str("\u{25CB} ");} } if n>10{dots.push_str("\u{2026}");}
   dots_label.set_text(dots.trim()); dots_label.add_css_class("active"); count_label.set_text(&format!("{} / {}  \u{2014}  [{}/{}]",sel+1,n,sel+1,n));
  })
 };
 let do_select={
  let filtered=filtered.clone(); let selected=selected.clone(); let window=window.clone(); let selecting=selecting.clone(); let select_box=select_box.clone(); let select_app=select_app.clone(); let select_icon=select_icon.clone(); let usage=usage.clone();
  move||{
   if selecting.get(){return;} let list=filtered.borrow(); if list.is_empty(){return;} let sel=selected.get().min(list.len()-1); let Some(e)=list.get(sel).cloned() else {return;};
   selecting.set(true); select_app.set_text(&e.name); if let Some(ic)=&e.icon{ select_icon.set_icon_name(Some(ic.as_str())); } select_box.set_visible(true);
   let w2=window.clone(); let e2=e.clone(); let usage2=usage.clone(); glib::timeout_add_local_once(std::time::Duration::from_millis(620), move||{ let exec=strip_field_codes(&e2.exec); eprintln!("[rlax-launcher] HENSHIN! {} -> {}", e2.name, exec); match rlax_hypr::launch_app(&exec){ Ok(_)=>{ if let Err(er)=usage2.borrow_mut().record_launch(&e2){ eprintln!("[rlax-launcher] could not save usage history: {er}"); } w2.close(); }, Err(er)=>{eprintln!("[rlax] launch failed: {}",er); w2.close();}} });
  }
 };
 let populate={
  let apps_rc=apps_rc.clone(); let filtered=filtered.clone(); let selected=selected.clone(); let render_ring=render_ring.clone();
  move|q:&str|{ let qq=sanitize_query(q); let m:Vec<AppEntry>=apps_rc.iter().filter(|a| a.matches(&qq)).take(120).cloned().collect(); *filtered.borrow_mut()=m; selected.set(0); render_ring(); }
 };
 populate(""); { let p=populate.clone(); search.connect_search_changed(move|e| p(e.text().as_str())); }
 { let d=do_select.clone(); search.connect_activate(move|_| d()); }
 let go_prev={ let f=filtered.clone(); let s=selected.clone(); let r=render_ring.clone(); move||{ let n=f.borrow().len(); if n==0{return;} s.set(if s.get()==0{n-1}else{s.get()-1}); r(); }};
 let go_next={ let f=filtered.clone(); let s=selected.clone(); let r=render_ring.clone(); move||{ let n=f.borrow().len(); if n==0{return;} s.set((s.get()+1)%n); r(); }};
 { let g=go_prev.clone(); btn_left.connect_clicked(move|_| g()); }
 { let g=go_next.clone(); btn_right.connect_clicked(move|_| g()); }
 for(slot,btn) in cards.iter().enumerate(){
  let ds=do_select.clone(); let f=filtered.clone(); let s=selected.clone(); let r=render_ring.clone();
  btn.connect_clicked(move|_|{ if slot==4{ ds(); return; } let off=slot as i32-4; let n=f.borrow().len(); if n==0{return;} s.set((s.get() as i32+off).rem_euclid(n as i32) as usize); r(); });
 }
 let kc=gtk4::EventControllerKey::new(); kc.set_propagation_phase(gtk4::PropagationPhase::Capture);
 { let w=window.clone(); let gp=go_prev.clone(); let gn=go_next.clone(); let ds=do_select.clone(); kc.connect_key_pressed(move|_,kv,_,_|{ match kv{ k if k==Key::Escape=>{w.close(); return true.into();}, k if k==Key::Return||k==Key::KP_Enter||k==Key::ISO_Enter=>{ds(); return true.into();}, k if k==Key::Left||k==Key::Up=>{gp(); return true.into();}, k if k==Key::Right||k==Key::Down||k==Key::Tab||k==Key::ISO_Left_Tab=>{gn(); return true.into();}, _=>{} } false.into()}); }
 window.add_controller(kc);
 let scroll=gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
 { let gp=go_prev.clone(); let gn=go_next.clone(); scroll.connect_scroll(move|_,_,dy|{ if dy<0.0{gp();} else if dy>0.0{gn();} true.into()}); } fixed.add_controller(scroll);
 window.present(); let s2=search.clone(); glib::idle_add_local_once(move||{ s2.grab_focus(); });

 overlay.set_child(Some(&root));
}

