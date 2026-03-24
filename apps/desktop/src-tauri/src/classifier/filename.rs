use super::engine::rule_engine;
use super::rules::RULES;
use std::path::Path;

/// Classification result for a single file
pub struct Classification {
    pub category: String,
    pub subcategory: String,
    pub confidence: f32,
    /// Duration in seconds (0.0 if not yet analysed)
    pub duration: f32,
    /// Sample rate in Hz (0 if not yet analysed)
    pub sample_rate: u32,
}

/// Normalize a path segment for keyword matching:
/// - lowercase
/// - all non-alphanumeric chars → space (removes underscores, hyphens, dots, digits as separators)
/// - padded with a leading and trailing space so rules can use `" hh "` for word-boundary matching
fn normalize(s: &str) -> String {
    let body: String = s
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { ' ' })
        .collect();
    format!(" {body} ")
}

/// Classify a sample based on its filename and up to two parent directory names.
///
/// Match priority (descending confidence):
///   1. filename stem          → 0.85
///   2. immediate parent dir   → 0.65
///   3. grandparent dir        → 0.50
///
/// The normalized text uses word-boundary padding so short keywords like `" hh "`
/// only match the standalone token, not substrings of longer words.
pub fn classify_by_filename(path: &Path) -> Classification {
    let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let norm_filename = normalize(filename);

    if let Some((rule_idx, _)) = rule_engine().find_match(&norm_filename) {
        let rule = &RULES[rule_idx];
        return Classification {
            category: rule.category.to_string(),
            subcategory: extract_subcategory(&norm_filename, rule.category),
            confidence: 0.85,
            duration: 0.0,
            sample_rate: 0,
        };
    }

    // Try immediate parent directory
    let parent_name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let norm_parent = normalize(parent_name);

    if let Some((rule_idx, _)) = rule_engine().find_match(&norm_parent) {
        let rule = &RULES[rule_idx];
        return Classification {
            category: rule.category.to_string(),
            subcategory: String::new(),
            confidence: 0.65,
            duration: 0.0,
            sample_rate: 0,
        };
    }

    // Try grandparent directory
    let grandparent_name = path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let norm_grandparent = normalize(grandparent_name);

    if let Some((rule_idx, _)) = rule_engine().find_match(&norm_grandparent) {
        let rule = &RULES[rule_idx];
        return Classification {
            category: rule.category.to_string(),
            subcategory: String::new(),
            confidence: 0.50,
            duration: 0.0,
            sample_rate: 0,
        };
    }

    Classification {
        category: "unknown".to_string(),
        subcategory: String::new(),
        confidence: 0.0,
        duration: 0.0,
        sample_rate: 0,
    }
}

