/// Classification rules mapping keywords to categories
pub struct ClassificationRule {
    pub category: &'static str,
    pub keywords: &'static [&'static str],
    pub color: &'static str,
}

pub const RULES: &[ClassificationRule] = &[
    ClassificationRule {
        category: "kick",
        keywords: &["kick", "kck", "kik", "bd", "bass drum", "bassdrum"],
        color: "#ff6b35",
    },
    ClassificationRule {
        category: "snare",
        keywords: &["snare", "snr", "sd", "rimshot", "rim"],
        color: "#4a9eff",
    },
    ClassificationRule {
        category: "hihat",
        keywords: &["hihat", "hi-hat", "hh", "hat", "open hat", "closed hat"],
        color: "#fbbf24",
    },
    ClassificationRule {
        category: "clap",
        keywords: &["clap", "clp", "handclap"],
        color: "#a78bfa",
    },
    ClassificationRule {
        category: "pad",
        keywords: &["pad", "atmosphere", "atmo", "ambient"],
        color: "#4ade80",
    },
    ClassificationRule {
        category: "lead",
        keywords: &["lead", "ld", "synth lead", "pluck"],
        color: "#f472b6",
    },
    ClassificationRule {
        category: "bass",
        keywords: &["bass", "sub", "808", "low"],
        color: "#ef4444",
    },
    ClassificationRule {
        category: "fx",
        keywords: &["fx", "effect", "sfx", "riser", "impact", "sweep", "noise"],
        color: "#06b6d4",
    },
    ClassificationRule {
        category: "vocal",
        keywords: &["vocal", "vox", "voice", "acapella", "adlib"],
        color: "#fb923c",
    },
    ClassificationRule {
        category: "loop",
        keywords: &["loop", "lp", "drum loop", "melody loop"],
        color: "#818cf8",
    },
];
