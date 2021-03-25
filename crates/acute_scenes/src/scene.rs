use acute_ecs::systems::Builder;
use acute_ecs::{Resources, Schedule, World};

pub struct Scene {
    pub world: World,
    pub schedule: Schedule,
}

impl Scene {
    pub fn new(schedule_builder: Option<Builder>) -> Self {
        let world = World::default();

        let schedule = schedule_builder.unwrap_or(Schedule::builder()).build();

        Self { world, schedule }
    }

    pub fn update(&mut self, resources: &mut Resources) {
        self.schedule.execute(&mut self.world, resources);
    }
}
