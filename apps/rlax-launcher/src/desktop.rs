use std::fs;
use std::path::Path;

/// Single app entry parsed from .desktop file
#[derive(Clone, Debug)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub comment: Option<String>,
}

impl AppEntry {
    pub fn matches(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let q = query.to_lowercase();
        self.name.to_lowercase().contains(&q)
            || self.exec.to_lowercase().contains(&q)
            || self
                .comment
                .as_ref()
                .map(|c| c.to_lowercase().contains(&q))
                .unwrap_or(false)
    }
}

/// Parse all .desktop files from standard locations
pub fn load_apps() -> Vec<AppEntry> {
    let mut apps = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let mut dirs = vec![
        "/usr/share/applications".to_string(),
        "/usr/local/share/applications".to_string(),
    ];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(format!("{}/.local/share/applications", home));
    }
    // XDG_DATA_DIRS
    if let Ok(xdg) = std::env::var("XDG_DATA_DIRS") {
        for p in xdg.split(':') {
            dirs.push(format!("{}/applications", p));
        }
    }

    for dir in dirs {
        let path = Path::new(&dir);
        if !path.is_dir() {
            continue;
        }
        let Ok(entries) = fs::read_dir(path) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("desktop") {
                continue;
            }
            if let Some(app) = parse_desktop_file(&p) {
                // dedup by name+exec
                let key = format!("{}|{}", app.name, app.exec);
                if seen.insert(key) {
                    apps.push(app);
                }
            }
        }
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn parse_desktop_file(path: &Path) -> Option<AppEntry> {
    let content = fs::read_to_string(path).ok()?;
    let mut name: Option<String> = None;
    let mut exec: Option<String> = None;
    let mut icon: Option<String> = None;
    let mut comment: Option<String> = None;
    let mut no_display = false;
    let mut hidden = false;
    let mut in_desktop_entry = false;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_desktop_entry {
            continue;
        }
        if line.starts_with("NoDisplay=true") {
            no_display = true;
        }
        if line.starts_with("Hidden=true") {
            hidden = true;
        }
        if let Some(v) = line.strip_prefix("Name=") {
            // only first Name (without locale)
            if name.is_none() {
                name = Some(v.trim().to_string());
            }
        }
        if let Some(v) = line.strip_prefix("Exec=") {
            if exec.is_none() {
                exec = Some(v.trim().to_string());
            }
        }
        if let Some(v) = line.strip_prefix("Icon=") {
            if icon.is_none() {
                icon = Some(v.trim().to_string());
            }
        }
        if let Some(v) = line.strip_prefix("Comment=") {
            if comment.is_none() {
                comment = Some(v.trim().to_string());
            }
        }
    }

    if no_display || hidden {
        return None;
    }
    let name = name?;
    let exec = exec?;
    if name.is_empty() || exec.is_empty() {
        return None;
    }
    // filter out obviously invalid
    if !rlax_security::is_safe_exec(&exec) {
        return None;
    }
    Some(AppEntry {
        name,
        exec,
        icon,
        comment,
    })
}
