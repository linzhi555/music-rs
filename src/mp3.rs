use std::io::BufReader;

pub struct Player {
    pub sink: rodio::Sink,
    _stream: rodio::OutputStream,
    _handle: rodio::OutputStreamHandle,
}
impl Player {
    pub fn new() -> Self {
        let (_stream, _handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&_handle).unwrap();
        return Player {
            sink,
            _stream,
            _handle,
        };
    }
    pub fn play(&mut self, path: &str) {
        let file = std::fs::File::open(path).unwrap();
        if !self.sink.empty() {
            self.sink.stop()
        }
        self.sink = rodio::Sink::try_new(&self._handle).unwrap();
        self.sink
            .append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    }
    pub fn toggle_pause(&self) {
        if self.sink.empty() {
            return;
        }
        if self.sink.is_paused() {
            self.sink.play()
        } else {
            self.sink.pause();
        }
    }
    pub fn empty(&self) -> bool {
        return self.sink.empty();
    }
}
