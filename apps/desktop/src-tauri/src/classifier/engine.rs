use aho_corasick::AhoCorasick;
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

        let automaton = AhoCorasick::new(&patterns).expect("Failed to build AhoCorasick automaton");

        Self {
            automaton,
            pattern_map,
        }
    }

    /// Find the first matching rule for the given text.
    /// Returns (rule_index, matched_keyword) or None.
    pub fn find_match(&self, text: &str) -> Option<(usize, &'static str)> {
        self.automaton
            .find(text)
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
        let result = engine.find_match("my_kick_sample");
        assert!(result.is_some());
        let (idx, _) = result.unwrap();
        assert_eq!(RULES[idx].category, "kick");
    }

    #[test]
    fn test_engine_finds_snare() {
        let engine = rule_engine();
        let result = engine.find_match("snare_acoustic_01");
        assert!(result.is_some());
        let (idx, _) = result.unwrap();
        assert_eq!(RULES[idx].category, "snare");
    }

    #[test]
    fn test_engine_no_match() {
        let engine = rule_engine();
        let result = engine.find_match("completely_random_name");
        assert!(result.is_none());
    }
}
