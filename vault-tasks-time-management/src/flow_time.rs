use std::time::Duration;

use color_eyre::{eyre::bail, Result};

use crate::{time_management_technique::TimeManagementTechnique, State};

#[derive(Debug)]
pub struct FlowTime {
    break_factor: u32,
}

impl FlowTime {
    /// Creates a new `FlowTime` object from a break time factor.
    /// After the first focus time (t), break time will be computed as t / `break_factor`
    /// # Errors
    /// Will return an error if `break_factor` <= 0
    pub fn new(break_factor: u32) -> Result<Self> {
        if break_factor == 0 {
            bail!("Break Factor for FlowTime is negative")
        }
        Ok(Self { break_factor })
    }
}

impl TimeManagementTechnique for FlowTime {
    fn switch(&mut self, state: &Option<State>, time_spent: Duration) -> State {
        match state {
            Some(State::Focus(_)) => State::Break(Some(time_spent / self.break_factor)),
            Some(State::Break(_)) | None => State::Focus(None),
        }
    }
}
