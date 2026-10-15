//! rlax-security: Security helpers
//! Central place for input validation & safe helpers.

/// Validate & sanitize search query: trim, limit length, strip control chars.
pub fn sanitize_query(input: &str) -> String {
    let trimmed = input.trim();
    // limit to 128 chars to prevent DoS via huge strings
    let limited: String = trimmed.chars().take(128).collect();
    // strip control characters
    limited
        .chars()
        .filter(|c| !c.is_control())
        .collect()
}

/// Validate desktop Exec field is safe to launch.
/// Rejects field codes that could be abused if not expanded safely.
pub fn is_safe_exec(exec: &str) -> bool {
    // .desktop Exec may contain %f %F %u %U etc.
    // We strip them before launch; but reject obviously dangerous
    if exec.trim().is_empty() {
        return false;
    }
    // Block shell injection attempts in desktop files (rare but check)
    let bad = [";", "|", "&", "`", "$("];
    !bad.iter().any(|p| exec.contains(p))
}

/// Strip .desktop field codes (%f, %F, %u, %U, %i, %c, %k, etc.)
pub fn strip_field_codes(exec: &str) -> String {
    let mut out = String::new();
    let mut chars = exec.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            // skip next char (field code)
            chars.next();
        } else {
            out.push(c);
        }
    }
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sanitize() {
        assert_eq!(sanitize_query("  hello \n "), "hello");
        assert_eq!(sanitize_query(&"a".repeat(200)).len(), 128);
    }
    #[test]
    fn test_strip() {
        assert_eq!(strip_field_codes("firefox %u"), "firefox");
        assert_eq!(strip_field_codes("code --new-window %F"), "code --new-window");
    }
}
