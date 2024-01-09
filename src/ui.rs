use crate::{
    art,
    state::{Mode, State},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Row, Table},
    Frame,
};

const CHECK_MARK: &str = "✅";

pub(crate) fn ui(f: &mut Frame, state: &mut State) {
    let size = f.size();
    // header
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)])
        .split(size);

    f.render_widget(
        Paragraph::new(format!("{:5}", art::POMODORO_LOGO))
            .white()
            .alignment(Alignment::Center),
        chunks[0],
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(75)])
        .split(chunks[1]);

    // timer
    render_timer(f, state, chunks[0]);

    // tasks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[1]);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = state.input.visual_scroll(width as usize);

    let second_line_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[0]);

    let task_input = Paragraph::new(state.input.value())
        .style(match state.mode {
            Mode::Normal | Mode::Edit | Mode::Break => Style::default(),
            Mode::Insert => Style::default().fg(Color::LightMagenta),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Task Input"));

    f.render_widget(task_input, second_line_chunks[0]);

    let pomodoro_no = state.get_pomodoro_no();
    let pomodoro_no_paragraph =
        Paragraph::new(format!("{pomodoro_no}")).alignment(Alignment::Center);
    let pomodoro_no_block = Block::default().borders(Borders::ALL).title("Pomodoro No");
    f.render_widget(
        pomodoro_no_paragraph.block(pomodoro_no_block),
        second_line_chunks[1],
    );

    let tasks: Vec<_> = state
        .get_tasks()
        .into_iter()
        .map(|task| {
            Row::new(vec![
                Line::from(if task.completed { CHECK_MARK } else { "" })
                    .alignment(Alignment::Center),
                Line::from(task.description),
                Line::from(task.estimation.to_string()),
            ])
        })
        .collect();

    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ];

    let tasks_style = match state.mode {
        Mode::Normal | Mode::Insert | Mode::Break => Style::default(),
        Mode::Edit => Style::default().fg(Color::LightMagenta),
    };

    let tasks_table = Table::new(tasks, widths)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .header(
            Row::new(vec![
                Line::from("Completed").alignment(Alignment::Center),
                Line::from("Task"),
                Line::from("Est."),
            ])
            .style(Style::new().bold().fg(Color::LightGreen))
            .bottom_margin(1),
        )
        .style(tasks_style)
        .highlight_style(Style::new().reversed());

    f.render_stateful_widget(tasks_table, chunks[1], &mut state.task_table_state);

    if state.show_help_popup {
        render_help_popup(f, size);
    }

    if state.mode == Mode::Break {
        render_break_popup(f, state, size);
    }
}

fn render_break_popup(f: &mut Frame, state: &mut State, size: Rect) {
    let block = Block::default().title("Break").borders(Borders::ALL);

    let (break_time_remaining, break_time_elapsed) = state.get_break_timer_state();

    let break_timer_length = state.get_break_timer_length();

    let percentage = (break_time_elapsed as f64 / break_timer_length as f64 * 100.0) as u16;
    let label = build_timer_label(break_time_elapsed, break_time_remaining, break_timer_length);
    let label_span = Span::styled(label, Style::new().white().bold());
    let break_timer = Gauge::default()
        .gauge_style(Style::default().light_magenta())
        .block(block)
        .percent(percentage)
        .label(label_span);

    // let break_timer = Gauge::default()
    let area = centered_rect(60, 50, size);
    f.render_widget(Clear, size); //this clears out the background
    f.render_widget(break_timer, area);
}

fn render_help_popup(f: &mut Frame, size: Rect) {
    let block = Block::default().title("Help").borders(Borders::ALL);
    let area = centered_rect(60, 50, size);
    let help_rows = vec![
        Row::new(vec!["i", "enter 'insert' mode to create a new task"]),
        Row::new(vec!["e", "enter 'edit' mode to edit a task"]),
        Row::new(vec!["j/↓", "move down a row"]),
        Row::new(vec!["h/↑", "move up a row"]),
        Row::new(vec!["m", "mark a task as done"]),
        Row::new(vec!["d", "delete an item"]),
        Row::new(vec!["?", "show help popup"]),
        Row::new(vec!["q", "quit"]),
        Row::new(vec!["r", "reset and advanced to next pomodoro"]),
        Row::new(vec!["s", "start/resume timer"]),
        Row::new(vec!["p", "pause timer"]),
        Row::new(vec!["Esc", "return to 'normal' mode"]),
    ];
    let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];

    let table = Table::new(help_rows, widths)
        .header(
            Row::new(vec![Line::from("Key"), Line::from("Action")])
                .style(Style::new().bold().fg(Color::LightGreen)),
        )
        .block(block);

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(table, area);
}

fn build_timer_label(time_elapsed: i128, time_remaining: i128, timer_length: i128) -> String {
    format!(
        "{}/{} ({})",
        format_seconds(time_elapsed),
        format_seconds(timer_length),
        format_seconds(time_remaining)
    )
}

fn render_timer(f: &mut Frame, state: &mut State, chunk: Rect) {
    // TODO: figure out why this blinking doesn't work
    let timer_block = Block::default().borders(Borders::ALL).title("Timer");
    let (time_remaining, time_elapsed) = state.get_timer_state();
    let timer_length = state.timer_length();
    let percentage = (time_elapsed as f64 / timer_length as f64 * 100.0) as u16;

    let label = build_timer_label(time_elapsed, time_remaining, timer_length);
    let label_span = Span::styled(label, Style::new().white().bold());
    let timer = Gauge::default()
        .gauge_style(Style::default().light_magenta())
        .block(timer_block)
        .percent(percentage)
        .label(label_span);

    f.render_widget(timer, chunk);
}

fn format_seconds(seconds: i128) -> String {
    let remaining_minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:0>2}:{:0>2}", remaining_minutes, remaining_seconds)
}

/// helper function to create a centred rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
