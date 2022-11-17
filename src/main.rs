use chrono::prelude::*;
use std::io;
use webbrowser;

fn current_episode() -> i64 {
    let base_episode_no = 895;
    let base_episode_date = Local.ymd(2022, 11, 2).and_hms(0, 0, 0);
    let today = Local::now();
    let diff = today - base_episode_date;
    return base_episode_no + diff.num_weeks();
}
fn main() {
    let current_episode = current_episode();
    println!("Enter Security Now episode no. [{}]> ", current_episode);

    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let mut episode;

    if str.trim() == "" {
        episode = current_episode;
    } else {
        episode = str.trim().parse().expect("Expect a number.");
        if episode < 0 {
            episode = current_episode + episode;
        }
    }

    let url = format!("https://www.grc.com/sn/sn-{episode}-notes.pdf");
    println!("{url}");

    if webbrowser::open(&url).is_ok() {
        // ...
    }
}
