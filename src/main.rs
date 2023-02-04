use std::env;
use kdam::{tqdm, BarExt};
use std::str::FromStr;

fn hms_to_seconds(s: &str) -> f64 {
    let parts = s.split(":").collect::<Vec<&str>>();
    let hours = f64::from_str(parts[0]).unwrap();
    let minutes = f64::from_str(parts[1]).unwrap();
    let seconds = f64::from_str(parts[2]).unwrap();

    (hours * 3600.0) + (minutes * 60.0) + seconds
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let sleep_time = hms_to_seconds(config.query.as_str()) / 100.0;

    println!("Timer starting for {}", config.query);

    let render_length = 100;

    let mut pb = tqdm!(
        total = render_length,
        desc = "Timer   ",
        animation = "fillup",
        position = 0,
        force_refresh = true
    );

    pb.set_postfix("");

    for _ in 0..render_length {
        pb.update(1);
        std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
    }
}

struct Config {
    query: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();

    Config { query }
}

