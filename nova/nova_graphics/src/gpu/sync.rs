// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::Gpu;
use crate::Backend;
use gfx_hal::Device as _;

pub type HalSemaphore = <Backend as gfx_hal::Backend>::Semaphore;
type HalFence = <Backend as gfx_hal::Backend>::Fence;

#[derive(Debug)]
pub struct Semaphore(HalSemaphore);

impl Semaphore {
  pub fn new(gpu: &Gpu) -> Self {
    Self(gpu.device.create_semaphore().unwrap())
  }

  pub fn destroy(self, gpu: &Gpu) {
    unsafe {
      gpu.device.destroy_semaphore(self.0);
    }
  }

  pub fn as_hal(&self) -> &HalSemaphore {
    &self.0
  }
}

#[derive(Debug)]
pub struct Fence(HalFence);

impl Fence {
  pub fn new(gpu: &Gpu) -> Self {
    Self(gpu.device.create_fence(true).unwrap())
  }

  pub fn wait_and_reset(&self, gpu: &Gpu) {
    unsafe {
      gpu
        .device
        .wait_for_fence(&self.0, !0)
        .expect("Could not wait for fence");

      gpu
        .device
        .reset_fence(&self.0)
        .expect("Could not reset fence");
    }
  }

  pub fn destroy(self, gpu: &Gpu) {
    unsafe {
      gpu.device.destroy_fence(self.0);
    }
  }

  pub fn as_hal(&self) -> &HalFence {
    &self.0
  }
}