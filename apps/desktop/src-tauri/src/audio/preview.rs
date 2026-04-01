use crate::error::CraiteError;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct AudioPreview {
    sink: Arc<Mutex<Option<Sink>>>,
    stream: Option<OutputStream>,
    generation: Arc<AtomicU64>,
}

impl AudioPreview {
    pub fn new() -> Self {
        AudioPreview {
            sink: Arc::new(Mutex::new(None)),
            stream: None,
            generation: Arc::new(AtomicU64::new(0)),
        }
    }

    fn ensure_stream(&mut self) -> Result<&OutputStream, CraiteError> {
        if self.stream.is_none() {
            let stream = OutputStreamBuilder::open_default_stream()
                .map_err(|e: rodio::StreamError| CraiteError::Audio(e.to_string()))?;
            self.stream = Some(stream);
        }
        Ok(self.stream.as_ref().unwrap())
    }

    /// Play an audio file for preview. Returns the generation ID for monitoring.
    pub fn play(&mut self, path: &Path) -> Result<u64, CraiteError> {
        // Stop current playback without destroying the stream
        if let Ok(mut guard) = self.sink.lock() {
            if let Some(sink) = guard.take() {
                sink.stop();
            }
        }

        let gen = self.generation.fetch_add(1, Ordering::SeqCst) + 1;

        let stream = self.ensure_stream()?;
        let sink = Sink::connect_new(stream.mixer());

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).map_err(|e| CraiteError::Audio(e.to_string()))?;

        sink.append(source);

        *self
            .sink
            .lock()
            .map_err(|e| CraiteError::Audio(format!("Mutex lock failed: {}", e)))? = Some(sink);

        Ok(gen)
    }

    /// Stop any currently playing audio
    pub fn stop(&mut self) {
        if let Ok(mut guard) = self.sink.lock() {
            if let Some(sink) = guard.take() {
                sink.stop();
            }
        }
    }

    pub fn sink_ref(&self) -> Arc<Mutex<Option<Sink>>> {
        Arc::clone(&self.sink)
    }

    pub fn generation_ref(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.generation)
    }
}
