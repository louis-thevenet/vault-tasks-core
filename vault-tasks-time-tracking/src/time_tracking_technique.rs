use std::time::Duration;

use crate::State;

pub trait TimeTrackingTechnique {
    fn switch(self, state: Option<State>, time_spent: Duration) -> (State, Self);
}
