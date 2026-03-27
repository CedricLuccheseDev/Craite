/// Classification rules mapping keywords to categories.
///
/// Keywords are matched against a normalized string where all non-alphanumeric chars
/// are replaced by spaces and the whole string is padded with spaces on both sides.
/// This allows word-boundary-aware matching by padding short/ambiguous keywords
/// with spaces (e.g., `" hh "` only matches the standalone token "hh").
pub struct ClassificationRule {
    pub category: &'static str,
    pub keywords: &'static [&'static str],
    pub color: &'static str,
}

pub const RULES: &[ClassificationRule] = &[
    // --- Drums (#ff6b35) ---
    ClassificationRule {
        category: "kick",
        // "bd" and "kck"/"kik" removed — too short, replaced by normalized variants
        keywords: &["kick", " kck ", " kik ", "bass drum", "bassdrum", "kickdrum"],
        color: "#ff6b35",
    },
    ClassificationRule {
        category: "snare",
        // "sd" and "rim" removed — too generic; rimshot kept (specific enough)
        keywords: &["snare", " snr ", "rimshot", "snappy"],
        color: "#ff6b35",
    },
    ClassificationRule {
        category: "hihat",
        // "hh" replaced with " hh " (word-boundary); "hat" kept (domain-specific enough)
        keywords: &["hihat", "hi hat", "hi-hat", " hh ", " hat ", "closed hat", "open hat", "pedal hat"],
        color: "#ff6b35",
    },
    ClassificationRule {
        category: "clap",
        // "clp" wrapped with spaces for word-boundary
        keywords: &["clap", " clp ", "handclap", "snap clap"],
        color: "#ff6b35",
    },
    ClassificationRule {
        category: "tom",
        // "tom" wrapped — "tomorrow"/"tomato" would not match " tom " after normalization
        keywords: &[" tom ", "tom tom", "floor tom", "rack tom", "high tom", "mid tom", "low tom"],
        color: "#ff6b35",
    },
    ClassificationRule {
        category: "cymbal",
        // "crash"/"ride" alone are specific enough in audio sample naming context
        keywords: &["cymbal", "crash", "ride cymbal", "splash", "china cymbal"],
        color: "#ff6b35",
    },
    ClassificationRule {
        category: "perc",
        keywords: &[
            "perc", "percussion",
            "shaker", "tambourine", " tamb ",
            "conga", "bongo", "cowbell",
            "woodblock", "cajon", "maraca",
            "agogo", "cabasa",
        ],
        color: "#ff6b35",
    },
    // --- Bass (#ef4444) ---
    ClassificationRule {
        category: "bass",
        // "sub" and "low" removed (too generic); "sub bass"/"subbass" kept (specific)
        keywords: &["bass", "808", "sub bass", "subbass", "bassline", "reese"],
        color: "#ef4444",
    },
    // --- Melodic (#4ade80) ---
    ClassificationRule {
        category: "pad",
        keywords: &["pad", "atmosphere", "atmo", "ambient", "texture", "drone", "sustain pad"],
        color: "#4ade80",
    },
    ClassificationRule {
        category: "lead",
        // "ld" removed; "pluck" added (common lead type); "synth" alone is too broad → "synth lead"
        keywords: &["lead", "synth lead", "pluck", "mono lead"],
        color: "#4ade80",
    },
    ClassificationRule {
        category: "arp",
        // " arp " with spaces to avoid matching "sharp", "harper", etc.
        keywords: &[" arp ", "arpeggio", "arpeggiated"],
        color: "#4ade80",
    },
    ClassificationRule {
        category: "chord",
        keywords: &["chord", "stab", " chrd "],
        color: "#4ade80",
    },
    // --- Vocal (#f472b6) ---
    ClassificationRule {
        category: "vocal",
        // "vox" wrapped with spaces to avoid matching "voxel" etc.
        keywords: &["vocal", " vox ", "voice", "acapella", "adlib", "choir", "hook", "chant"],
        color: "#f472b6",
    },
    // --- FX (#06b6d4) ---
    ClassificationRule {
        category: "fx",
        // "noise" removed (too generic); " fx " and " sfx " wrapped for word boundary
        // "effect" kept (6 chars, specific enough)
        keywords: &[
            " fx ", " sfx ", "effect",
            "riser", "impact", "sweep", "downlift", "uplifter",
            "whoosh", "reverse", "transition",
        ],
        color: "#06b6d4",
    },
    // --- Loops (#818cf8) ---
    ClassificationRule {
        category: "loop",
        // "lp" removed (too short/generic); "loop" alone is specific enough
        keywords: &["loop", "drum loop", "melody loop", "bass loop", "full loop"],
        color: "#818cf8",
    },
];
