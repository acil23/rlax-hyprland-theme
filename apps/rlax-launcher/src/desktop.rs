use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Single app entry parsed from .desktop file
#[derive(Clone, Debug)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub comment: Option<String>,
}

impl AppEntry {
    pub fn usage_key(&self) -> String {
        format!("{}\u{1f}{}", self.name, self.exec)
    }

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

/// Persistent, per-user launch counts. The file contains only hex-encoded
/// app identity keys and integer counts; no application arguments or queries.
#[derive(Debug)]
pub struct UsageStore {
    path: PathBuf,
    counts: HashMap<String, u64>,
}

impl UsageStore {
    pub fn load_default() -> Self {
        let base = std::env::var_os("XDG_STATE_HOME")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state"))
            })
            .unwrap_or_else(|| std::env::temp_dir().join("rlax-launcher-state"));
        Self::load(base.join("rlax-launcher/usage.tsv"))
    }

    fn load(path: PathBuf) -> Self {
        let mut counts = HashMap::new();
        if let Ok(contents) = fs::read_to_string(&path) {
            for line in contents.lines() {
                let Some((count, encoded_key)) = line.split_once('\t') else {
                    continue;
                };
                let (Ok(count), Some(key)) = (count.parse::<u64>(), decode_hex(encoded_key)) else {
                    continue;
                };
                counts.insert(key, count);
            }
        }
        Self { path, counts }
    }

    pub fn count(&self, app: &AppEntry) -> u64 {
        self.counts.get(&app.usage_key()).copied().unwrap_or(0)
    }

    /// Increment the in-memory count, then atomically persist it. Even if disk
    /// writing fails, this launch remains counted for the current session.
    pub fn record_launch(&mut self, app: &AppEntry) -> std::io::Result<()> {
        let key = app.usage_key();
        let count = self.counts.entry(key).or_insert(0);
        *count = count.saturating_add(1);
        self.persist()
    }

    pub fn sort_apps(&self, apps: &mut [AppEntry]) {
        apps.sort_by(|a, b| {
            self.count(b)
                .cmp(&self.count(a))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
    }

    fn persist(&self) -> std::io::Result<()> {
        let parent = self.path.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "usage file has no parent directory",
            )
        })?;
        fs::create_dir_all(parent)?;

        let mut entries: Vec<_> = self.counts.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        let mut contents = String::new();
        for (key, count) in entries {
            contents.push_str(&format!("{}\t{}\n", count, encode_hex(key.as_bytes())));
        }

        let temp_path = self
            .path
            .with_extension(format!("tsv.{}.tmp", std::process::id()));
        {
            let mut file = fs::File::create(&temp_path)?;
            file.write_all(contents.as_bytes())?;
            file.sync_all()?;
        }
        fs::rename(temp_path, &self.path)
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push(HEX[(byte >> 4) as usize] as char);
        result.push(HEX[(byte & 0x0f) as usize] as char);
    }
    result
}

fn decode_hex(value: &str) -> Option<String> {
    if value.len() % 2 != 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks_exact(2) {
        let high = (pair[0] as char).to_digit(16)? as u8;
        let low = (pair[1] as char).to_digit(16)? as u8;
        bytes.push((high << 4) | low);
    }
    String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod usage_tests {
    use super::{AppEntry, UsageStore};
    use std::path::PathBuf;

    fn app(name: &str, exec: &str) -> AppEntry {
        AppEntry {
            name: name.into(),
            exec: exec.into(),
            icon: None,
            comment: None,
        }
    }

    #[test]
    fn launch_counts_persist_and_are_specific_to_app() {
        let path = std::env::temp_dir().join(format!("rlax-usage-test-{}.tsv", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let firefox = app("Firefox", "firefox");
        let browser = app("Browser", "firefox");
        let mut usage = UsageStore::load(path.clone());
        usage.record_launch(&firefox).unwrap();
        usage.record_launch(&firefox).unwrap();

        let restored = UsageStore::load(path.clone());
        assert_eq!(restored.count(&firefox), 2);
        assert_eq!(restored.count(&browser), 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn malformed_usage_lines_are_ignored() {
        let path: PathBuf =
            std::env::temp_dir().join(format!("rlax-usage-bad-{}.tsv", std::process::id()));
        std::fs::write(&path, "not-a-count\t66697265666f78\n2\tbad-hex\n").unwrap();
        let usage = UsageStore::load(path.clone());
        assert_eq!(usage.count(&app("Firefox", "firefox")), 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn apps_sort_by_frequency_then_alphabetically() {
        let path = std::env::temp_dir().join(format!("rlax-usage-sort-{}.tsv", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let alpha = app("Alpha", "alpha");
        let beta = app("Beta", "beta");
        let gamma = app("Gamma", "gamma");
        let mut usage = UsageStore::load(path.clone());
        usage.record_launch(&beta).unwrap();
        usage.record_launch(&gamma).unwrap();
        usage.record_launch(&gamma).unwrap();

        let mut apps = vec![gamma, beta, alpha];
        usage.sort_apps(&mut apps);
        assert_eq!(
            apps.iter().map(|app| app.name.as_str()).collect::<Vec<_>>(),
            ["Gamma", "Beta", "Alpha"]
        );
        std::fs::remove_file(path).unwrap();
    }
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
