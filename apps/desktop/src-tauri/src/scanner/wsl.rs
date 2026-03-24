use std::path::Path;
use std::process::Command;

/// Resolve the actual Windows username from WSL.
/// Tries cmd.exe first, then scans /mnt/c/Users/ for real user directories.
pub fn resolve_windows_username() -> String {
    if let Some(name) = resolve_via_cmd() {
        return name;
    }
    if let Some(name) = resolve_via_user_dirs() {
        return name;
    }
    // Last resort: assume WSL username matches Windows
    std::env::var("USER").unwrap_or_default()
}

fn resolve_via_cmd() -> Option<String> {
    // Redirect stderr to suppress UNC path warnings from cmd.exe in WSL
    let output = Command::new("cmd.exe")
        .args(["/C", "echo", "%USERNAME%"])
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    let raw = String::from_utf8_lossy(&output.stdout);
    // cmd.exe may print UNC warnings before the result; take last non-empty line
    let name = raw
        .lines()
        .rev()
        .map(|l| l.trim())
        .find(|l| !l.is_empty() && !l.contains('%'))?
        .to_string();
    if name.is_empty() {
        return None;
    }
    Some(name)
}

const SYSTEM_USER_DIRS: &[&str] = &[
    "All Users",
    "Default",
    "Default User",
    "Public",
    "TEMP",
    "desktop.ini",
];

fn resolve_via_user_dirs() -> Option<String> {
    let users_dir = Path::new("/mnt/c/Users");
    let entries = std::fs::read_dir(users_dir).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if SYSTEM_USER_DIRS.contains(&name.as_str()) {
            continue;
        }
        if name.starts_with("UMFD-") || name.contains("Font Driver") {
            continue;
        }
        let path = entry.path();
        if path.is_dir() && path.join("AppData").is_dir() {
            return Some(name);
        }
    }
    None
}
