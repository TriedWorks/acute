use acute_ecs::legion::systems::{schedule::Schedulable, SystemBuilder};

pub use rusty_timer::Timer;

pub fn update_timer() -> Box<dyn Schedulable> {
    SystemBuilder::new("UpdateTimer")
        .write_resource::<Timer>()
        .build(|_, _, timer, _| {
            timer.update_delta_time();
            timer.update_fixed_time();
        })
}
