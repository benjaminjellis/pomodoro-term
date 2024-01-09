use std::time::SystemTime;

use ratatui::widgets::TableState;
use tui_input::Input;

/// Default time is 25 minutes, represented here in micro-seconds
const DEFAULT_TIMER_LENGTH: i128 = 1500000000;
const DEFAULT_BREAK_TIMER_LENGTH: i128 = 300000000;

#[derive(Debug)]
struct Timer {
    running: bool,
    finished: bool,
    last_updated_time: SystemTime,
    /// Time remaining in micro-seconds
    last_recorded_time_remaining: i128,
    timer_length: i128,
}

impl Timer {
    ///
    /// # Arguments
    /// * `length` - length of the timer in micro-seconds
    fn new(length: i128) -> Self {
        Self {
            running: false,
            finished: false,
            last_recorded_time_remaining: length,
            timer_length: length,
            last_updated_time: SystemTime::now(),
        }
    }

    fn reset(&mut self) {
        *self = Self::new(self.timer_length);
    }

    fn get_length(&self) -> i128 {
        micro_seconds_to_seconds(self.timer_length)
    }

    fn start(&mut self) {
        self.running = true;
        self.last_updated_time = SystemTime::now()
    }

    fn pause(&mut self) {
        self.running = false;
    }

    /// Get the state of the timer
    ///
    /// # Returns
    /// the time remaining, the time elapsed
    pub(crate) fn get_state(&mut self) -> (i128, i128) {
        let time_remaining = self.get_time_remaining();
        let time_elapsed = micro_seconds_to_seconds(self.timer_length) - time_remaining;
        (time_remaining, time_elapsed)
    }

    fn get_time_remaining(&mut self) -> i128 {
        if self.running {
            // find out how long has elapsed since the time remaining was calculated
            let elapsed_since_last_updated =
                self.last_updated_time.elapsed().unwrap().as_micros() as i128;

            // figure out the time remaining
            let time_remaining_μs = self.last_recorded_time_remaining - elapsed_since_last_updated;

            // update the state with the time we calculate
            self.last_updated_time = SystemTime::now();

            // if the time remaining is less than or equal to 0 then the timer has finished
            // so we need to stop timing and set the time remaining to 0
            if time_remaining_μs <= 0 {
                self.pause();
                self.finished = true;
                let time_remaining_s = 0;
                self.last_recorded_time_remaining = time_remaining_s;
                time_remaining_s
            } else {
                self.last_recorded_time_remaining = time_remaining_μs;

                micro_seconds_to_seconds(time_remaining_μs)
            }
        } else {
            micro_seconds_to_seconds(self.last_recorded_time_remaining)
        }
    }
}

fn micro_seconds_to_seconds(micro_seconds: i128) -> i128 {
    micro_seconds / 1000000
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Normal,
    Insert,
    Edit,
    Break,
}

#[derive(Clone, Debug)]
pub(crate) struct Task {
    pub(crate) completed: bool,
    pub(crate) description: String,
    pub(crate) estimation: i16,
}

#[derive(Debug)]
pub(crate) struct State {
    timer: Timer,
    break_timer: Timer,
    pomodoro_no: u16,
    pub(crate) mode: Mode,
    pub(crate) input: Input,
    tasks: Vec<Task>,
    pub(crate) task_table_state: TableState,
    pub(crate) show_help_popup: bool,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::new(DEFAULT_TIMER_LENGTH),
            break_timer: Timer::new(DEFAULT_BREAK_TIMER_LENGTH),
            pomodoro_no: 1,
            mode: Mode::Normal,
            input: Input::default(),
            tasks: vec![],
            task_table_state: TableState::default(),
            show_help_popup: false,
        }
    }

    pub(crate) fn get_pomodoro_no(&self) -> u16 {
        self.pomodoro_no
    }

    pub(crate) fn start_break_timer(&mut self) {
        self.mode = Mode::Break;
        self.timer.pause();
        self.break_timer.start();
    }

    pub(crate) fn reset_break_timer(&mut self) {
        self.mode = Mode::Normal;
        self.break_timer.reset();
    }

    pub(crate) fn get_tasks(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    pub(crate) fn add_new_task(&mut self, task_description: String) {
        let new_task = Task {
            completed: false,
            description: task_description,
            estimation: 0,
        };
        self.tasks.push(new_task);
    }

    pub(crate) fn delete_current_task(&mut self) {
        if let Some(task_index) = self.task_table_state.selected() {
            self.tasks.remove(task_index);
        }
    }

    pub(crate) fn start_timer(&mut self) {
        self.timer.start();
    }

    pub(crate) fn reset_timer(&mut self) {
        self.timer.reset();
        self.pomodoro_no += 1;
    }

    pub(crate) fn pause_timer(&mut self) {
        self.timer.pause();
    }

    /// Get the state of the timer
    ///
    /// # Returns
    /// the time remaining, the time elapsed
    pub(crate) fn get_timer_state(&mut self) -> (i128, i128) {
        self.timer.get_state()
    }

    /// Get the state of the break timer
    ///
    /// # Returns
    /// the time remaining, the time elapsed
    pub(crate) fn get_break_timer_state(&mut self) -> (i128, i128) {
        self.break_timer.get_state()
    }

    #[allow(dead_code)]
    pub(crate) fn timer_is_finished(&self) -> bool {
        self.timer.finished
    }

    pub(crate) fn timer_length(&self) -> i128 {
        self.timer.get_length()
    }

    pub(crate) fn get_break_timer_length(&self) -> i128 {
        self.break_timer.get_length()
    }

    pub fn next_table_row(&mut self) {
        let i = if let Some(i) = self.task_table_state.selected() {
            if i >= self.tasks.len() - 1 {
                0
            } else {
                i + 1
            }
        } else {
            0
        };
        self.task_table_state.select(Some(i));
    }

    pub(crate) fn mark_current_task_as_completed(&mut self) {
        if let Some(i) = self.task_table_state.selected() {
            let task_item = self.tasks.get_mut(i).unwrap();
            task_item.completed = true;
        }
    }

    pub fn previous_table_row(&mut self) {
        let i = if let Some(i) = self.task_table_state.selected() {
            if i == 0 {
                self.tasks.len() - 1
            } else {
                i - 1
            }
        } else {
            0
        };
        self.task_table_state.select(Some(i));
    }

    pub(crate) fn unselect_table_item(&mut self) {
        self.task_table_state.select(None);
    }
}
