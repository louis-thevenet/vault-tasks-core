use std::time::Duration;

use time_tracking_technique::TimeTrackingTechnique;
mod flow_time;
mod pomodoro;
mod time_tracking_technique;

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Focus(Option<Duration>),
    Break(Option<Duration>),
}
#[derive(Debug)]
/// Provides tracking methods using a generic `TimeTrackingTechnique`
pub struct TimeTrackingEngine<T: TimeTrackingTechnique> {
    pub mode: T,
    pub state: Option<State>,
}

impl<T: TimeTrackingTechnique> TimeTrackingEngine<T> {
    /// Creates a new [`TimeTrackingEngine<T>`].
    pub fn new(technique: T) -> Self {
        Self {
            mode: technique,
            state: None,
        }
    }

    /// Returns the next state of the time tracking engine.
    /// # Argument
    /// - `time_spent: Duration`: The duration of the previous session.
    /// # Returns
    /// - `Option<Duration>`: Whether there is or not an explicit duration for the next session
    /// - `TimeTrackingEngine<T>`: The next state of the engine
    pub fn switch(self, time_spent: Duration) -> (Option<Duration>, Self) {
        let (new_state, new_mode) = self.mode.switch(self.state, time_spent);

        let duration = match new_state {
            State::Focus(d) | State::Break(d) => d,
        };

        (
            duration,
            Self {
                state: Some(new_state),
                mode: new_mode,
            },
        )
    }
}

