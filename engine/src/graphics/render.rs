// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod framebuffer;
mod pass;

use super::{CommandPool, Commands, Fence, PipelineStage, QueueSubmission, Queues, Semaphore};
use crate::ecs;
use crate::utils::Droppable;
use crate::window;

pub use self::framebuffer::*;
pub use self::pass::*;

pub struct Renderer {
  pass: Pass,
  presenter: window::Presenter,
  fence: Fence,
  backbuffer_semaphore: Semaphore,
  render_semaphore: Semaphore,
  framebuffer: Droppable<Framebuffer>,
  commands: Commands,
}

impl Renderer {
  pub fn new(res: &ecs::Resources) -> Self {
    let presenter = window::Presenter::new(res);

    let device = res.fetch();

    let fence = Fence::new(&device);
    let backbuffer_semaphore = Semaphore::new(&device);
    let render_semaphore = Semaphore::new(&device);

    let commands = {
      let queues = res.fetch::<Queues>();

      let pool = CommandPool::new(
        &device,
        queues
          .get_graphics_queue()
          .expect("The device does not suppord graphics commands."),
      );

      pool.acquire()
    };

    Renderer {
      pass: Pass::new(&device),
      presenter,
      fence,
      backbuffer_semaphore,
      render_semaphore,
      framebuffer: Droppable::dropped(),
      commands,
    }
  }

  pub fn render(&mut self, res: &ecs::Resources) {
    self.fence.wait_and_reset();

    self.framebuffer.take();
    self.presenter.begin(&self.backbuffer_semaphore);
    self.framebuffer = Framebuffer::new(&self.pass, self.presenter.backbuffer()).into();

    self.commands.begin();

    self
      .commands
      .begin_render_pass(&self.pass, &self.framebuffer);

    self.commands.finish_render_pass();
    self.commands.finish();

    {
      let mut queues = res.fetch_mut::<Queues>();

      queues.submit(QueueSubmission {
        commands: &self.commands,
        wait_semaphores: Some((
          &self.backbuffer_semaphore,
          PipelineStage::COLOR_ATTACHMENT_OUTPUT,
        )),
        signal_semaphores: Some(&self.render_semaphore),
        fence: Some(&self.fence),
      });
    }

    self.presenter.finish(res, Some(&self.render_semaphore));
  }
}
