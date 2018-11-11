use crate::prelude::*;
use crate::stage::Position;
use nova::time::Clock;

/// Component that stores the velocity of an entity on the stage.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Velocity {
  pub vector: Vector3<f32>,
}

// Sets the default velocity to zero.
impl Default for Velocity {
  fn default() -> Self {
    Velocity {
      vector: Vector3::zeros(),
    }
  }
}

/// System that moves each entity on the stage with a `Velocity` component by
/// adding it to its `Position` component.
pub struct Mover;

impl<'a> System<'a> for Mover {
  type SystemData = (
    ReadResource<'a, Clock>,
    ReadStorage<'a, Velocity>,
    WriteStorage<'a, Position>,
  );

  fn run(&mut self, (clock, velocities, mut positions): Self::SystemData) {
    for (velocity, position) in (&velocities, &mut positions).join() {
      position.point += velocity.vector * clock.delta_time as f32;
    }
  }
}
