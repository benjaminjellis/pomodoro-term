use std::time::SystemTime;

/// Default time is 25 minutes, represented here in micro-seconds
const DEFAULT_TIMER_LENGTH: i128 = 60000000;
// const DEFAULT_TIMER_LENGTH: i128 = 1500000000;

#[derive(Debug)]
struct Timer {
    running: bool,
    finished: bool,
    last_updated_time: SystemTime,
    /// Time remaining in micro-seconds
    last_recorded_time_remaining: i128,
    next_colour: TimerColour,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum TimerColour {
    Red,
    White,
}

impl Timer {
    fn new() -> Self {
        Self {
            running: false,
            finished: false,
            last_recorded_time_remaining: DEFAULT_TIMER_LENGTH,
            last_updated_time: SystemTime::now(),
            next_colour: TimerColour::Red,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn start(&mut self) {
        self.running = true;
        self.last_updated_time = SystemTime::now()
    }

    fn pause(&mut self) {
        self.running = false;
    }

    pub(crate) fn get_time_remaining(&mut self) -> i128 {
        if self.running {
            // find out how long has elapsed since the time remaining was calculated
            let elapsed_since_last_upated =
                self.last_updated_time.elapsed().unwrap().as_micros() as i128;

            // figure out the time remaining
            let time_remaining_μs = self.last_recorded_time_remaining - elapsed_since_last_upated;

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

                time_remaining_μs / 1000000
            }
        } else {
            self.last_recorded_time_remaining / 1000000
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;

    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        dbg!(&timer);

        timer.start();
        std::thread::sleep(std::time::Duration::from_secs(2));
        timer.get_time_remaining();
        dbg!(&timer);
        std::thread::sleep(std::time::Duration::from_secs(2));
        let ts = timer.get_time_remaining();
        dbg!(ts);

        dbg!(&timer);
        panic!("I just want to see results");
    }
}

#[derive(Debug)]
pub(crate) struct State {
    timer: Timer,
    pomodoro_no: u16,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::new(),
            pomodoro_no: 1,
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

    pub(crate) fn time_remaining(&mut self) -> i128 {
        self.timer.get_time_remaining()
    }

    pub(crate) fn timer_is_finished(&mut self) -> bool {
        self.timer.finished
    }

    pub(crate) fn next_timer_colour(&mut self) -> TimerColour {
        match self.timer.next_colour {
            TimerColour::Red => {
                self.timer.next_colour = TimerColour::White;
            }
            TimerColour::White => {
                self.timer.next_colour = TimerColour::Red;
            }
        }
        self.timer.next_colour
    }
}
