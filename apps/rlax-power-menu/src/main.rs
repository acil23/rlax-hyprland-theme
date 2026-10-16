use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Label, Orientation,
    Overlay,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::Cell;
use std::process::Command;
use std::rc::Rc;

const APP_ID: &str = "tech.aklein.rlax-power-menu";
const DEFAULT_BACKGROUND: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/power_menu.jpeg");
const COUNTDOWN_SECONDS: u32 = 9;
const ACTION_ROW_TOP: i32 = 430;

#[derive(Clone, Copy)]
enum Action {
    Reboot,
    Shutdown,
    Sleep,
    Lock,
    Logout,
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn background_path() -> String {
    if let Ok(path) = std::env::var("RLAX_POWER_MENU_BACKGROUND") {
        return path;
    }
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(".local/share")));
    data_home.map(|p| p.join("rlax-theme/assets/power_menu.jpeg"))
        .filter(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| DEFAULT_BACKGROUND.to_owned())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("RLAX POWER MENU")
        .decorated(false)
        .resizable(false)
        .default_width(1460)
        .default_height(820)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_namespace(Some("rlax-power-menu"));
    for edge in [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
        window.set_anchor(edge, false);
        window.set_margin(edge, 0);
    }
    window.set_exclusive_zone(0);

    let overlay = Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_vexpand(true);

    let background = gtk4::Picture::for_filename(background_path());
    background.set_content_fit(gtk4::ContentFit::Cover);
    background.set_can_shrink(true);
    background.set_hexpand(true);
    background.set_vexpand(true);
    overlay.set_child(Some(&background));

    let shade = GtkBox::new(Orientation::Vertical, 0);
    shade.add_css_class("power-shade");
    shade.set_hexpand(true);
    shade.set_vexpand(true);
    overlay.add_overlay(&shade);

    let root = GtkBox::new(Orientation::Vertical, 16);
    root.add_css_class("power-root");
    root.set_halign(gtk4::Align::Fill);
    root.set_valign(gtk4::Align::Fill);
    root.set_hexpand(true);
    root.set_vexpand(true);
    root.set_margin_start(50);
    root.set_margin_end(50);
    root.set_margin_top(24);
    root.set_margin_bottom(22);
    shade.append(&root);

    let top = GtkBox::new(Orientation::Horizontal, 0);
    top.add_css_class("power-topline");
    let stage = Label::new(Some("P1  LEVEL 99  //  STAGE: POWER MENU"));
    stage.add_css_class("power-stage");
    stage.set_halign(gtk4::Align::Start);
    let clock = Label::new(None);
    clock.add_css_class("power-clock");
    clock.set_halign(gtk4::Align::End);
    clock.set_hexpand(true);
    top.append(&stage);
    top.append(&clock);
    root.append(&top);

    let title = Label::new(Some("CONTINUE?"));
    title.add_css_class("power-title");
    title.set_halign(gtk4::Align::Center);
    title.set_margin_top(10);
    root.append(&title);

    let countdown = Label::new(Some(&format!("{:02}", COUNTDOWN_SECONDS)));
    countdown.add_css_class("power-countdown");
    countdown.set_halign(gtk4::Align::Center);
    root.append(&countdown);

    let hint = Label::new(Some("INSERT GASHAT OR SELECT ACTION"));
    hint.add_css_class("power-hint");
    hint.set_halign(gtk4::Align::Center);
    hint.set_margin_bottom(12);
    root.append(&hint);

    let actions = GtkBox::new(Orientation::Horizontal, 14);
    actions.add_css_class("power-actions");
    actions.set_halign(gtk4::Align::Center);
    actions.set_homogeneous(true);

    let selected = Rc::new(Cell::new(false));
    let specs = [
        (
            "⟳",
            "REBOOT",
            "CONTINUE? (INSERT GASHAT)",
            Action::Reboot,
            "reboot",
        ),
        ("☠", "SHUTDOWN", "GAME OVER", Action::Shutdown, "shutdown"),
        ("Ⅱ", "SLEEP", "PAUSE GAME", Action::Sleep, "sleep"),
        ("♙", "LOCK", "SAVE GAME", Action::Lock, "lock"),
        ("⇥", "LOG OUT", "EXIT PLAYER", Action::Logout, "logout"),
    ];
    for (index, (icon_text, label_text, sublabel, action, class)) in specs.into_iter().enumerate() {
        let button = Button::new();
        button.add_css_class("power-action");
        button.add_css_class(class);
        button.set_hexpand(true);
        button.set_vexpand(false);
        button.set_size_request(0, 132);
        button.set_tooltip_text(Some(sublabel));
        button.set_widget_name(&format!("power-action-{index}"));
        let button_content = GtkBox::new(Orientation::Vertical, 3);
        button_content.set_halign(gtk4::Align::Center);
        button_content.set_valign(gtk4::Align::Center);
        let icon = Label::new(Some(icon_text));
        icon.add_css_class("power-action-icon");
        let label = Label::new(Some(label_text));
        label.add_css_class("power-action-label");
        button_content.append(&icon);
        button_content.append(&label);
        button.set_child(Some(&button_content));
        let window = window.clone();
        let selected = selected.clone();
        button.connect_clicked(move |_| {
            if selected.replace(true) {
                return;
            }
            run_action(action, &window);
        });
        actions.append(&button);
    }
    // Position independently from the vertical content column. A vexpand
    // spacer there used to pin the buttons to the bottom regardless of margin.
    actions.set_valign(gtk4::Align::Start);
    actions.set_margin_top(ACTION_ROW_TOP);
    overlay.add_overlay(&actions);

    let bottom = GtkBox::new(Orientation::Horizontal, 0);
    bottom.add_css_class("power-bottomline");
    bottom.set_margin_top(8);
    let footer = Label::new(Some("◀  CHOOSE YOUR FATE  ▶"));
    footer.add_css_class("power-footer");
    footer.set_halign(gtk4::Align::Center);
    bottom.append(&footer);
    root.append(&bottom);

    install_css();
    window.set_child(Some(&overlay));

    let clock_for_timer = clock.clone();
    glib::timeout_add_seconds_local(1, move || {
        clock_for_timer.set_text(&format!("◷ TIME: {}", chrono_time()));
        glib::ControlFlow::Continue
    });
    clock.set_text(&format!("◷ TIME: {}", chrono_time()));

    let remaining = Rc::new(Cell::new(COUNTDOWN_SECONDS));
    let countdown_window = window.clone();
    let countdown_label = countdown.clone();
    let countdown_selected = selected.clone();
    glib::timeout_add_seconds_local(1, move || {
        if countdown_selected.get() {
            return glib::ControlFlow::Break;
        }
        let next = remaining.get().saturating_sub(1);
        remaining.set(next);
        countdown_label.set_text(&format!("{next:02}"));
        if next == 0 {
            countdown_window.close();
            glib::ControlFlow::Break
        } else {
            glib::ControlFlow::Continue
        }
    });

    let key_controller = EventControllerKey::new();
    let key_window = window.clone();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            key_window.close();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    window.add_controller(key_controller);

    window.present();
}

