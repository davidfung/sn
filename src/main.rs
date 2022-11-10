use chrono::prelude::*;
use std::io;
use webbrowser;

fn current_episode() -> String {
    let base_episode_no = 895;
    let base_episode_date = Local.ymd(2022, 11, 2).and_hms(0, 0, 0);
    let today = Local::now();
    let diff = today - base_episode_date;
    let s = format!("{}", base_episode_no + diff.num_weeks());
    s
}

fn main() {
    let mut episode = current_episode();
    println!("Enter Security Now episode no. [{}]> ", current_episode());

    let mut user_episode = String::new();
    io::stdin()
        .read_line(&mut user_episode)
        .expect("Failed to read line");
    user_episode = format!("{}", user_episode.trim());

    if user_episode != "" {
        episode = user_episode;
    }

    let url = format!("https://www.grc.com/sn/sn-{episode}-notes.pdf");
    println!("{url}");

    if webbrowser::open(&url).is_ok() {
        // ...
    }
}
