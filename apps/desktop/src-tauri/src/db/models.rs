use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub category: String,
    pub subcategory: String,
    pub confidence: f32,
    pub source: String,
    pub duration: f64,
    pub sample_rate: u32,
    pub linked_path: Option<String>,
    /// File modification time (seconds since epoch) for incremental scan
    #[serde(default)]
    pub mtime: u64,
    /// Whether this sample is excluded from export
    #[serde(default)]
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub color: String,
    pub count: usize,
    pub subcategories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub total_files: usize,
    pub classified_files: usize,
    pub categories: Vec<Category>,
    pub samples: Vec<Sample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub path: String,
    pub label: String,
    pub enabled: bool,
    #[serde(rename = "type")]
    pub source_type: String,
    pub sample_count: usize,
}
