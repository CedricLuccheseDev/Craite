pub mod audio;
pub mod engine;
pub mod path;
pub mod rules;

/// Classification result for a single file
#[derive(Clone)]
pub struct Classification {
    pub category: String,
    pub subcategory: String,
    pub confidence: f32,
    /// Duration in seconds (0.0 if not yet analysed)
    pub duration: f32,
    /// Sample rate in Hz (0 if not yet analysed)
    pub sample_rate: u32,
}
