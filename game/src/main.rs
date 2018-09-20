// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate nova;

use nova::prelude::*;

struct Game {
  window: platform::Window,
  canvas: graphics::Canvas,
}

impl Application for Game {
  // Set up world and systems.
  fn setup<'a, 'b>(&mut self, world: &mut World, systems: &mut DispatcherBuilder<'a, 'b>) {
    assets::setup(world);
    graphics::setup(world);
    input::setup(world);
    time::setup(world);

    stage::setup(world, systems);
    stage::draw::setup(world, systems);

    unstable::setup(world, systems);

    setup(world).expect("could not load assets");
  }

  // Perform early update logic before systems are run.
  fn before_update(&mut self, world: &mut World) {
    self.window.update();

    if self.window.was_resized() {
      self.canvas.resize(self.window.size());
    }

    time::tick(world);
    input::update(world, &mut self.window);
  }

  // Perform update logic after systems are run.
  fn update(&mut self, world: &mut World) {
    self
      .canvas
      .clear(graphics::Color::new(0.53, 0.87, 0.52, 1.0));

    stage::draw(world, &mut self.canvas);

    self.canvas.present();
  }

  // Return `false` to exit the game.
  fn is_running(&self) -> bool {
    !self.window.is_closing()
  }
}

/// Main entry point of the program.
pub fn main() {
  let window = platform::Window::new("nova-game");
  let canvas = graphics::Canvas::new(&window);

  let game = Game { window, canvas };

  game.run();
}

/// Set up program test world.
fn setup<'a, 'b>(world: &mut World) -> Result<(), assets::Error> {
  // Load actor templates.
  let hero_template = assets::load(world, &assets::PathBuf::from("hero-f/actor.yml"))?;

  let monster_template = assets::load(
    world,
    &assets::PathBuf::from("004-fire-salamander/actor.yml"),
  )?;

  // Create actor entities.
  let hero = stage::actors::build_entity(Arc::new(hero_template), world.create_entity()).build();

  let _monster = stage::actors::build_entity(Arc::new(monster_template), world.create_entity())
    .with(stage::Position {
      point: Point3::new(32.0, 24.0, 0.0),
    })
    .build();

  // Set the camera target to the hero.
  world.write_resource::<stage::Camera>().set_target(hero);

  // Set the hero to be input controlled.
  world
    .write_storage()
    .insert(hero, unstable::InputControlled)?;

  // Set the object shadow texture.
  let circle = assets::load(world, &assets::PathBuf::from("circle.png")).ok();

  world
    .write_resource::<stage::objects::draw::Settings>()
    .shadow_texture = circle;

  Ok(())
}
