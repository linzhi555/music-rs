use std::io::BufReader;
use std::{thread,time};

pub fn play(path: &str) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(path).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    thread::sleep(time::Duration::from_secs(20));
}
