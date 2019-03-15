// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{assets, graphics};
use crate::clock;
use crate::ecs;
use crate::engine::Engine;
use crate::renderer::Renderer;
use crate::ui;
use crate::window::{self, Window};
use std::ops::{Deref, DerefMut};

pub struct App {
  ui_painter: ui::Painter,
  renderer: Renderer,
  gamepad_updater: input::gamepad::UpdateGamepad,
  keyboard_updater: input::keyboard::UpdateKeyboard,
  engine: Engine,
}

impl Default for App {
  fn default() -> Self {
    App::new()
  }
}

impl App {
  pub fn new() -> Self {
    let mut engine = Engine::new();

    assets::setup(&mut engine, Default::default());
    graphics::setup(&mut engine);
    Window::setup(&mut engine, Default::default());
    ui::setup(&mut engine);

    let mut renderer = Renderer::new(&engine);
    let ui_painter = ui::Painter::new(&mut renderer);

    let mut gamepad_updater = input::gamepad::UpdateGamepad::new();
    let mut keyboard_updater = input::keyboard::UpdateKeyboard::new();

    ecs::System::setup(&mut gamepad_updater, engine.resources_mut());
    ecs::System::setup(&mut keyboard_updater, engine.resources_mut());

    App {
      ui_painter,
      renderer,
      gamepad_updater,
      keyboard_updater,
      engine,
    }
  }

  pub fn run(mut self) {
    // Previous time storage for delta time calculation.
    let mut previous_instant = None;

    // Register an event reader for window events.
    let mut event_reader = {
      let mut events = self.engine.resources().fetch_mut::<window::Events>();

      events.channel_mut().register_reader()
    };

    loop {
      // Update input before each frame.
      ecs::System::run(
        &mut self.gamepad_updater,
        ecs::SystemData::fetch(self.engine.resources()),
      );

      ecs::System::run(
        &mut self.keyboard_updater,
        ecs::SystemData::fetch(self.engine.resources()),
      );

      // Tick the engine once.
      self.tick(clock::DeltaTime::SincePrevious(&mut previous_instant));

      // Exit if the player tried to close the window.
      {
        let events = self.engine.resources().fetch::<window::Events>();
        let mut close_requested = false;

        for event in events.channel().read(&mut event_reader) {
          if let window::Event::CloseRequested = event {
            close_requested = true;
          }
        }

        if close_requested {
          break;
        }
      }

      // Otherwise render a frame.
      self.render();
    }

    // Clean up device resources.
    self.ui_painter.destroy(self.renderer.device());
    self.renderer.destroy();
  }

  pub fn render(&mut self) {
    let mut render = self.renderer.begin();

    self.ui_painter.draw(&mut render, self.engine.resources());

    self.renderer.finish(self.engine.resources());
  }
}

impl Deref for App {
  type Target = Engine;

  fn deref(&self) -> &Engine {
    &self.engine
  }
}

impl DerefMut for App {
  fn deref_mut(&mut self) -> &mut Engine {
    &mut self.engine
  }
}
