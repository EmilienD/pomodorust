use clap::{App, ArgMatches};
use pomodorust::*;
use std::time::Duration;

fn main() {
    use ItemType::*;

    let matches = App::new("pomodorust")
        .version("0.0.1")
        .author("Emilien Durieu <contact@emilienduri.eu>")
        .about("Pomodoro from the command line")
        .arg("-s, --short-break-duration=[SECONDS] 'Sets a custom duration in minutes for short breaks, defaults to 5'")
        .arg("-l, --long-break-duration=[SECONDS] 'Sets a custom duration in minutes for long breaks, defaults to 25'")
        .arg("-w, --work-session-duration=[SECONDS] 'Sets a custom duration in minutes for work sessions, defaults to 25'")
        .arg("-n, --no-loop 'Program stops after a full cycle'")
        .get_matches();
    let configuration = Configuration {
        short_break_duration: get_minutes_number_argument(&matches, "short-break-duration", 5),
        long_break_duration: get_minutes_number_argument(&matches, "long-break-duration", 25),
        work_session_duration: get_minutes_number_argument(&matches, "work-session-duration", 25),
        pre_short_break_message: &String::from(
            "It's been 25 minutes of hard work, enjoy a 5 minute break!",
        ),
        pre_long_break_message: &String::from(
            "You've done a lot of work already, sweetie! Enjoy a 25 minutes break!",
        ),
        pre_work_session_message: &String::from(""),
        post_short_break_message: &String::from(
            "Hope you enjoyed the break, now back to work, handsome!",
        ),
        post_long_break_message: &String::from(
            "Hope that was a relaxing break, now back to work, chief!",
        ),
        post_work_session_message: &String::from(""),
    };
    let pomodoro = create_pomodoro(
        &vec![
            WorkSession,
            ShortBreak,
            WorkSession,
            ShortBreak,
            WorkSession,
            ShortBreak,
            WorkSession,
            LongBreak,
        ],
        configuration,
    );
    println!("Let's get some shit done, comrade!");
    if matches.is_present("no-loop") {
        start_pomodoro(&pomodoro)
    } else {
        loop {
            start_pomodoro(&pomodoro)
        }
    }
}

fn get_minutes_number_argument(matches: &ArgMatches, name: &str, default: u64) -> Duration {
    let minute = 60;
    Duration::from_secs(if let Ok(s) = matches.value_of_t::<u64>(name) {
        minute * s
    } else {
        minute * default
    })
}
