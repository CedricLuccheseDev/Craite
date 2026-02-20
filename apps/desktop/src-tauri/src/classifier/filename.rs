use std::path::Path;
use super::rules::RULES;

/// Classification result for a single file
pub struct Classification {
    pub category: String,
    pub subcategory: String,
    pub confidence: f32,
}

/// Classify a sample based on its filename and parent directory
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

    // Check each rule against filename + parent directory
    for rule in RULES {
        for keyword in rule.keywords {
            if combined.contains(keyword) {
                return Classification {
                    category: rule.category.to_string(),
                    subcategory: extract_subcategory(&filename, rule.category),
                    confidence: 0.8,
                };
            }
        }
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
            if filename.contains("open") { "open".to_string() }
            else if filename.contains("closed") { "closed".to_string() }
            else { String::new() }
        }
        "bass" => {
            if filename.contains("808") { "808".to_string() }
            else if filename.contains("sub") { "sub".to_string() }
            else { String::new() }
        }
        _ => String::new(),
    }
}
