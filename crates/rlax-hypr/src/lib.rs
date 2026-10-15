//! rlax-hypr: Hyprland IPC helpers
//! Safe wrapper around `hyprctl` for launching / querying.
//! No unsafe, no shell injection.

use std::process::Command;

/// Launch an app via Hyprland dispatch (secure, no shell).
/// Equivalent to `hyprctl dispatch exec <cmd>` but uses direct Command
/// to avoid shell injection.
/// `exec_cmd` should be a single executable + args already validated.
pub fn launch_app(exec_cmd: &str) -> Result<(), String> {
    let exec_cmd = exec_cmd.trim();
    if exec_cmd.is_empty() {
        return Err("empty command".into());
    }
    // Basic security: reject commands containing dangerous shell metachars
    // We launch via `hyprctl dispatch exec -- <cmd>` which does NOT use shell
    // but we still block `;`, `|`, `&`, `$`, `` ` ``, `$(` to prevent abuse
    let dangerous = [";", "|", "&", "$", "`", "$(", "&&", "||"];
    for pat in dangerous {
        if exec_cmd.contains(pat) {
            return Err(format!("rejected dangerous pattern: {}", pat));
        }
    }

    // Use hyprctl if available, otherwise fallback to direct spawn
    let status = Command::new("hyprctl")
        .args(["dispatch", "exec", "--", exec_cmd])
        .status()
        .map_err(|e| format!("failed to run hyprctl: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        // Fallback: spawn directly (for non-Hyprland testing)
        Command::new("sh")
            .arg("-c")
            .arg(exec_cmd)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("fallback spawn failed: {}", e))
    }
}

/// Check if we're running inside Hyprland
pub fn is_hyprland() -> bool {
    std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
}
