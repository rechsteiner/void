use crate::scene::Scene;
use rapier2d::pipeline::PhysicsPipeline;

pub struct Simulation {
    pipeline: PhysicsPipeline,
    pub scene: Scene,
}

impl Simulation {
    pub fn new(scene: Scene) -> Simulation {
        let pipeline = PhysicsPipeline::new();
        Simulation { pipeline, scene }
    }

    pub fn next_state(&mut self) {
        self.pipeline.step(
            &self.scene.gravity,
            &self.scene.integration_parameters,
            &mut self.scene.broad_phase,
            &mut self.scene.narrow_phase,
            &mut self.scene.bodies,
            &mut self.scene.colliders,
            &mut self.scene.joints,
            &(),
            &(),
        )
    }
}
