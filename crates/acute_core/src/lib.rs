use legion::*;
use acute_app::AppBuilder;

pub trait Plugin {
    fn add(app: &mut AppBuilder) -> &mut AppBuilder;
}