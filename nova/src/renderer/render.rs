// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::commands::{CommandBufferExt, Commands};
use super::pipeline::Pipeline;
use super::texture::{TextureCache, TextureId};
use super::{Allocator, Device};
use crate::graphics;
use std::iter;

pub struct Render<'a> {
  pub(super) cmd: &'a mut Commands,
  pub(super) device: &'a Device,
  pub(super) allocator: &'a mut Allocator,
  pub(super) texture_cache: &'a mut TextureCache,
}

impl<'a> Render<'a> {
  pub fn bind_pipeline(&mut self, pipeline: &Pipeline) {
    unsafe {
      self.cmd.buffer.bind_graphics_pipeline(&pipeline.raw);
    }
  }

  pub fn bind_image_cached(
    &mut self,
    pipeline: &Pipeline,
    binding: usize,
    image: &graphics::Image,
    id_cache: &mut Option<TextureId>,
  ) {
    let descriptor_set =
      self
        .texture_cache
        .get_cached(image, id_cache, &self.device, &mut self.allocator);

    unsafe {
      self.cmd.buffer.bind_graphics_descriptor_sets(
        &pipeline.raw_layout,
        binding,
        iter::once(descriptor_set),
        &[],
      );
    }
  }

  pub fn push_constant<T>(&mut self, pipeline: &Pipeline, index: usize, value: &T) {
    let range = pipeline.push_constant_range(index);

    // Convert the constant to a slice of `u32` as vulkan/gfx-hal expects.
    let constants =
      unsafe { std::slice::from_raw_parts(value as *const T as *const u32, range.len()) };

    unsafe {
      self.cmd.buffer.push_graphics_constants(
        &pipeline.raw_layout,
        gfx_hal::pso::ShaderStageFlags::VERTEX | gfx_hal::pso::ShaderStageFlags::FRAGMENT,
        range.start,
        constants,
      );
    }
  }

  pub fn draw(&mut self, vertices: std::ops::Range<u32>) {
    unsafe {
      self.cmd.buffer.draw(vertices, 0..1);
    }
  }
}