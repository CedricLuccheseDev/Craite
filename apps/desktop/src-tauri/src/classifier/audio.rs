use super::Classification;
use std::path::Path;

/// Attempt to classify a sample by analysing its audio content.
/// Only handles WAV files for now; returns `None` for other formats or on read error.
///
/// This is a lightweight, zero-ML analysis: it reads the first ~512ms of audio
/// and computes three scalar features that are sufficient to distinguish the
/// major percussive/tonal categories without a heavy model.
///
/// Confidence is intentionally lower than filename-based classification (0.55)
/// because audio features alone can be ambiguous.
pub fn classify_by_audio(path: &Path) -> Option<Classification> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    if ext != "wav" {
        return None;
    }

    let mut reader = hound::WavReader::open(path).ok()?;
    let spec = reader.spec();

    if spec.bits_per_sample == 0 {
        return None;
    }

    let total_interleaved = reader.len(); // total samples across all channels
    let channels = spec.channels as u32;
    let sample_rate = spec.sample_rate;
    let duration = total_interleaved as f32 / channels as f32 / sample_rate as f32;

    // Read at most 512ms of audio (all channels interleaved)
    let frames_to_read = (sample_rate as f32 * 0.512) as usize;
    let to_read = (frames_to_read * channels as usize).min(total_interleaved as usize);

    let interleaved: Vec<f32> = read_as_f32(&mut reader, &spec, to_read)?;

    if interleaved.is_empty() {
        return None;
    }

    // Mix down to mono
    let mono: Vec<f32> = if channels == 1 {
        interleaved
    } else {
        interleaved
            .chunks(channels as usize)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32)
            .collect()
    };

    let features = extract_features(&mono, sample_rate, duration);
    let (category, subcategory) = infer_category(&features);

    Some(Classification {
        category,
        subcategory,
        confidence: 0.55,
        duration,
        sample_rate,
    })
}

// ─── Feature extraction ──────────────────────────────────────────────────────

struct AudioFeatures {
    duration: f32,
    /// Time in ms from start until signal reaches 80% of peak amplitude
    attack_ms: f32,
    /// Zero-crossing rate: proportion of adjacent sample pairs with opposite signs
    zcr: f32,
    /// Ratio of low-frequency energy (crude downsampling proxy) to total RMS
    low_ratio: f32,
    /// Ratio of second-half RMS to first-half RMS (high = sustained, low = decaying)
    energy_consistency: f32,
}

fn extract_features(mono: &[f32], sample_rate: u32, duration: f32) -> AudioFeatures {
    let n = mono.len();

    // Zero-crossing rate
    let zcr = mono
        .windows(2)
        .filter(|w| (w[0] >= 0.0) != (w[1] >= 0.0))
        .count() as f32
        / n as f32;

    // Attack: samples until we reach 80% of peak
    let peak = mono.iter().copied().fold(0.0_f32, |a, s| a.max(s.abs()));
    let attack_samples = if peak > 0.001 {
        mono.iter().position(|s| s.abs() >= peak * 0.8).unwrap_or(0)
    } else {
        0
    };
    let attack_ms = attack_samples as f32 / sample_rate as f32 * 1000.0;

    // Low-frequency energy ratio.
    // Averaging every 32 samples acts as a crude low-pass filter (≈ fc ≈ sr/64).
    // Compare the RMS of the downsampled signal to total RMS.
    const DOWNSAMPLE: usize = 32;
    let downsampled: Vec<f32> = mono
        .chunks(DOWNSAMPLE)
        .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
        .collect();
    let total_rms = rms(mono);
    let low_rms = rms(&downsampled);
    let low_ratio = if total_rms > 0.001 {
        (low_rms / total_rms).min(1.0)
    } else {
        0.0
    };

    // Energy consistency: second half vs first half
    let mid = n / 2;
    let first_rms = rms(&mono[..mid]);
    let second_rms = rms(&mono[mid..]);
    let energy_consistency = if first_rms > 0.001 {
        (second_rms / first_rms).min(1.0)
    } else {
        0.0
    };

    AudioFeatures {
        duration,
        attack_ms,
        zcr,
        low_ratio,
        energy_consistency,
    }
}

fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = samples.iter().map(|s| s * s).sum();
    (sum_sq / samples.len() as f32).sqrt()
}

// ─── Category inference ───────────────────────────────────────────────────────

fn infer_category(f: &AudioFeatures) -> (String, String) {
    // Loops: long duration with consistent energy across the window
    if f.duration > 2.0 && f.energy_consistency > 0.65 {
        return ("loop".into(), String::new());
    }

    // Fast attack → percussive family
    if f.attack_ms < 40.0 {
        if f.low_ratio > 0.45 {
            // Heavy low-end + fast transient = kick (or 808 hit)
            return ("kick".into(), String::new());
        }
        if f.zcr > 0.20 {
            // Bright high-frequency, very fast = hihat; slightly slower = clap
            if f.attack_ms < 8.0 {
                return ("hihat".into(), String::new());
            }
            return ("clap".into(), String::new());
        }
        // Mid-frequency fast transient = snare
        return ("snare".into(), String::new());
    }

    // Slow attack → tonal/sustained family
    if f.attack_ms > 150.0 {
        if f.duration > 3.0 || f.energy_consistency > 0.5 {
            return ("pad".into(), String::new());
        }
        if f.low_ratio > 0.50 {
            return ("bass".into(), String::new());
        }
        return ("pad".into(), String::new());
    }

    // Medium attack — use tonal cues to distinguish
    if f.zcr > 0.20 {
        // High ZCR with no clear fast transient = noise/fx/texture
        return ("fx".into(), String::new());
    }
    if f.low_ratio > 0.55 {
        return ("bass".into(), String::new());
    }

    ("lead".into(), String::new())
}

// ─── WAV decoding helpers ─────────────────────────────────────────────────────

fn read_as_f32(
    reader: &mut hound::WavReader<std::io::BufReader<std::fs::File>>,
    spec: &hound::WavSpec,
    to_read: usize,
) -> Option<Vec<f32>> {
    let samples = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .samples::<f32>()
            .take(to_read)
            .filter_map(|s| s.ok())
            .collect(),

        hound::SampleFormat::Int if spec.bits_per_sample <= 16 => {
            let max_val = (1_i32 << (spec.bits_per_sample.saturating_sub(1))) as f32;
            reader
                .samples::<i16>()
                .take(to_read)
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / max_val)
                .collect()
        }

        hound::SampleFormat::Int => {
            let max_val = (1_i64 << (spec.bits_per_sample.saturating_sub(1) as u32)) as f32;
            reader
                .samples::<i32>()
                .take(to_read)
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / max_val)
                .collect()
        }
    };

    Some(samples)
}
