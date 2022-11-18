mod app;
mod mp3;
use std::error::Error;
use std::time;
fn main() -> Result<(), Box<dyn Error>> {
    //    test_player();
    app::run()?;
    Ok(())
}
use crate::mp3::Player;
use std::thread;
use std::time::Duration;
fn test_player() {
    let mut p = Player::new();
    p.play("/home/lin/Videos/p1.mp3");
    println!("{}", p.sink.len());
    println!("{}", p.empty());
    loop {
        if p.empty() {
            println!("finished");
            p.play("/home/lin/Videos/p1.mp3");
        }

        println!("{:?}", time::SystemTime::now());
        thread::sleep(Duration::from_secs(1));
    }
}
