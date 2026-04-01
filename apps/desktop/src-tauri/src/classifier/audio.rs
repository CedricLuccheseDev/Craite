use super::Classification;
use std::path::Path;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

const SUPPORTED_EXTS: &[&str] = &["wav", "mp3", "flac", "ogg", "aiff", "aif"];

/// Classify a sample by analysing its audio content.
/// Supports WAV, MP3, FLAC, OGG, AIFF via symphonia.
pub fn classify_by_audio(path: &Path) -> Option<Classification> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    if !SUPPORTED_EXTS.contains(&ext.as_str()) {
        return None;
    }

    let (mono, sample_rate, duration) = decode_first_512ms(path)?;
    if mono.is_empty() {
        return None;
    }

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

/// Decode up to 512ms of audio from any supported format, return mono f32 samples.
fn decode_first_512ms(path: &Path) -> Option<(Vec<f32>, u32, f32)> {
    let file = std::fs::File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .ok()?;

    let mut format = probed.format;
    let track = format.default_track()?;
    let sample_rate = track.codec_params.sample_rate?;
    let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(1) as usize;
    let track_id = track.id;

    let total_frames = track
        .codec_params
        .n_frames
        .unwrap_or(sample_rate as u64 * 10);
    let duration = total_frames as f32 / sample_rate as f32;

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .ok()?;

    let max_frames = (sample_rate as f32 * 0.512) as usize;
    let mut mono = Vec::with_capacity(max_frames);

    while mono.len() < max_frames {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(_) => break,
        };
        if packet.track_id() != track_id {
            continue;
        }
        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(_) => continue,
        };

        let spec = *decoded.spec();
        let num_frames = decoded.frames();
        let mut buf = SampleBuffer::<f32>::new(num_frames as u64, spec);
        buf.copy_interleaved_ref(decoded);
        let samples = buf.samples();

        let ch = channels.max(1);
        for frame in samples.chunks(ch) {
            if mono.len() >= max_frames {
                break;
            }
            let sum: f32 = frame.iter().sum();
            mono.push(sum / ch as f32);
        }
    }

    Some((mono, sample_rate, duration))
}

// ─── Feature extraction ──────────────────────────────────────────────────────

struct AudioFeatures {
    duration: f32,
    attack_ms: f32,
    zcr: f32,
    low_ratio: f32,
    energy_consistency: f32,
}

fn extract_features(mono: &[f32], sample_rate: u32, duration: f32) -> AudioFeatures {
    let n = mono.len();

    let zcr = mono
        .windows(2)
        .filter(|w| (w[0] >= 0.0) != (w[1] >= 0.0))
        .count() as f32
        / n as f32;

    let peak = mono.iter().copied().fold(0.0_f32, |a, s| a.max(s.abs()));
    let attack_samples = if peak > 0.001 {
        mono.iter().position(|s| s.abs() >= peak * 0.8).unwrap_or(0)
    } else {
        0
    };
    let attack_ms = attack_samples as f32 / sample_rate as f32 * 1000.0;

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
    if f.duration > 2.0 && f.energy_consistency > 0.65 {
        return ("loop".into(), String::new());
    }

    if f.attack_ms < 40.0 {
        if f.low_ratio > 0.45 {
            return ("kick".into(), String::new());
        }
        if f.zcr > 0.20 {
            if f.attack_ms < 8.0 {
                return ("hihat".into(), String::new());
            }
            return ("clap".into(), String::new());
        }
        return ("snare".into(), String::new());
    }

    if f.attack_ms > 150.0 {
        if f.duration > 3.0 || f.energy_consistency > 0.5 {
            return ("pad".into(), String::new());
        }
        if f.low_ratio > 0.50 {
            return ("bass".into(), String::new());
        }
        return ("pad".into(), String::new());
    }

    if f.zcr > 0.20 {
        return ("fx".into(), String::new());
    }
    if f.low_ratio > 0.55 {
        return ("bass".into(), String::new());
    }

    ("lead".into(), String::new())
}
