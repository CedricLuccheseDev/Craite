use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use crate::error::CraiteError;

pub struct AudioPreview {
    sink: Arc<Mutex<Option<Sink>>>,
    _stream: Option<OutputStream>,
}

impl AudioPreview {
    pub fn new() -> Self {
        AudioPreview {
            sink: Arc::new(Mutex::new(None)),
            _stream: None,
        }
    }

    /// Play an audio file for preview
    pub fn play(&mut self, path: &Path) -> Result<(), CraiteError> {
        self.stop();

        let stream = OutputStreamBuilder::open_default_stream()
            .map_err(|e: rodio::StreamError| CraiteError::Audio(e.to_string()))?;

        let sink = Sink::connect_new(stream.mixer());

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)
            .map_err(|e| CraiteError::Audio(e.to_string()))?;

        sink.append(source);

        *self.sink.lock().unwrap() = Some(sink);
        self._stream = Some(stream);

        Ok(())
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
}
