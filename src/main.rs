use clap::Parser;
use kbar::Bar;
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use std::io::Cursor;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args = Args::parse();
    let sleep_time = hms_to_seconds(args.time.as_str()) / 100.0;
    let mut bar = Bar::new();

    bar.set_job_label(format!("Timer {} :", args.time).as_str());

    for x in 1..101 {
        bar.reach_percent(x);
        sleep(Duration::from_secs_f64(sleep_time));
    }

    println!("Finished 🥳️");

    // Play alert sound
    #[cfg(target_os = "windows")]
    const MP3_FILE: &'static [u8] = include_bytes!("alert.mp3");

    #[cfg(not(target_os = "windows"))]
    const MP3_FILE: &'static [u8] = include_bytes!("alert.mp3");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(Cursor::new(MP3_FILE));
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.set_volume(args.volume);
    sink.sleep_until_end();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Use time in hh:mm:ss format")]
    time: String,

    #[arg(short, long, default_value_t = 0.5, help = "Volume of alert sound")]
    volume: f32,
}

fn hms_to_seconds(s: &str) -> f64 {
    let parts = s.split(":").collect::<Vec<&str>>();
    let hours = f64::from_str(parts[0]).unwrap();
    let minutes = f64::from_str(parts[1]).unwrap();
    let seconds = f64::from_str(parts[2]).unwrap();

    (hours * 3600.0) + (minutes * 60.0) + seconds
}
