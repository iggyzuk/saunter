use log::debug;
use saunter::{
    math::{self, MathError},
    tick::{Tick, Ticks},
};
use std::time::Instant;

// can't keep tick as u8 since it's used in both the simulation and the visuals.
// tick lerp could return tick view struct, downside, more structs,
// upside you get to control what is passed down to rendering.
// u8 -> f32, and don't pass unnecessary data.
// because esentially we are yet again cloning the entire tick (game) state.
#[derive(Clone, Debug)]
struct TestTick {
    tick: u8,
}

struct TestTickView {
    tick: f32,
}

impl Tick for TestTick {
    type TickView = TestTickView;

    fn lerp(&self, b: &Self, t: f32) -> Result<TestTickView, MathError> {
        Ok(TestTickView {
            tick: math::lerp(self.tick as f32, b.tick as f32, t)?,
        })
    }

    fn get_time(&self) -> &Instant {
        unimplemented!();
    }
}

#[test]
fn test_ticks_update() {
    let mut ticks = Ticks {
        last_tick: None,
        new_tick: TestTick { tick: 0 },
    };

    assert!(ticks.last_tick.is_none());
    assert_eq!(ticks.new_tick.tick, 0);

    ticks.update(TestTick { tick: 1 });
    assert!(ticks.last_tick.is_some());
    assert_eq!(ticks.last_tick.as_ref().unwrap().tick, 0);
    assert_eq!(ticks.new_tick.tick, 1);

    ticks.update(TestTick { tick: 2 });
    assert_eq!(ticks.last_tick.as_ref().unwrap().tick, 1);
    assert_eq!(ticks.new_tick.tick, 2);

    ticks.update(TestTick { tick: 3 });
    assert_eq!(ticks.last_tick.as_ref().unwrap().tick, 2);
    assert_eq!(ticks.new_tick.tick, 3);
}

#[test]
fn test_ticks_lerp() {
    let mut ticks = Ticks {
        last_tick: None,
        new_tick: TestTick { tick: 0 },
    };

    let frame_count = 10;

    for i in 1..10 {
        ticks.update(TestTick { tick: i });
        for frame in 0..frame_count {
            let time_unit = frame as f32 / frame_count as f32;
            let tick_view = ticks.lerp(time_unit).unwrap();

            let expected_lerp_val = math::lerp(
                ticks.new_tick.tick as f32,
                ticks.last_tick.as_ref().unwrap().tick as f32,
                time_unit,
            )
            .unwrap();

            assert_eq!(tick_view.tick, expected_lerp_val);
        }
    }
}
