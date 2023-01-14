use saunter::math::{self, MathError};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct WinitTick {
    time: Instant,
    pub val: f32,
}

pub struct WinitTickView {
    pub val: f32,
}

impl saunter::tick::Tick for WinitTick {
    fn lerp(&self, b: &Self, t: f32) -> Result<WinitTickView, MathError> {
        Ok(WinitTickView {
            val: math::lerp(self.val, b.val, t)?,
        })
    }

    fn get_time(&self) -> &Instant {
        &self.time
    }

    type TickView = WinitTickView;
}

impl WinitTick {
    pub fn new(time: Instant, val: f32) -> Self {
        WinitTick { time, val }
    }
}
