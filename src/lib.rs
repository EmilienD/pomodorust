use std::{thread, time::Duration};

pub struct Configuration<'a> {
  pub short_break_duration: Duration,
  pub long_break_duration: Duration,
  pub work_session_duration: Duration,
  pub pre_short_break_message: &'a String,
  pub pre_long_break_message: &'a String,
  pub pre_work_session_message: &'a String,
  pub post_short_break_message: &'a String,
  pub post_long_break_message: &'a String,
  pub post_work_session_message: &'a String,
}

pub enum ItemType {
  LongBreak,
  ShortBreak,
  WorkSession,
}

pub struct Item<'a> {
  remaining_duration: Duration,
  start_message: &'a String,
  end_message: &'a String,
}

pub struct Cycle<'a> {
  items: Vec<Item<'a>>,
}

pub fn create_pomodoro<'a>(
  item_types: &Vec<ItemType>,
  configuration: Configuration<'a>,
) -> Cycle<'a> {
  use ItemType::*;
  let items = item_types
    .iter()
    .map(|item_type| match item_type {
      LongBreak => Item {
        start_message: configuration.pre_long_break_message,
        end_message: configuration.post_long_break_message,
        remaining_duration: configuration.long_break_duration,
      },
      ShortBreak => Item {
        start_message: configuration.pre_short_break_message,
        end_message: configuration.post_short_break_message,
        remaining_duration: configuration.short_break_duration,
      },
      WorkSession => Item {
        start_message: configuration.pre_work_session_message,
        end_message: configuration.post_work_session_message,
        remaining_duration: configuration.work_session_duration,
      },
    })
    .collect();
  Cycle { items }
}

fn std_output(message: &String) {
  println!("{}", message)
}

fn cycling(pomodoro: &Cycle, timer: &mut impl FnMut(Duration), output: &mut impl FnMut(&String)) {
  for item in &pomodoro.items {
    if item.start_message.len() > 0 {
      output(item.start_message);
    }
    timer(item.remaining_duration);
    if item.end_message.len() > 0 {
      output(item.end_message);
    }
  }
}

pub fn start_pomodoro(pomodoro: &Cycle) {
  cycling(&pomodoro, &mut thread::sleep, &mut std_output)
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn cycling_full_config_test() {
    let configuration = Configuration {
      short_break_duration: Duration::new(0, 0),
      long_break_duration: Duration::new(0, 0),
      work_session_duration: Duration::new(0, 0),
      pre_short_break_message: &String::from("test"),
      pre_long_break_message: &String::from("test"),
      pre_work_session_message: &String::from("test"),
      post_short_break_message: &String::from("test"),
      post_long_break_message: &String::from("test"),
      post_work_session_message: &String::from("test"),
    };
    let pomodoro = create_pomodoro(&vec![ItemType::WorkSession], configuration);
    let mut output_calls = 0;
    let mut timer_calls = 0;

    cycling(
      &pomodoro,
      &mut |_| {
        timer_calls += 1;
      },
      &mut |_| output_calls += 1,
    );

    assert_eq!(output_calls, 2);
    assert_eq!(timer_calls, 1);
  }
  #[test]
  fn cycling_empty_message_config_test() {
    let configuration = Configuration {
      short_break_duration: Duration::new(0, 0),
      long_break_duration: Duration::new(0, 0),
      work_session_duration: Duration::new(0, 0),
      pre_short_break_message: &String::from(""),
      pre_long_break_message: &String::from(""),
      pre_work_session_message: &String::from(""),
      post_short_break_message: &String::from(""),
      post_long_break_message: &String::from(""),
      post_work_session_message: &String::from(""),
    };
    let pomodoro = create_pomodoro(&vec![ItemType::WorkSession], configuration);
    let mut output_calls = 0;
    let mut timer_calls = 0;

    cycling(&pomodoro, &mut |_| timer_calls += 1, &mut |_| {
      output_calls += 1
    });

    assert_eq!(output_calls, 0);
    assert_eq!(timer_calls, 1);
  }
}