/// Extract a more specific subcategory from the normalized filename.
/// Only called when a category match was found in the filename itself (not parent dirs).
fn extract_subcategory(norm_filename: &str, category: &str) -> String {
    match category {
        "kick" => {
            if norm_filename.contains("808") {
                "808".into()
            } else if norm_filename.contains("layered") {
                "layered".into()
            } else if norm_filename.contains("acoustic") {
                "acoustic".into()
            } else if norm_filename.contains("sub") {
                "sub".into()
            } else {
                String::new()
            }
        }
        "snare" => {
            if norm_filename.contains("rimshot") {
                "rimshot".into()
            } else if norm_filename.contains("acoustic") {
                "acoustic".into()
            } else if norm_filename.contains("lofi") || norm_filename.contains("lo fi") {
                "lo-fi".into()
            } else if norm_filename.contains("fat") {
                "fat".into()
            } else {
                String::new()
            }
        }
        "hihat" => {
            if norm_filename.contains("open") {
                "open".into()
            } else if norm_filename.contains("closed") {
                "closed".into()
            } else if norm_filename.contains("pedal") {
                "pedal".into()
            } else {
                String::new()
            }
        }
        "bass" => {
            if norm_filename.contains("808") {
                "808".into()
            } else if norm_filename.contains("sub") {
                "sub".into()
            } else if norm_filename.contains("reese") {
                "reese".into()
            } else {
                String::new()
            }
        }
        "pad" => {
            if norm_filename.contains("dark") {
                "dark".into()
            } else if norm_filename.contains("warm") {
                "warm".into()
            } else if norm_filename.contains("ambient") {
                "ambient".into()
            } else {
                String::new()
            }
        }
        "fx" => {
            if norm_filename.contains("riser") {
                "riser".into()
            } else if norm_filename.contains("impact") {
                "impact".into()
            } else if norm_filename.contains("sweep") {
                "sweep".into()
            } else if norm_filename.contains("whoosh") {
                "whoosh".into()
            } else if norm_filename.contains("reverse") {
                "reverse".into()
            } else {
                String::new()
            }
        }
        "loop" => {
            if norm_filename.contains("drum") {
                "drum".into()
            } else if norm_filename.contains("melody") || norm_filename.contains("melodic") {
                "melody".into()
            } else if norm_filename.contains("bass") {
                "bass".into()
            } else if norm_filename.contains("full") {
                "full".into()
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_classify_kick() {
        let path = PathBuf::from("/samples/drums/kick_heavy.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "kick");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_snare() {
        let path = PathBuf::from("/samples/drums/snare_acoustic.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "snare");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_hihat_open() {
        let path = PathBuf::from("/samples/hats/hihat_open_01.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "hihat");
        assert_eq!(result.subcategory, "open");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_hihat_closed() {
        let path = PathBuf::from("/samples/hats/hihat_closed_02.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "hihat");
        assert_eq!(result.subcategory, "closed");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_bass_808() {
        let path = PathBuf::from("/samples/bass/bass_808_long.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "bass");
        assert_eq!(result.subcategory, "808");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_bass_sub() {
        let path = PathBuf::from("/samples/bass/sub_bass.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "bass");
        assert_eq!(result.subcategory, "sub");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_unknown() {
        let path = PathBuf::from("/samples/random/file123.xyz");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "unknown");
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_classify_with_parent_directory() {
        // "heavy_01" doesn't match, but parent "kicks" contains "kick"
        let path = PathBuf::from("/samples/kicks/heavy_01.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "kick");
        assert_eq!(result.confidence, 0.65);
    }

    #[test]
    fn test_classify_with_grandparent_directory() {
        // Neither filename nor parent match, but grandparent "Snares" does
        let path = PathBuf::from("/Splice/Snares/Acoustic/01.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "snare");
        assert_eq!(result.confidence, 0.50);
    }

    #[test]
    fn test_no_false_positive_hh_in_word() {
        // "pshh" should NOT match hihat — " hh " is not in " pshh 01 "
        let path = PathBuf::from("/samples/random/pshh_01.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "unknown");
    }

    #[test]
    fn test_no_false_positive_low_in_word() {
        // "mellow_pad" should NOT match bass (old keyword "low" is gone)
        let path = PathBuf::from("/samples/pads/mellow_pad.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "pad");
    }

    #[test]
    fn test_bass_drum_beats_bass() {
        // "bass drum" (9 chars) should win over "bass" (4 chars) → kick
        let path = PathBuf::from("/samples/kicks/bass_drum_01.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "kick");
    }

    #[test]
    fn test_extract_subcategory_hihat_open() {
        let result = extract_subcategory(" hihat open 01 ", "hihat");
        assert_eq!(result, "open");
    }

    #[test]
    fn test_extract_subcategory_hihat_closed() {
        let result = extract_subcategory(" hihat closed 02 ", "hihat");
        assert_eq!(result, "closed");
    }

    #[test]
    fn test_extract_subcategory_bass_808() {
        let result = extract_subcategory(" bass 808 long ", "bass");
        assert_eq!(result, "808");
    }

    #[test]
    fn test_extract_subcategory_bass_sub() {
        let result = extract_subcategory(" sub bass deep ", "bass");
        assert_eq!(result, "sub");
    }

    #[test]
    fn test_extract_subcategory_no_match() {
        let result = extract_subcategory(" generic bass ", "bass");
        assert_eq!(result, "");
    }
}
