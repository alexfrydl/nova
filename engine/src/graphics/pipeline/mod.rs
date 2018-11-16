// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod descriptor;
pub mod shader;
pub mod vertex;

mod builder;

pub use self::builder::PipelineBuilder;
pub use self::descriptor::{Descriptor, DescriptorLayout, DescriptorPool, DescriptorSet};
pub use self::shader::{Shader, ShaderKind, ShaderSet};
pub use self::vertex::{VertexAttribute, VertexData};
pub use gfx_hal::pso::PipelineStage as Stage;

use super::backend;
use super::hal::prelude::*;
use super::Device;
use crate::utils::Droppable;
use std::ops::Range;
use std::sync::Arc;

/// A graphics pipeline that configures rendering with input descriptors, push
/// constants, and shaders.
pub struct Pipeline {
  device: Arc<Device>,
  raw: Droppable<(backend::GraphicsPipeline, backend::PipelineLayout)>,
  push_constants: Vec<(hal::pso::ShaderStageFlags, Range<u32>)>,
  descriptor_layouts: Vec<Arc<DescriptorLayout>>,
  shaders: ShaderSet,
}

impl Pipeline {
  /// Creates a new pipeline from the returned [`PipelineBuilder`].
  pub fn new() -> PipelineBuilder {
    PipelineBuilder::default()
  }

  /// Gets a reference to the raw backend layout of the pipeline.
  pub fn raw_layout(&self) -> &backend::PipelineLayout {
    &self.raw.1
  }

  /// Gets the raw constant range and shader stages of a defined push constant
  /// by index.
  pub fn raw_push_constant(&self, index: usize) -> (hal::pso::ShaderStageFlags, Range<u32>) {
    self.push_constants[index].clone()
  }

  /// Gets a list of references to the descriptor layouts defined in the
  /// pipeline.
  pub fn descriptor_layouts(&self) -> &[Arc<DescriptorLayout>] {
    &self.descriptor_layouts
  }

  /// Gets a reference to the shader set used in the pipeline.
  pub fn shaders(&self) -> &ShaderSet {
    &self.shaders
  }
}

// Implement `AsRef` to expose the raw backend graphics pipeline.
impl AsRef<backend::GraphicsPipeline> for Pipeline {
  fn as_ref(&self) -> &backend::GraphicsPipeline {
    &self.raw.0
  }
}

/// Implement `Drop` to destroy the raw backend resources.
impl Drop for Pipeline {
  fn drop(&mut self) {
    let device = self.device.raw();

    if let Some((pipeline, layout)) = self.raw.take() {
      device.destroy_pipeline_layout(layout);
      device.destroy_graphics_pipeline(pipeline);
    }
  }
}