use std::{thread, time::Duration};

struct Configuration<'a> {
    short_break_duration: Duration,
    long_break_duration: Duration,
    work_session_duration: Duration,
    pre_short_break_message: &'a String,
    pre_long_break_message: &'a String,
    pre_work_session_message: &'a String,
}

enum ItemType {
    LongBreak,
    ShortBreak,
    WorkSession,
}

struct Item<'a> {
    remaining_duration: Duration,
    start_message: &'a String,
}

struct Cycle<'a> {
    items: Vec<Item<'a>>,
}

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
        pre_work_session_message: &String::from(
            "Hope you enjoyed the break, now back to work handsome!",
        ),
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
    loop {
        for item in &pomodoro.items {
            println!("{}", item.start_message);
            thread::sleep(item.remaining_duration);
        }
    }
}

fn create_pomodoro<'a>(item_types: &Vec<ItemType>, configuration: Configuration<'a>) -> Cycle<'a> {
    use ItemType::*;
    let items = item_types
        .iter()
        .map(|item_type| match item_type {
            LongBreak => Item {
                start_message: configuration.pre_long_break_message,
                remaining_duration: configuration.long_break_duration,
            },
            ShortBreak => Item {
                start_message: configuration.pre_short_break_message,
                remaining_duration: configuration.short_break_duration,
            },
            WorkSession => Item {
                start_message: configuration.pre_work_session_message,
                remaining_duration: configuration.work_session_duration,
            },
        })
        .collect();
    Cycle { items }
}
