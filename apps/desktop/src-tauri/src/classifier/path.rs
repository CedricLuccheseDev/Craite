use super::engine::rule_engine;
use super::rules::RULES;
use super::Classification;
use std::path::Path;

/// Confidence levels by distance from file (0 = filename, 1 = parent, …)
const CONFIDENCE_BY_DEPTH: [f32; 5] = [0.85, 0.65, 0.55, 0.45, 0.40];

fn confidence_for_depth(depth: usize) -> f32 {
    if depth < CONFIDENCE_BY_DEPTH.len() {
        CONFIDENCE_BY_DEPTH[depth]
    } else {
        *CONFIDENCE_BY_DEPTH.last().unwrap()
    }
}

/// Normalize a path segment for keyword matching:
/// lowercase, non-alnum → space, padded with leading/trailing space for word-boundary matching.
fn normalize(s: &str) -> String {
    let body: String = s
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { ' ' })
        .collect();
    format!(" {body} ")
}

/// Classify a sample by scanning the full path from filename up to `source_root`.
///
/// Walks every ancestor directory between the file and the source root,
/// matching each segment against the keyword engine. Returns the closest
/// (= highest confidence) match found.
pub fn classify_by_path(path: &Path, source_root: &Path) -> Classification {
    let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let norm_filename = normalize(filename);

    // Depth 0: filename itself
    if let Some((rule_idx, _)) = rule_engine().find_match(&norm_filename) {
        let rule = &RULES[rule_idx];
        return Classification {
            category: rule.category.to_string(),
            subcategory: extract_subcategory(&norm_filename, rule.category),
            confidence: CONFIDENCE_BY_DEPTH[0],
            duration: 0.0,
            sample_rate: 0,
        };
    }

    // Depth 1+: walk ancestors up to source_root
    let mut depth: usize = 1;
    let mut current = path.parent();

    while let Some(dir) = current {
        // Stop when we've reached or passed the source root
        if dir == source_root || !dir.starts_with(source_root) {
            break;
        }

        let dir_name = dir.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if !dir_name.is_empty() {
            let norm = normalize(dir_name);
            if let Some((rule_idx, _)) = rule_engine().find_match(&norm) {
                let rule = &RULES[rule_idx];
                return Classification {
                    category: rule.category.to_string(),
                    subcategory: String::new(), // subcategories only from filename
                    confidence: confidence_for_depth(depth),
                    duration: 0.0,
                    sample_rate: 0,
                };
            }
        }

        depth += 1;
        current = dir.parent();
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

    const ROOT: &str = "/samples";

    #[test]
    fn test_classify_kick() {
        let path = PathBuf::from("/samples/drums/kick_heavy.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "kick");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_snare() {
        let path = PathBuf::from("/samples/drums/snare_acoustic.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "snare");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_hihat_open() {
        let path = PathBuf::from("/samples/hats/hihat_open_01.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "hihat");
        assert_eq!(result.subcategory, "open");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_bass_808() {
        let path = PathBuf::from("/samples/bass/bass_808_long.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "bass");
        assert_eq!(result.subcategory, "808");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_classify_unknown() {
        let path = PathBuf::from("/samples/random/file123.xyz");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "unknown");
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_classify_with_parent_directory() {
        let path = PathBuf::from("/samples/kicks/heavy_01.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "kick");
        assert_eq!(result.confidence, 0.65);
    }

    #[test]
    fn test_classify_with_grandparent_directory() {
        let path = PathBuf::from("/samples/Snares/Acoustic/01.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "snare");
        assert_eq!(result.confidence, 0.55);
    }

    #[test]
    fn test_deep_path_at_depth_3() {
        // "Kicks" is at depth 3 from file → confidence 0.45
        let path = PathBuf::from("/samples/Pack/Kicks/Sub/xyz_42.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "kick");
        assert_eq!(result.confidence, 0.55);
    }

    #[test]
    fn test_deep_path_no_match_stops_at_root() {
        let path = PathBuf::from("/samples/PackA/FolderB/random/xyz.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "unknown");
    }

    #[test]
    fn test_no_false_positive_hh_in_word() {
        let path = PathBuf::from("/samples/random/pshh_01.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "unknown");
    }

    #[test]
    fn test_bass_drum_beats_bass() {
        let path = PathBuf::from("/samples/kicks/bass_drum_01.wav");
        let result = classify_by_path(&path, Path::new(ROOT));
        assert_eq!(result.category, "kick");
    }

    #[test]
    fn test_extract_subcategory_hihat_open() {
        let result = extract_subcategory(" hihat open 01 ", "hihat");
        assert_eq!(result, "open");
    }

    #[test]
    fn test_extract_subcategory_bass_808() {
        let result = extract_subcategory(" bass 808 long ", "bass");
        assert_eq!(result, "808");
    }
}
