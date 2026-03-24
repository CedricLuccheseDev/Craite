use super::engine::rule_engine;
use super::rules::RULES;
use std::path::Path;

/// Classification result for a single file
pub struct Classification {
    pub category: String,
    pub subcategory: String,
    pub confidence: f32,
}

/// Classify a sample based on its filename and parent directory.
/// Uses a pre-compiled AhoCorasick automaton for single-pass matching.
pub fn classify_by_filename(path: &Path) -> Classification {
    let filename = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let parent = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let combined = format!("{parent} {filename}");

    // Single-pass multi-pattern match via AhoCorasick
    if let Some((rule_idx, _keyword)) = rule_engine().find_match(&combined) {
        let rule = &RULES[rule_idx];
        return Classification {
            category: rule.category.to_string(),
            subcategory: extract_subcategory(&filename, rule.category),
            confidence: 0.8,
        };
    }

    Classification {
        category: "unknown".to_string(),
        subcategory: String::new(),
        confidence: 0.0,
    }
}

fn extract_subcategory(filename: &str, category: &str) -> String {
    // Try to extract a more specific subcategory
    match category {
        "hihat" => {
            if filename.contains("open") {
                "open".to_string()
            } else if filename.contains("closed") {
                "closed".to_string()
            } else {
                String::new()
            }
        }
        "bass" => {
            if filename.contains("808") {
                "808".to_string()
            } else if filename.contains("sub") {
                "sub".to_string()
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
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_classify_snare() {
        let path = PathBuf::from("/samples/drums/snare_acoustic.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "snare");
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_classify_hihat_open() {
        let path = PathBuf::from("/samples/hats/hihat_open_01.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "hihat");
        assert_eq!(result.subcategory, "open");
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_classify_hihat_closed() {
        let path = PathBuf::from("/samples/hats/hihat_closed_02.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "hihat");
        assert_eq!(result.subcategory, "closed");
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_classify_bass_808() {
        let path = PathBuf::from("/samples/bass/bass_808_long.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "bass");
        assert_eq!(result.subcategory, "808");
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_classify_bass_sub() {
        let path = PathBuf::from("/samples/bass/sub_bass.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "bass");
        assert_eq!(result.subcategory, "sub");
        assert_eq!(result.confidence, 0.8);
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
        let path = PathBuf::from("/samples/kicks/heavy_01.wav");
        let result = classify_by_filename(&path);
        assert_eq!(result.category, "kick");
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_extract_subcategory_hihat_open() {
        let result = extract_subcategory("hihat_open_01", "hihat");
        assert_eq!(result, "open");
    }

    #[test]
    fn test_extract_subcategory_hihat_closed() {
        let result = extract_subcategory("hihat_closed_02", "hihat");
        assert_eq!(result, "closed");
    }

    #[test]
    fn test_extract_subcategory_bass_808() {
        let result = extract_subcategory("bass_808_long", "bass");
        assert_eq!(result, "808");
    }

    #[test]
    fn test_extract_subcategory_bass_sub() {
        let result = extract_subcategory("sub_bass_deep", "bass");
        assert_eq!(result, "sub");
    }

    #[test]
    fn test_extract_subcategory_no_match() {
        let result = extract_subcategory("generic_bass", "bass");
        assert_eq!(result, "");
    }
}
