use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn main() {
    let mut seconds = Duration::from_secs(900);
    let mut alternator = true;

    loop {
        thread::sleep(seconds.clone());

        if alternator {
            println!("Time to Stand Up!");
        } else {
            println!("Time to Get Back to Work!");
        }

        play_audio();

        alternator = !alternator;
        seconds = if alternator {
            Duration::from_secs(15)
        } else {
            Duration::from_secs(900)
        };
    }
}

fn play_audio() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("../assets/notifSound.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
