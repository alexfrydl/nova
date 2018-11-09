use super::*;
use crate::graphics::device::{Fence, Semaphore};
use crate::graphics::rendering::{CommandBuffer, CommandBufferKind, CommandPool};
use crate::graphics::window::Framebuffer;
use crate::utils::{Chain, SmallVec};
use std::sync::Arc;

pub const FRAME_COUNT: usize = 3;

pub struct Renderer {
  pass: Arc<RenderPass>,
  command_pool: Arc<CommandPool>,
  frames: Chain<Frame>,
}

struct Frame {
  semaphore: Semaphore,
  commands: SmallVec<[CommandBuffer; 1]>,
}

impl Renderer {
  pub fn new(queue: &Arc<device::Queue>) -> Self {
    let device = queue.device();

    Renderer {
      pass: RenderPass::new(device),
      command_pool: CommandPool::new(&queue),
      frames: Chain::allocate(FRAME_COUNT, |_| Frame {
        semaphore: Semaphore::new(device),
        commands: SmallVec::new(),
      }),
    }
  }

  pub fn pass(&self) -> &Arc<RenderPass> {
    &self.pass
  }

  pub fn render(
    &mut self,
    framebuffer: &Framebuffer,
    commands: impl IntoIterator<Item = CommandBuffer>,
    wait_for: &Semaphore,
    fence: Option<&Fence>,
  ) -> &Semaphore {
    let mut primary = CommandBuffer::new(&self.command_pool, CommandBufferKind::Primary);

    primary.begin();
    primary.begin_pass(self.pass(), &framebuffer);
    primary.execute_commands(commands);
    primary.finish();

    let frame = self.frames.next();

    frame.commands.clear();
    frame.commands.push(primary);

    unsafe {
      self.command_pool.queue().raw_mut().submit_raw(
        gfx_hal::queue::RawSubmission {
          cmd_buffers: frame.commands.iter().map(CommandBuffer::raw),
          wait_semaphores: &[(
            wait_for.raw(),
            gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
          )],
          signal_semaphores: &[frame.semaphore.raw()],
        },
        fence.map(Fence::raw),
      );
    }

    &frame.semaphore
  }
}
