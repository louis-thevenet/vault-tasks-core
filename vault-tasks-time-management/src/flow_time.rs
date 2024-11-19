use std::time::Duration;

use color_eyre::{eyre::bail, Result};

use crate::{time_management_technique::TimeManagementTechnique, State};

pub struct FlowTime {
    break_factor: u32,
}

impl FlowTime {
    pub fn new(break_factor: u32) -> Result<Self> {
        if break_factor == 0 {
            bail!("Break Factor for FlowTime is negative")
        }
        Ok(Self { break_factor })
    }
}

impl TimeManagementTechnique for FlowTime {
    fn switch(self, state: Option<State>, time_spent: Duration) -> (State, Self) {
        match state {
            Some(State::Focus(_)) => ((State::Break(Some(time_spent / self.break_factor))), self),
            Some(State::Break(_)) | None => (State::Focus(None), self),
        }
    }
}
