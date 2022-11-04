use std::io::BufReader;

pub struct Player {
    sink: rodio::Sink,
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
    pub fn play(&self, path: &str) {
        let file = std::fs::File::open(path).unwrap();
        self.sink
            .append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    }
}
