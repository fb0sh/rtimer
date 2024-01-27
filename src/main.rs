use chrono::{Local, Timelike};

use clap::Parser;
use regex::{Captures, Regex};
use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::{process, time::Duration};
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    ///!1h:1m:30s  !12h:1m:30s  !1h:1m  !1m:30s  !1h:30s  !1h  !2m  !3s  @12:33
    pattern: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    cli.pattern
        .as_deref()
        .map(|pattern| select_pattern(pattern))
        .map(|dur| start_timer(dur));
}

fn select_pattern(pattern: &str) -> Duration {
    println!("P: {}", pattern);
    let p1 = Regex::new(r"(?:^!)(\d{1,2}[hms]):?(\d{1,2}[hms])?:?(\d{1,2}[hms])?").unwrap();
    let p2 = Regex::new(r"(?:^\@)(\d+):(\d+)").unwrap();

    if p1.is_match(pattern) {
        return p1.captures(pattern).map(|data| handle_p1(data)).unwrap();
    } else if p2.is_match(pattern) {
        return p2.captures(pattern).map(|data| handle_p2(data)).unwrap();
    } else {
        eprintln!("Pls use -h flag to check you args!");
        process::exit(1);
    }
}

fn handle_p1(data: Captures<'_>) -> Duration {
    // !(\d+[hms]:?){1,3}
    let mut total: u64 = 0;

    for i in 1..data.len() {
        data.get(i).map(|one| {
            let p = one.as_str();
            p.chars().last().map(|c| {
                let secs = &p[0..p.len() - 1].parse::<u64>().unwrap();
                match c {
                    'h' => total += secs * 60 * 60,
                    'm' => total += secs * 60,
                    's' => total += secs,
                    _ => (),
                }
            })
        });
    }

    Duration::from_secs(total)
}

fn handle_p2(data: Captures<'_>) -> Duration {
    // (:?^@)(\d+):(\d+)
    let now = Local::now();
    let h = *&data[1].parse::<i32>().unwrap();
    let m = *&data[2].parse::<i32>().unwrap();
    let nh = now.hour() as i32;
    let nm = now.minute() as i32;
    if h >= nh && m >= nm {
        let hs = (h - nh) * 60 * 60;
        let ms = (m - nm) * 60;
        let total = (hs + ms) as u64;
        Duration::from_secs(total)
    } else {
        eprintln!("Check your time");
        process::exit(1);
    }
}

fn start_timer(secs: Duration) {
    println!("secs:={}s", &secs.as_secs());

    sleep(secs);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = File::open("tip.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
