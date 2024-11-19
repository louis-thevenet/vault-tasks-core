use std::time::Duration;

use crate::State;

pub trait TimeManagementTechnique {
    fn switch(self, state: Option<State>, time_spent: Duration) -> (State, Self);
}
