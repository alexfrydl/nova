// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::assets::AssetLoader;
use crate::clock;
use crate::ui;
use crate::window::{self, Window};
use crate::{Engine, Renderer};
use std::ops::{Deref, DerefMut};

pub struct App {
  ui_painter: ui::Painter,
  renderer: Renderer,
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

    engine.resources_mut().insert(AssetLoader::default());

    Window::setup(&mut engine, Default::default());
    ui::setup(&mut engine);

    let renderer = Renderer::new(&engine.resources().fetch());
    let ui_painter = ui::Painter::new(&renderer);

    App {
      ui_painter,
      renderer,
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
    let cmd = self.renderer.begin();

    self.ui_painter.draw(cmd.into(), self.engine.resources());

    self.renderer.finish();
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