fn chrono_time() -> String {
    // Read local wall-clock time without introducing another crate.
    let output = Command::new("date").arg("+%H:%M:%S").output();
    output
        .ok()
        .filter(|result| result.status.success())
        .map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned())
        .unwrap_or_else(|| "--:--:--".to_owned())
}

fn action_command(action: Action) -> (&'static str, Vec<&'static str>) {
    let command = match action {
        Action::Reboot => ("systemctl", vec!["reboot"]),
        Action::Shutdown => ("systemctl", vec!["poweroff"]),
        Action::Sleep => ("systemctl", vec!["suspend"]),
        Action::Lock => ("hyprlock", vec![]),
        Action::Logout => ("hyprctl", vec!["dispatch", "exit"]),
    };
    command
}

fn run_action(action: Action, window: &ApplicationWindow) {
    let command = action_command(action);
    match Command::new(command.0).args(command.1).spawn() {
        Ok(_) => window.close(),
        Err(error) => eprintln!("[rlax-power-menu] failed to run {}: {error}", command.0),
    }
}

#[cfg(test)]
mod action_tests {
    use super::{action_command, Action};

    #[test]
    fn actions_map_to_expected_explicit_commands() {
        assert_eq!(
            action_command(Action::Reboot),
            ("systemctl", vec!["reboot"])
        );
        assert_eq!(
            action_command(Action::Shutdown),
            ("systemctl", vec!["poweroff"])
        );
        assert_eq!(
            action_command(Action::Sleep),
            ("systemctl", vec!["suspend"])
        );
        assert_eq!(action_command(Action::Lock), ("hyprlock", vec![]));
        assert_eq!(
            action_command(Action::Logout),
            ("hyprctl", vec!["dispatch", "exit"])
        );
    }
}

