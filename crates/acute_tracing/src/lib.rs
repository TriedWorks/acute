use acute_app::{App, Plugin};
pub use tracing;
pub use tracing::{error, event, info, trace, warn, Level};

#[derive(Default)]
pub struct TracingPlugin {}

impl Plugin for TracingPlugin {
    fn build(&self, _app: &mut App) {
        tracing_subscriber::fmt()
            .with_timer(tracing_subscriber::fmt::time::time())
            .init();

        info!("Loaded Plugin: Tracing")
    }
}

pub fn init_tracing_extern() {
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();
}
