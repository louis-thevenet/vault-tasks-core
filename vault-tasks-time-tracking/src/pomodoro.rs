use std::time::Duration;

use crate::{time_tracking_technique::TimeTrackingTechnique, State};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Pomodoro {
    focus_duration: Duration,
    break_count: usize,
    short_breaks_before_long: usize,
    short_break_duration: Duration,
    long_break_duration: Duration,
}
impl Pomodoro {
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
impl TimeTrackingTechnique for Pomodoro {
    fn switch(self, state: Option<State>, _time_spent: Duration) -> (State, Self) {
        match state {
            Some(State::Focus(_)) => {
                if self.short_breaks_before_long == self.break_count {
                    (
                        State::Break(Some(self.long_break_duration)),
                        Self {
                            break_count: 0,
                            ..self
                        },
                    )
                } else {
                    (
                        State::Break(Some(self.short_break_duration)),
                        Self {
                            break_count: self.break_count + 1,
                            ..self
                        },
                    )
                }
            }

            Some(State::Break(_)) | None => (State::Focus(Some(self.focus_duration)), self),
        }
    }
}
