
struct HealthComponent {
	entity_id: usize,
	health: usize
}

struct Component {
	entity_id: usize,
}


impl Component for HealthComponent {}

struct RenderComponent {}

impl Component for RenderComponent {}

struct World {
}

impl World {

	fn get_entities() -> Vec<Entity> {}
	fn find_components_by_type<T>(component: T) -> Vec<T> where T: Component {
		return vec![];
	}

	fn find_by_entity() -> Vec<Component> {
		return vec![];
	}

	fn insert_entity(entity: usize) {}
	fn update_entity(entity: usize) {}
	fn remove_entity(entity: usize) {}
}

trait System {
	fn on_tick(world: World) -> World {}
}

struct RenderSystem {}

impl System for RenderSystem {
		fn on_tick(world: World) {
			for component in world.find_components_by_type(RenderComponent.Type) {
				//canvas.draw(component.stroke)
			}
		}
}

struct KillSystem {}

impl System for KillSystem {
		fn on_tick(world: World) {
			for component in world.find_components_by_type(HealthComponent.Type) {
				if component.health <= 0 {
					world.remove_entity(component.entity_id)
				}
			}
		}
}

struct PhysicsSystem {
	pipeline: Pipeline
}

impl System for KillSystem {
		fn on_tick(world: World) {
			for component in world.find_components_by_type(PhysicsComponent.Type) {
				// Map components to Rapier world
				pipeline.tick(rapier_world);
				// Map Rapier world to components
				for entity in entities {
					world.update(...)
				}
			}
		}
}
