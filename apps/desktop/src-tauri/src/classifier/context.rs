use crate::db::models::Sample;
use std::collections::HashMap;
use std::path::Path;

/// Post-classification pass: use directory sibling context to resolve unknowns.
///
/// Groups samples by parent directory. If a dominant category represents ≥ 75%
/// of classified siblings and there are at least 3 classified siblings,
/// unknown samples in that directory inherit the dominant category (confidence 0.40).
pub fn apply_directory_context(samples: &mut [Sample]) {
    let mut dir_groups: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, sample) in samples.iter().enumerate() {
        if let Some(parent) = Path::new(&sample.path).parent() {
            dir_groups
                .entry(parent.to_string_lossy().to_string())
                .or_default()
                .push(i);
        }
    }

    for indices in dir_groups.values() {
        // Count categories (excluding unknown)
        let mut counts: HashMap<&str, usize> = HashMap::new();
        let mut known_count: usize = 0;

        for &i in indices {
            if samples[i].category != "unknown" {
                *counts.entry(&samples[i].category).or_default() += 1;
                known_count += 1;
            }
        }

        // Need at least 3 classified siblings for a meaningful signal
        if known_count < 3 {
            continue;
        }

        // Find dominant category
        let (dominant, dominant_count) = counts
            .iter()
            .max_by_key(|(_, &c)| c)
            .map(|(cat, &c)| (*cat, c))
            .unwrap();

        let ratio = dominant_count as f32 / known_count as f32;
        if ratio < 0.75 {
            continue;
        }

        // Patch unknowns with the dominant category
        let dominant_owned = dominant.to_string();
        for &i in indices {
            if samples[i].category == "unknown" {
                samples[i].category = dominant_owned.clone();
                samples[i].confidence = 0.40;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(path: &str, category: &str) -> Sample {
        Sample {
            id: 0,
            name: Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            path: path.to_string(),
            category: category.to_string(),
            subcategory: String::new(),
            confidence: if category == "unknown" { 0.0 } else { 0.85 },
            source: "/samples".to_string(),
            duration: 0.5,
            sample_rate: 44100,
            linked_path: None,
            mtime: 0,
            hidden: false,
        }
    }

    #[test]
    fn test_unknown_inherits_dominant_category() {
        let mut samples = vec![
            sample("/samples/kicks/a.wav", "kick"),
            sample("/samples/kicks/b.wav", "kick"),
            sample("/samples/kicks/c.wav", "kick"),
            sample("/samples/kicks/xyz.wav", "unknown"),
        ];
        apply_directory_context(&mut samples);
        assert_eq!(samples[3].category, "kick");
        assert_eq!(samples[3].confidence, 0.40);
    }

    #[test]
    fn test_mixed_directory_no_override() {
        let mut samples = vec![
            sample("/samples/mixed/a.wav", "kick"),
            sample("/samples/mixed/b.wav", "snare"),
            sample("/samples/mixed/c.wav", "hihat"),
            sample("/samples/mixed/xyz.wav", "unknown"),
        ];
        apply_directory_context(&mut samples);
        assert_eq!(samples[3].category, "unknown"); // no dominant category
    }

    #[test]
    fn test_too_few_siblings_no_override() {
        let mut samples = vec![
            sample("/samples/kicks/a.wav", "kick"),
            sample("/samples/kicks/b.wav", "kick"),
            sample("/samples/kicks/xyz.wav", "unknown"),
        ];
        apply_directory_context(&mut samples);
        assert_eq!(samples[2].category, "unknown"); // only 2 classified siblings
    }
}
