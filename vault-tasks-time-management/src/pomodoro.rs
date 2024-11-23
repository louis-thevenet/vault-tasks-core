use std::time::Duration;

use crate::{time_management_technique::TimeManagementTechnique, State};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Pomodoro {
    focus_duration: Duration,
    break_count: usize,
    short_breaks_before_long: usize,
    short_break_duration: Duration,
    long_break_duration: Duration,
}
impl Pomodoro {
    pub fn new(
        focus_duration: Duration,
        short_breaks_before_long: usize,
        short_break_duration: Duration,
        long_break_duration: Duration,
    ) -> Self {
        Self {
            focus_duration,
            break_count: 0,
            short_breaks_before_long,
            short_break_duration,
            long_break_duration,
        }
    }

    pub fn classic_pomodoro() -> Self {
        Self {
            focus_duration: Duration::from_secs(25 * 60),
            short_break_duration: Duration::from_secs(5 * 60),
            long_break_duration: Duration::from_secs(15 * 60),
            break_count: 0,
            short_breaks_before_long: 3,
        }
    }
}
impl TimeManagementTechnique for Pomodoro {
    fn switch(&mut self, state: &Option<State>, _time_spent: Duration) -> State {
        match state {
            Some(State::Focus(_)) => {
                if self.short_breaks_before_long == self.break_count {
                    self.break_count = 0;
                    State::Break(Some(self.long_break_duration))
                } else {
                    self.break_count += 1;
                    State::Break(Some(self.short_break_duration))
                }
            }

            Some(State::Break(_)) | None => State::Focus(Some(self.focus_duration)),
        }
    }
}
