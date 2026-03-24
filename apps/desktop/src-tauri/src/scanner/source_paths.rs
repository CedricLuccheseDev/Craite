use super::wsl::resolve_windows_username;
use std::path::Path;

/// Resolved platform paths for template substitution
pub struct PlatformPaths {
    pub home: String,
    pub mnt: String,
    pub appdata: String,
    pub localappdata: String,
    pub programdata: String,
    pub programfiles: String,
}

impl PlatformPaths {
    pub fn resolve() -> Self {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_default()
            .replace('\\', "/");

        let is_wsl = home.starts_with("/home/") && Path::new("/mnt/c").exists();

        let mnt = if is_wsl {
            let win_user = resolve_windows_username();
            format!("/mnt/c/Users/{win_user}")
        } else {
            String::new()
        };

        let (appdata, localappdata, programdata, programfiles) = if is_wsl {
            (
                format!("{mnt}/AppData/Roaming"),
                format!("{mnt}/AppData/Local"),
                "/mnt/c/ProgramData".into(),
                "/mnt/c/Program Files".into(),
            )
        } else {
            (
                std::env::var("APPDATA")
                    .unwrap_or_default()
                    .replace('\\', "/"),
                std::env::var("LOCALAPPDATA")
                    .unwrap_or_default()
                    .replace('\\', "/"),
                "C:/ProgramData".into(),
                "C:/Program Files".into(),
            )
        };

        Self {
            home,
            mnt,
            appdata,
            localappdata,
            programdata,
            programfiles,
        }
    }

    pub fn expand(&self, template: &str) -> String {
        template
            .replace("{home}", &self.home)
            .replace("{mnt}", &self.mnt)
            .replace("{appdata}", &self.appdata)
            .replace("{localappdata}", &self.localappdata)
            .replace("{programdata}", &self.programdata)
            .replace("{programfiles}", &self.programfiles)
    }

    /// Detect all known sample directories that exist on this system
    pub fn detect_sources(&self) -> Vec<(String, String)> {
        let mut dirs = Vec::new();

        for (template, label) in ALL_CANDIDATES {
            let path = self.expand(template);
            if !path.is_empty() && Path::new(&path).is_dir() {
                dirs.push((path, label.to_string()));
            }
        }

        dirs
    }
}

