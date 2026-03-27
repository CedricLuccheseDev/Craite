use super::Classification;

/// Fuse path-based and audio-based classification signals into a final result.
///
/// Priority: path classification is preferred when confident (folder/filename
/// context is very domain-specific in audio production). Audio analysis
/// supplements or overrides when path evidence is weak or absent.
pub fn fuse_signals(path_cls: &Classification, audio_cls: Option<&Classification>) -> Classification {
    let audio = match audio_cls {
        Some(a) => a,
        None => return path_cls.clone(),
    };

    // Both unknown → unknown
    if path_cls.category == "unknown" && audio.category == "unknown" {
        return Classification {
            category: "unknown".into(),
            subcategory: String::new(),
            confidence: 0.0,
            duration: audio.duration,
            sample_rate: audio.sample_rate,
        };
    }

    // Path unknown → audio wins
    if path_cls.category == "unknown" {
        return audio.clone();
    }

    // Audio unknown → path wins (but grab duration/sample_rate from audio)
    if audio.category == "unknown" {
        return Classification {
            duration: audio.duration,
            sample_rate: audio.sample_rate,
            ..path_cls.clone()
        };
    }

    // Both have results — agreement?
    if path_cls.category == audio.category {
        return Classification {
            category: path_cls.category.clone(),
            subcategory: path_cls.subcategory.clone(),
            confidence: (path_cls.confidence + audio.confidence * 0.3).min(0.95),
            duration: audio.duration,
            sample_rate: audio.sample_rate,
        };
    }

    // Disagreement: path confident (≥ 0.65 = parent match or better) → path wins
    if path_cls.confidence >= 0.65 {
        return Classification {
            confidence: path_cls.confidence * 0.9,
            duration: audio.duration,
            sample_rate: audio.sample_rate,
            ..path_cls.clone()
        };
    }

    // Low-confidence path vs audio — higher confidence wins
    if audio.confidence > path_cls.confidence {
        return Classification {
            duration: audio.duration,
            sample_rate: audio.sample_rate,
            ..audio.clone()
        };
    }

    // Default: path wins with penalty
    Classification {
        confidence: path_cls.confidence * 0.85,
        duration: audio.duration,
        sample_rate: audio.sample_rate,
        ..path_cls.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cls(cat: &str, conf: f32) -> Classification {
        Classification {
            category: cat.into(),
            subcategory: String::new(),
            confidence: conf,
            duration: 0.5,
            sample_rate: 44100,
        }
    }

    #[test]
    fn test_no_audio() {
        let path = cls("kick", 0.85);
        let result = fuse_signals(&path, None);
        assert_eq!(result.category, "kick");
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_both_unknown() {
        let path = cls("unknown", 0.0);
        let audio = cls("unknown", 0.0);
        let result = fuse_signals(&path, Some(&audio));
        assert_eq!(result.category, "unknown");
    }

    #[test]
    fn test_path_unknown_audio_wins() {
        let path = cls("unknown", 0.0);
        let audio = cls("kick", 0.55);
        let result = fuse_signals(&path, Some(&audio));
        assert_eq!(result.category, "kick");
        assert_eq!(result.confidence, 0.55);
    }

    #[test]
    fn test_agreement_boosts_confidence() {
        let path = cls("kick", 0.65);
        let audio = cls("kick", 0.55);
        let result = fuse_signals(&path, Some(&audio));
        assert_eq!(result.category, "kick");
        assert!(result.confidence > 0.65);
    }

    #[test]
    fn test_disagreement_path_confident() {
        let path = cls("kick", 0.65);
        let audio = cls("snare", 0.55);
        let result = fuse_signals(&path, Some(&audio));
        assert_eq!(result.category, "kick");
        assert!(result.confidence < 0.65); // penalty
    }

    #[test]
    fn test_disagreement_audio_wins_over_weak_path() {
        let path = cls("bass", 0.40);
        let audio = cls("kick", 0.55);
        let result = fuse_signals(&path, Some(&audio));
        assert_eq!(result.category, "kick");
    }
}
