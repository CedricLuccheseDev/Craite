use crate::error::CraiteError;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct AudioPreview {
    sink: Arc<Mutex<Option<Sink>>>,
    _stream: Option<OutputStream>,
    /// Incremented on each play() call so stale monitor threads can self-terminate
    generation: Arc<AtomicU64>,
}

impl AudioPreview {
    pub fn new() -> Self {
        AudioPreview {
            sink: Arc::new(Mutex::new(None)),
            _stream: None,
            generation: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Play an audio file for preview. Returns the generation ID for monitoring.
    pub fn play(&mut self, path: &Path) -> Result<u64, CraiteError> {
        self.stop();

        let gen = self.generation.fetch_add(1, Ordering::SeqCst) + 1;

        let stream = OutputStreamBuilder::open_default_stream()
            .map_err(|e: rodio::StreamError| CraiteError::Audio(e.to_string()))?;

        let sink = Sink::connect_new(stream.mixer());

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).map_err(|e| CraiteError::Audio(e.to_string()))?;

        sink.append(source);

        *self
            .sink
            .lock()
            .map_err(|e| CraiteError::Audio(format!("Mutex lock failed: {}", e)))? = Some(sink);
        self._stream = Some(stream);

        Ok(gen)
    }

    /// Stop any currently playing audio
    pub fn stop(&mut self) {
        if let Ok(mut guard) = self.sink.lock() {
            if let Some(sink) = guard.take() {
                sink.stop();
            }
        }
        self._stream = None;
    }

    /// Get a clone of the sink Arc for monitoring playback state
    pub fn sink_ref(&self) -> Arc<Mutex<Option<Sink>>> {
        Arc::clone(&self.sink)
    }

    /// Get a clone of the generation counter for monitoring
    pub fn generation_ref(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.generation)
    }
}