/// All known sample/library directory candidates across platforms.
/// Templates: {home}, {mnt}, {appdata}, {localappdata}, {programdata}, {programfiles}
const ALL_CANDIDATES: &[(&str, &str)] = &[
    // ── Splice ──
    ("{home}/Splice/sounds", "Splice"),
    ("{mnt}/Splice/sounds", "Splice"),
    ("{home}/Documents/Splice/Samples", "Splice"),
    ("{mnt}/Documents/Splice/Samples", "Splice"),
    // ── FL Studio ──
    (
        "{programfiles}/Image-Line/FL Studio 20/Data/Patches/Packs",
        "FL Studio 20",
    ),
    (
        "{programfiles}/Image-Line/FL Studio 21/Data/Patches/Packs",
        "FL Studio 21",
    ),
    (
        "{programfiles}/Image-Line/FL Studio 24/Data/Patches/Packs",
        "FL Studio 24",
    ),
    ("{home}/Documents/Image-Line/FL Studio", "FL Studio User"),
    ("{mnt}/Documents/Image-Line/FL Studio", "FL Studio User"),
    // ── Ableton Live ──
    (
        "{home}/Documents/Ableton/User Library",
        "Ableton User Library",
    ),
    (
        "{home}/Documents/Ableton/Factory Packs",
        "Ableton Factory Packs",
    ),
    ("{home}/Music/Ableton/User Library", "Ableton User Library"),
    (
        "{home}/Music/Ableton/Factory Packs",
        "Ableton Factory Packs",
    ),
    (
        "{mnt}/Documents/Ableton/User Library",
        "Ableton User Library",
    ),
    (
        "{mnt}/Documents/Ableton/Factory Packs",
        "Ableton Factory Packs",
    ),
    // ── Logic Pro (macOS) ──
    ("/Library/Audio/Apple Loops/Apple", "Apple Loops"),
    (
        "{home}/Library/Audio/Apple Loops/User Loops",
        "User Apple Loops",
    ),
    (
        "/Library/Application Support/Logic/Sampler Instruments",
        "Logic Sampler",
    ),
    (
        "/Library/Audio/Impulse Responses/Apple",
        "Logic Impulse Responses",
    ),
    // ── Native Instruments ──
    ("{home}/Documents/Native Instruments", "Native Instruments"),
    ("{mnt}/Documents/Native Instruments", "Native Instruments"),
    ("/Users/Shared", "NI Shared Content"),
    ("C:/Users/Public/Documents", "NI Public Content"),
    ("{mnt}/../Public/Documents", "NI Public Content"),
    // ── Serum (Xfer) ──
    ("{home}/Documents/Xfer/Serum Presets", "Serum"),
    ("{mnt}/Documents/Xfer/Serum Presets", "Serum"),
    ("/Library/Audio/Presets/Xfer Records/Serum Presets", "Serum"),
    // ── Reason Studios ──
    (
        "{programdata}/Propellerhead Software/Soundbanks",
        "Reason Soundbanks",
    ),
    (
        "/Library/Application Support/Propellerhead Software/Soundbanks",
        "Reason Soundbanks",
    ),
    ("{home}/Music/Reason Studios/ReFills", "Reason ReFills"),
    // ── Bitwig Studio ──
    (
        "{localappdata}/Bitwig Studio/installed-packages",
        "Bitwig Packages",
    ),
    (
        "{home}/Library/Application Support/Bitwig/Bitwig Studio/installed-packages",
        "Bitwig Packages",
    ),
    ("{home}/.BitwigStudio/installed-packages", "Bitwig Packages"),
    ("{home}/Documents/Bitwig Studio", "Bitwig Studio"),
    ("{mnt}/Documents/Bitwig Studio", "Bitwig Studio"),
    // ── Studio One (PreSonus) ──
    (
        "{home}/Documents/Studio One/Sound Sets",
        "Studio One Sound Sets",
    ),
    (
        "{mnt}/Documents/Studio One/Sound Sets",
        "Studio One Sound Sets",
    ),
    ("{home}/Documents/Studio One/Presets", "Studio One Presets"),
    // ── Cubase / Steinberg ──
    (
        "{programdata}/Steinberg/Content/VST Sound",
        "Steinberg VST Sound",
    ),
    (
        "/Library/Application Support/Steinberg/Content/VST Sound",
        "Steinberg VST Sound",
    ),
    (
        "{appdata}/Steinberg/Content/VST Sound",
        "Steinberg User Content",
    ),
    // ── REAPER ──
    ("{home}/Documents/REAPER Media", "REAPER Media"),
    ("{mnt}/Documents/REAPER Media", "REAPER Media"),
    // ── Renoise ──
    ("{home}/Documents/Renoise", "Renoise"),
    ("{home}/Renoise", "Renoise"),
    // ── Output Arcade ──
    ("{programdata}/Output/Arcade", "Arcade"),
    ("/Library/Application Support/Output/Arcade", "Arcade"),
    // ── Loopcloud ──
    ("{appdata}/Loopcloud/library", "Loopcloud"),
    ("{home}/Library/Loopcloud/library", "Loopcloud"),
    // ── Pro Tools ──
    (
        "{programfiles}/Common Files/Avid/Audio/Plug-Ins",
        "Pro Tools Plugins",
    ),
    (
        "/Library/Application Support/Avid/Audio/Plug-Ins",
        "Pro Tools Plugins",
    ),
    // ── iZotope ──
    ("{home}/Documents/iZotope", "iZotope"),
    ("{mnt}/Documents/iZotope", "iZotope"),
    // ── Cymatics ──
    ("{programdata}/Cymatics", "Cymatics"),
    ("/Library/Audio/Presets/Cymatics", "Cymatics"),
    // ── Common user locations ──
    ("{home}/Documents/Samples", "Documents/Samples"),
    ("{home}/Music/Samples", "Music/Samples"),
    ("{home}/Desktop/Samples", "Desktop/Samples"),
    ("{home}/Downloads/Samples", "Downloads/Samples"),
    ("{mnt}/Documents/Samples", "Documents/Samples"),
    ("{mnt}/Music/Samples", "Music/Samples"),
    ("{mnt}/Desktop/Samples", "Desktop/Samples"),
    ("{mnt}/Downloads/Samples", "Downloads/Samples"),
];
