use clap::Parser;
use rtimer::{exec_shell, play_sound, select_pattern, Cli};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let cli = Cli::parse();
    cli.pattern
        .as_deref()
        .map(|pattern| select_pattern(pattern))
        .map(|dur| start_timer(dur, cli.cmd));
}

fn start_timer(secs: Duration, cmd: Option<String>) {
    println!("secs:={}s", &secs.as_secs());
    sleep(secs);
    cmd.map(|c| println!("{}", exec_shell(c)));
    play_sound();
    play_sound();
    play_sound();
}
