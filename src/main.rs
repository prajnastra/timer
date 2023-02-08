use kbar::Bar;
use rodio::{source::Source, Decoder, OutputStream};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let sleep_time = hms_to_seconds(config.query.as_str()) / 100.0;

    let mut bar = Bar::new();
    bar.set_job_label(format!("Timer {} :", config.query).as_str());

    for x in 1..101 {
        sleep(Duration::from_secs_f64(sleep_time));
        bar.reach_percent(x);
    }

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("assets/alert.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle
        .play_raw(source.convert_samples())
        .expect("Not able to play sound.");

    // The sound plays in a separate audio thread,
    sleep(Duration::from_secs(2));
}

struct Config {
    query: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();

    Config { query }
}

fn hms_to_seconds(s: &str) -> f64 {
    let parts = s.split(":").collect::<Vec<&str>>();
    let hours = f64::from_str(parts[0]).unwrap();
    let minutes = f64::from_str(parts[1]).unwrap();
    let seconds = f64::from_str(parts[2]).unwrap();

    (hours * 3600.0) + (minutes * 60.0) + seconds
}
