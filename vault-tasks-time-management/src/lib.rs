use std::time::Duration;

use time_management_technique::TimeManagementTechnique;

pub mod flow_time;
pub mod pomodoro;
pub mod time_management_technique;

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Focus(Option<Duration>),
    Break(Option<Duration>),
}
#[derive(Debug)]
/// Provides tracking methods using a generic `TimeTrackingTechnique`
pub struct TimeManagementEngine<T: TimeManagementTechnique> {
    pub mode: T,
    pub state: Option<State>,
}

impl<T: TimeManagementTechnique> TimeManagementEngine<T> {
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
    /// - `TimeManagementEngine<T>`: The next state of the engine
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

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::{flow_time::FlowTime, pomodoro::Pomodoro, State, TimeManagementEngine};

    use std::time::Duration;

    #[test]
    fn test_run_pomodoro() {
        let time_tracker = TimeManagementEngine::new(Pomodoro::classic_pomodoro());
        let focus_time = Duration::from_secs(60 * 25);
        let short_break_time = Duration::from_secs(60 * 5);
        assert_eq!(time_tracker.mode, Pomodoro::classic_pomodoro());
        assert!(time_tracker.state.is_none());

        let (to_spend_opt, time_tracker) = time_tracker.switch(Duration::default());
        assert!(time_tracker.state.is_some());
        assert_eq!(
            time_tracker.state.clone().unwrap(),
            State::Focus(Some(focus_time))
        );
        assert!(to_spend_opt.is_some());
        assert_eq!(focus_time, to_spend_opt.unwrap());

        let (to_spend_opt, time_tracker) = time_tracker.switch(Duration::default());
        assert!(time_tracker.state.is_some());
        assert_eq!(
            time_tracker.state.clone().unwrap(),
            State::Break(Some(short_break_time))
        );
        assert!(to_spend_opt.is_some());
        assert_eq!(short_break_time, to_spend_opt.unwrap());
    }
    #[test]
    fn test_full_run_pomodoro() {
        let mut time_tracker = TimeManagementEngine::new(Pomodoro::classic_pomodoro());
        assert_eq!(time_tracker.mode, Pomodoro::classic_pomodoro());
        assert!(time_tracker.state.is_none());

        let mut to_spend_opt = None;

        for _i in 0..2 {
            // (Focus -> Break) 3 times
            for _j in 0..(3 * 2) {
                let (to_spend_opt2, time_tracker2) = time_tracker.switch(Duration::from_secs(0));
                time_tracker = time_tracker2;
                to_spend_opt = to_spend_opt2;
            }

            assert!(time_tracker.state.is_some());
            assert_eq!(
                time_tracker.state.clone().unwrap(),
                State::Break(to_spend_opt)
            );
        }
    }
    #[test]
    fn test_run_flowtime() -> Result<()> {
        let break_factor = 5;
        let time_tracker = TimeManagementEngine::new(FlowTime::new(break_factor)?);

        assert!(time_tracker.state.is_none());

        let focus_time = Duration::from_secs(25);
        let break_time = focus_time / break_factor;

        let (to_spend_opt, time_tracker) = time_tracker.switch(Duration::from_secs(0));

        assert!(time_tracker.state.is_some());
        assert_eq!(time_tracker.state.clone().unwrap(), State::Focus(None));
        assert!(to_spend_opt.is_none());

        let (to_spend_opt, time_tracker) = time_tracker.switch(focus_time);
        assert!(time_tracker.state.is_some());
        assert_eq!(
            time_tracker.state.clone().unwrap(),
            State::Break(Some(break_time))
        );
        assert!(to_spend_opt.is_some());
        assert_eq!(break_time, to_spend_opt.unwrap());
        Ok(())
    }
}
