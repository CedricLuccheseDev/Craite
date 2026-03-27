use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::scanner::source_paths::PlatformPaths;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DawInfo {
    pub id: String,
    pub name: String,
    pub detected: bool,
    pub library_path: String,
}

/// DAW definition used to drive detection from data instead of code.
struct DawDef {
    id: &'static str,
    name: &'static str,
    /// Candidates to check for existence (detection)
    candidates: fn(&PlatformPaths) -> Vec<String>,
    /// Candidates for the CrAIte library folder
    library_candidates: fn(&PlatformPaths) -> Vec<String>,
    /// Fallback library path if no candidate exists
    library_fallback: fn(&PlatformPaths) -> String,
}

fn any_exists(candidates: &[String]) -> bool {
    candidates.iter().any(|p| Path::new(p).exists())
}

fn pick_first_existing(candidates: &[String]) -> Option<String> {
    candidates
        .iter()
        .find(|p| Path::new(p.as_str()).exists())
        .cloned()
}

/// Detect all registered DAWs using data-driven definitions.
pub fn detect_daws() -> Vec<DawInfo> {
    let p = PlatformPaths::resolve();

    DAWS.iter()
        .map(|d| {
            let candidates = (d.candidates)(&p);
            let lib_candidates = (d.library_candidates)(&p);
            let library_path = pick_first_existing(&lib_candidates)
                .map(|dir| format!("{dir}/CrAIte"))
                .unwrap_or_else(|| (d.library_fallback)(&p));

            DawInfo {
                id: d.id.into(),
                name: d.name.into(),
                detected: any_exists(&candidates),
                library_path,
            }
        })
        .collect()
}

const DAWS: &[DawDef] = &[
    DawDef {
        id: "fl_studio",
        name: "FL Studio",
        candidates: |p| {
            vec![
                format!("{}/Documents/Image-Line/FL Studio", p.home),
                format!("{}/Documents/Image-Line/FL Studio", p.mnt),
                format!("{}/Image-Line/FL Studio 24", p.programfiles),
                format!("{}/Image-Line/FL Studio 21", p.programfiles),
                format!("{}/Image-Line/FL Studio 20", p.programfiles),
                "/Applications/FL Studio.app".into(),
            ]
        },
        library_candidates: |p| {
            vec![
                format!("{}/Documents/Image-Line/FL Studio", p.mnt),
                format!("{}/Documents/Image-Line/FL Studio", p.home),
            ]
        },
        library_fallback: |p| format!("{}/Documents/Image-Line/FL Studio/Packs/CrAIte", p.mnt),
    },
    DawDef {
        id: "ableton",
        name: "Ableton Live",
        candidates: |p| {
            vec![
                format!("{}/Music/Ableton", p.mnt),
                format!("{}/Music/Ableton", p.home),
                format!("{}/Documents/Ableton", p.home),
                "/Applications/Ableton Live 12 Suite.app".into(),
            ]
        },
        library_candidates: |p| {
            vec![
                format!("{}/Music/Ableton/User Library/Samples", p.mnt),
                format!("{}/Music/Ableton/User Library/Samples", p.home),
                format!("{}/Documents/Ableton/User Library/Samples", p.home),
            ]
        },
        library_fallback: |p| format!("{}/Music/Ableton/User Library/Samples/CrAIte", p.mnt),
    },
    DawDef {
        id: "bitwig",
        name: "Bitwig Studio",
        candidates: |p| {
            vec![
                format!("{}/Documents/Bitwig Studio", p.mnt),
                format!("{}/Documents/Bitwig Studio", p.home),
                format!("{}/Bitwig Studio", p.home),
                format!("{}/.BitwigStudio", p.home),
            ]
        },
        library_candidates: |p| {
            vec![
                format!("{}/Documents/Bitwig Studio/Library/Samples", p.mnt),
                format!("{}/Documents/Bitwig Studio/Library/Samples", p.home),
            ]
        },
        library_fallback: |p| format!("{}/Documents/Bitwig Studio/Library/Samples/CrAIte", p.mnt),
    },
    DawDef {
        id: "studio_one",
        name: "Studio One",
        candidates: |p| {
            vec![
                format!("{}/Documents/Studio One", p.mnt),
                format!("{}/Documents/Studio One", p.home),
            ]
        },
        library_candidates: |p| {
            vec![
                format!("{}/Documents/Studio One/User Content", p.mnt),
                format!("{}/Documents/Studio One/User Content", p.home),
            ]
        },
        library_fallback: |p| format!("{}/Documents/Studio One/User Content/CrAIte", p.mnt),
    },
    DawDef {
        id: "logic",
        name: "Logic Pro",
        candidates: |p| {
            vec![
                format!("{}/Music/Logic", p.home),
                "/Applications/Logic Pro.app".into(),
            ]
        },
        library_candidates: |_| vec![],
        library_fallback: |p| format!("{}/Music/Audio Music Apps/Samples/CrAIte", p.home),
    },
    DawDef {
        id: "reaper",
        name: "REAPER",
        candidates: |p| {
            vec![
                format!("{}/AppData/Roaming/REAPER", p.mnt),
                format!("{}/.config/REAPER", p.home),
                format!("{}/Library/Application Support/REAPER", p.home),
            ]
        },
        library_candidates: |p| {
            vec![
                format!("{}/Documents/REAPER Media", p.mnt),
                format!("{}/Documents/REAPER Media", p.home),
            ]
        },
        library_fallback: |p| format!("{}/Documents/REAPER Media/CrAIte", p.mnt),
    },
    DawDef {
        id: "cubase",
        name: "Cubase",
        candidates: |p| {
            vec![
                format!("{}/Steinberg/Cubase", p.appdata),
                format!("{}/Steinberg/Cubase 14", p.programfiles),
                format!("{}/Steinberg/Cubase 13", p.programfiles),
                format!("{}/Steinberg/Cubase 12", p.programfiles),
                "/Applications/Cubase 14.app".into(),
                "/Applications/Cubase 13.app".into(),
            ]
        },
        library_candidates: |p| {
            vec![
                format!("{}/Steinberg/Content", p.appdata),
                format!("{}/Documents/Steinberg", p.home),
            ]
        },
        library_fallback: |p| format!("{}/Documents/Steinberg/CrAIte", p.home),
    },
    DawDef {
        id: "reason",
        name: "Reason",
        candidates: |p| {
            vec![
                format!("{}/Propellerhead Software/Soundbanks", p.programdata),
                "/Library/Application Support/Propellerhead Software/Soundbanks".into(),
                format!("{}/Music/Reason Studios", p.home),
            ]
        },
        library_candidates: |_| vec![],
        library_fallback: |p| format!("{}/Music/Reason Studios/CrAIte", p.home),
    },
    DawDef {
        id: "protools",
        name: "Pro Tools",
        candidates: |p| {
            vec![
                format!("{}/Avid/Pro Tools", p.programfiles),
                format!("{}/Avid/Pro Tools", p.appdata),
                "/Applications/Pro Tools.app".into(),
            ]
        },
        library_candidates: |_| vec![],
        library_fallback: |p| format!("{}/Documents/Pro Tools/CrAIte", p.home),
    },
    DawDef {
        id: "renoise",
        name: "Renoise",
        candidates: |p| {
            vec![
                format!("{}/Documents/Renoise", p.home),
                format!("{}/Renoise", p.home),
                format!("{}/.renoise", p.home),
            ]
        },
        library_candidates: |_| vec![],
        library_fallback: |p| format!("{}/Documents/Renoise/CrAIte", p.home),
    },
];
