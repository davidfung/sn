use chrono::prelude::*;
use std::io;
use webbrowser;

const URL_TEMPLATE: &str = "https://www.grc.com/sn/sn-{0}-notes.pdf";

fn main() {
    let episode = get_episode();
    display_show_notes(episode);
}

fn current_episode() -> i64 {
    let base_episode_no = 895;
    let base_episode_date = Local.ymd(2022, 11, 2).and_hms(0, 0, 0);
    let today = Local::now();
    let diff = today - base_episode_date;
    return base_episode_no + diff.num_weeks();
}

fn display_show_notes(episode: i64) {
    let url = URL_TEMPLATE.replace("{0}", &episode.to_string());
    println!("{url}");

    if webbrowser::open(&url).is_ok() {
        // ...
    }
}

fn get_episode() -> i64 {
    let mut episode;
    let current_episode = current_episode();
    println!("Enter Security Now episode no. [{}]> ", current_episode);

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    if s.trim() == "" {
        episode = current_episode;
    } else {
        episode = s.trim().parse().expect("Expect a number.");
        if episode < 0 {
            episode = current_episode + episode;
        }
    }
    episode
}
