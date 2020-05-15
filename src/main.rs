use pomodorust::*;
use std::time::Duration;

fn main() {
    use ItemType::*;
    let minute = 1;
    let configuration = Configuration {
        short_break_duration: Duration::from_secs(minute * 5),
        long_break_duration: Duration::from_secs(minute * 25),
        work_session_duration: Duration::from_secs(minute * 25),
        pre_short_break_message: &String::from(
            "It's been 25 minutes of hard work, enjoy a 5 minute break!",
        ),
        pre_long_break_message: &String::from(
            "You've done a lot of work already, sweetie! Enjoy a 25 minutes break!",
        ),
        pre_work_session_message: &String::from(""),
        post_short_break_message: &String::from(
            "Hope you enjoyed the break, now back to work handsome!",
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
    start_pomodoro(pomodoro)
}
