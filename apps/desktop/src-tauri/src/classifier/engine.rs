use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use std::sync::OnceLock;

use super::rules::RULES;

/// Pre-compiled multi-pattern matcher for fast classification.
/// Builds an AhoCorasick automaton from all keywords across all rules,
/// enabling single-pass substring matching instead of O(rules * keywords) checks.
pub struct RuleEngine {
    automaton: AhoCorasick,
    /// Maps each pattern index back to (rule_index, keyword)
    pattern_map: Vec<(usize, &'static str)>,
}

impl RuleEngine {
    fn build() -> Self {
        let mut patterns: Vec<&str> = Vec::new();
        let mut pattern_map: Vec<(usize, &'static str)> = Vec::new();

        for (rule_idx, rule) in RULES.iter().enumerate() {
            for keyword in rule.keywords {
                patterns.push(keyword);
                pattern_map.push((rule_idx, keyword));
            }
        }

        // LeftmostLongest: at the same start position, the longest keyword wins.
        // This ensures "bass drum" (→ kick) beats "bass" (→ bass) for "bass_drum_01".
        let automaton = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(&patterns)
            .expect("Failed to build AhoCorasick automaton");

        Self {
            automaton,
            pattern_map,
        }
    }

    /// Find the best matching rule for the given text.
    ///
    /// Uses `LeftmostLongest` match semantics:
    ///   - At the same start position, the longest keyword wins
    ///     ("bass drum" → kick beats "bass" → bass).
    ///   - When multiple matches occur at different positions, we prefer the longest;
    ///     ties in length are broken by leftmost position (smallest start index).
    pub fn find_match(&self, text: &str) -> Option<(usize, &'static str)> {
        self.automaton
            .find_iter(text)
            .max_by(|a, b| {
                a.len()
                    .cmp(&b.len())
                    // Equal length: prefer leftmost (smallest start = higher priority)
                    .then(b.start().cmp(&a.start()))
            })
            .map(|mat| self.pattern_map[mat.pattern().as_usize()])
    }
}

static ENGINE: OnceLock<RuleEngine> = OnceLock::new();

/// Get the global pre-compiled rule engine (built once, reused forever)
pub fn rule_engine() -> &'static RuleEngine {
    ENGINE.get_or_init(RuleEngine::build)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_finds_kick() {
        let engine = rule_engine();
        let result = engine.find_match(" my kick sample ");
        assert!(result.is_some());
        let (idx, _) = result.unwrap();
        assert_eq!(RULES[idx].category, "kick");
    }

    #[test]
    fn test_engine_finds_snare() {
        let engine = rule_engine();
        let result = engine.find_match(" snare acoustic 01 ");
        assert!(result.is_some());
        let (idx, _) = result.unwrap();
        assert_eq!(RULES[idx].category, "snare");
    }

    #[test]
    fn test_engine_no_match() {
        let engine = rule_engine();
        let result = engine.find_match(" completely random name ");
        assert!(result.is_none());
    }

    #[test]
    fn test_engine_prefers_longer_match() {
        // "bass drum" (9 chars) should beat "bass" (4 chars) → kick wins
        let engine = rule_engine();
        let result = engine.find_match(" bass drum 01 ");
        assert!(result.is_some());
        let (idx, _) = result.unwrap();
        assert_eq!(RULES[idx].category, "kick");
    }

    #[test]
    fn test_engine_word_boundary_hh() {
        // " hh " should match hihat, but "pshh" should not
        let engine = rule_engine();

        let match_hh = engine.find_match(" drums hh 01 ");
        assert!(match_hh.is_some());
        let (idx, _) = match_hh.unwrap();
        assert_eq!(RULES[idx].category, "hihat");

        // "pshh" after normalization becomes " pshh " — no standalone " hh "
        let no_match = engine.find_match(" pshh 01 ");
        assert!(no_match.is_none());
    }
}
