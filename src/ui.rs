use std::str::Lines;

use crate::{art, state::State};
use indoc::indoc;
use itertools::izip;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub(crate) fn ui(f: &mut Frame, state: &mut State) {
    // header
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
        .split(f.size());
    // let header = Block::default().borders(Borders::ALL);
    f.render_widget(Paragraph::new(art::POMODORO_LOGO).white(), chunks[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // timer
    let time_remaining = state.time_remaining();

    let timer_block = if state.timer_is_finished() {
        match state.next_timer_colour() {
            crate::state::TimerColour::Red => {
                Block::default().borders(Borders::ALL).title("Timer").red()
            }
            crate::state::TimerColour::White => {
                Block::default().borders(Borders::ALL).title("Timer")
            }
        }
    } else {
        Block::default().borders(Borders::ALL).title("Timer")
    };

    let remaining_time_string = remaining_time_as_string(time_remaining);
    let timer = Paragraph::new(remaining_time_string).alignment(Alignment::Center);
    f.render_widget(timer.block(timer_block), chunks[0]);

    // tasks
    let tasks = Block::default().borders(Borders::ALL).title("Tasks");
    f.render_widget(tasks, chunks[1]);
}

fn remaining_time_as_string(remaining_seconds: i128) -> String {
    let remaining_minutes = format!("{:0>2}", remaining_seconds / 60);
    let remaining_seconds = format!("{:0>2}", remaining_seconds % 60);
    render_time(remaining_minutes, remaining_seconds)
}

fn render_time(remaining_minutes: String, remaining_seconds: String) -> String {
    let mut min_chars = remaining_minutes.chars().collect::<Vec<char>>();
    let mut seconds_chars = remaining_seconds.chars().collect::<Vec<char>>();
    let min_1 = render_char(min_chars.remove(0));
    let min_2 = render_char(min_chars.remove(0));
    let colon = render_char(':');
    let sec_1 = render_char(seconds_chars.remove(0));
    let sec_2 = render_char(seconds_chars.remove(0));
    izip!(min_1, min_2, colon, sec_1, sec_2)
        .map(|(min_1, min_2, colon, sec_1, sec_2)| {
            format!("{:5}{:5}{:5}{:5}{:5}", min_1, min_2, colon, sec_1, sec_2)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_char(c: char) -> Lines<'static> {
    match c {
        ':' => indoc! {r#"
    
  _ 
 (_)
    
  _ 
 (_)
"#}
        .lines(),
        '0' => indoc! {
            r#"
   ___  
  / _ \ 
 | | | |
 | | | |
 | |_| |
  \___/ 
"#
        }
        .lines(),
        '1' => indoc! {r#"
  __ 
 /_ |
  | |
  | |
  | |
  |_|
"#}
        .lines(),
        '2' => indoc! {r#"
  ___  
 |__ \ 
    ) |
   / / 
  / /_ 
 |____|
"#}
        .lines(),
        '3' => indoc! {r#"
  ____  
 |___ \ 
   __) |
  |__ < 
  ___) |
 |____/ 
"#}
        .lines(),
        '4' => indoc! {"
  _  _   
 | || |  
 | || |_ 
 |__   _|
    | |  
    |_|  
"}
        .lines(),
        '5' => indoc! {r#"
  _____ 
 | ____|
 | |__  
 |___ \ 
  ___) |
 |____/ 
"#}
        .lines(),
        '6' => indoc! {r#"
    __  
   / /  
  / /_  
 | '_ \ 
 | (_) |
  \___/ 
"#}
        .lines(),
        '7' => indoc! {r#"
  ______ 
 |____  |
     / / 
    / /  
   / /   
  /_/    
"#}
        .lines(),
        '8' => indoc! {r#"
   ___  
  / _ \ 
 | (_) |
  > _ < 
 | (_) |
  \___/ 
"#}
        .lines(),
        '9' => indoc! {r#"
   ___  
  / _ \ 
 | (_) |
  \__, |
    / / 
   /_/  
"#}
        .lines(),
        _ => panic!("something went wrong"),
    }
}
