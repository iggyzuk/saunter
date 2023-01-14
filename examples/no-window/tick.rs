use std::time::Instant;

use saunter::{
    math::{self, MathError},
    tick::Tick,
};

#[derive(Debug, Clone)]
pub struct NoWindowTick {
    pub time: Instant,
    pub val: u8,
}

pub struct NoWindowTickView {
    pub val: f32,
}

impl Tick for NoWindowTick {
    fn lerp(&self, b: &Self, t: f32) -> Result<NoWindowTickView, MathError> {
        Ok(NoWindowTickView {
            val: math::lerp(self.val as f32, b.val as f32, t)?,
        })
    }

    fn get_time(&self) -> &Instant {
        &self.time
    }

    type TickView = NoWindowTickView;
}
impl NoWindowTick {
    pub fn new(time: Instant, val: u8) -> Self {
        Self { time, val }
    }
}