fn install_css() {
    let css = gtk4::CssProvider::new();
    css.load_from_data(
        r#"
        window { background: #030309; }
        .power-shade { background: rgba(0, 0, 0, 0.18); }
        .power-root { color: #f4f4ff; }
        .power-topline { border-bottom: 2px solid rgba(255, 42, 145, 0.8); padding: 0 10px 8px; }
        .power-stage { color: #ffe94a; font-family: "Press Start 2P", monospace; font-size: 15px; font-weight: 900; }
        .power-clock { color: #fff4a6; font-family: "Press Start 2P", monospace; font-size: 13px; font-weight: 800; }
        .power-title { color: #ffe94a; font-family: "Press Start 2P", monospace; font-size: 44px; font-weight: 900; text-shadow: 0 0 16px #ff285e, 2px 2px #b90049; }
        .power-countdown { color: #ffe94a; font-family: "Press Start 2P", monospace; font-size: 92px; font-weight: 900; text-shadow: 0 0 22px #ff174f; }
        .power-hint { color: #ffffff; font-family: "Press Start 2P", monospace; font-size: 13px; font-weight: 800; }
        .power-actions { margin: 0 6px; }
        button.power-action {
            min-width: 120px; min-height: 104px; padding: 10px 8px;
            border: 3px solid; border-radius: 9px; background: rgba(4, 7, 15, 0.9);
            color: #fff; font-family: "Press Start 2P", monospace; font-weight: 800;
            box-shadow: 0 0 18px currentColor, inset 0 0 15px rgba(255,255,255,0.07);
        }
        label.power-action-icon { font-family: "Iosevka Nerd Font", monospace; font-size: 42px; font-weight: 1000; }
        label.power-action-label { font-family: "Press Start 2P", monospace; font-size: 13px; font-weight: 800; }
        button.power-action:hover, button.power-action:focus { background: rgba(30, 24, 48, 0.96); border-width: 4px; }
        button.power-action:active { background: rgba(65, 42, 75, 0.98); }
        button.reboot { color: #77ff8b; border-color: #55ff70; }
        button.shutdown { color: #ff597a; border-color: #ff315c; }
        button.sleep { color: #ffe678; border-color: #ffd447; }
        button.lock { color: #57f7f3; border-color: #22ecea; }
        button.logout { color: #e99aff; border-color: #d47aff; }
        .power-bottomline { border-top: 5px dashed #ff315c; padding-top: 10px; }
        .power-footer { color: #fff; font-family: "Press Start 2P", monospace; font-size: 13px; font-weight: 800; }
        "#,
    );
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &css,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::COUNTDOWN_SECONDS;

    #[test]
    fn countdown_is_nine_seconds_and_expires_at_zero() {
        let values: Vec<u32> = (0..=COUNTDOWN_SECONDS).rev().collect();
        assert_eq!(values, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        assert_eq!(values.last(), Some(&0));
    }
}
