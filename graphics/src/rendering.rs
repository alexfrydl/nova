use super::{Mesh, Pipeline, RenderTarget};
use gfx_hal::command::{CommandBufferFlags, RawCommandBuffer};
use gfx_hal::queue::RawCommandQueue;
use gfx_hal::{Device, Swapchain};
use smallvec::SmallVec;

pub struct BeginError;

pub fn begin(target: &mut RenderTarget) -> Result<(), BeginError> {
  let device = &target.context.device;
  let state = &mut target.states[target.current_state];

  device.wait_for_fence(&state.fence, !0);

  let swapchain = target.swapchain.as_mut().expect("swapchain was destroyed");

  state.image = swapchain
    .acquire_image(!0, gfx_hal::FrameSync::Semaphore(&state.acquire_semaphore))
    .map_err(|_| BeginError)?;

  state
    .command_buffer
    .begin(CommandBufferFlags::ONE_TIME_SUBMIT, Default::default());

  let viewport = gfx_hal::pso::Viewport {
    rect: gfx_hal::pso::Rect {
      x: 0,
      y: 0,
      w: target.width as i16,
      h: target.height as i16,
    },
    depth: 0.0..1.0,
  };

  state.command_buffer.set_viewports(0, &[viewport.clone()]);

  state.command_buffer.begin_render_pass(
    target.render_pass.raw(),
    &target.images[state.image as usize].framebuffer,
    viewport.rect,
    &[
      gfx_hal::command::ClearValue::Color(gfx_hal::command::ClearColor::Float([
        // TODO: This color should probably just be black, but using an
        //       off-black makes it more obvious when rendering is totally
        //       failing.
        0.008, 0.008, 0.012, 1.0, // “eigengrau”
      ]))
      .into(),
    ],
    gfx_hal::command::SubpassContents::Inline,
  );

  Ok(())
}

pub fn bind_pipeline(target: &mut RenderTarget, pipeline: &Pipeline) {
  let state = &mut target.states[target.current_state];

  state.command_buffer.bind_graphics_pipeline(pipeline.raw());
}

pub fn draw(target: &mut RenderTarget, mesh: &Mesh) {
  let state = &mut target.states[target.current_state];
  let mut buffers = SmallVec::<[_; 1]>::new();

  buffers.push((mesh.vertex_buffer().raw(), 0));

  state.command_buffer.bind_vertex_buffers(0, buffers);

  state
    .command_buffer
    .bind_index_buffer(gfx_hal::buffer::IndexBufferView {
      buffer: mesh.index_buffer().raw(),
      offset: 0,
      index_type: gfx_hal::IndexType::U16,
    });

  state
    .command_buffer
    .draw_indexed(0..mesh.indices(), 0, 0..1);
}

pub fn end(target: &mut RenderTarget) -> Result<(), ()> {
  let current_state = target.current_state;

  target.current_state = (current_state + 1) % target.states.len();

  let state = &mut target.states[current_state];

  state.command_buffer.finish();

  let mut cmd_buffers = SmallVec::<[_; 1]>::new();

  cmd_buffers.push(&state.command_buffer);

  unsafe {
    target.command_queue.submit_raw(
      gfx_hal::queue::RawSubmission {
        cmd_buffers,
        wait_semaphores: &[(
          &state.acquire_semaphore,
          gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
        )],
        signal_semaphores: &[&state.render_semaphore],
      },
      Some(&state.fence),
    );
  }

  let swapchain = target.swapchain.as_ref().expect("swapchain was destroyed");

  let mut swapchains = SmallVec::<[_; 1]>::new();
  let mut wait_semaphores = SmallVec::<[_; 1]>::new();

  swapchains.push((swapchain, state.image));
  wait_semaphores.push(&state.render_semaphore);

  let result = target.command_queue.present(swapchains, wait_semaphores);

  result
}
