use legion::*;

pub use rusty_timer::Timer;

#[system]
pub fn update_timer(#[resource] timer: &mut Timer) {
    timer.update_delta_time();
    timer.update_fixed_time();
}
