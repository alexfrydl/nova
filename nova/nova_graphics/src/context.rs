// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub use gfx_hal::error::DeviceCreationError;
use gfx_hal::{Instance as _, PhysicalDevice as _};

/// Central state of the graphics library.
///
/// There should be only one context per device in an application.
///
/// This structure is cloneable and all clones refer to the same context. When
/// all clones are dropped, the allocated device resources will be destroyed.
#[derive(Clone)]
pub struct Context(Arc<ContextInner>);

pub struct ContextInner {
  // Fields must be in this order so that they are dropped in this order.
  pub(crate) memory: Memory,
  pub(crate) queues: cmd::Queues,
  pub(crate) device: backend::Device,
  pub(crate) adapter: backend::Adapter,
  pub(crate) backend: backend::Instance,
  pub(crate) logger: log::Logger,
}

impl Context {
  /// Creates a new context using the best available graphics device.
  pub fn new(logger: &log::Logger) -> Result<Self, NewContextError> {
    // Instantiate the backend.
    let backend = backend::Instance::create("nova", 1);

    log::debug!(logger, "instantiated graphics backend"; "backend" => backend::NAME);

    // Get and log all available adapters.
    let mut adapters = backend.enumerate_adapters();

    for adapter in &adapters {
      log::debug!(logger,
        "found graphics adapter";
        "type" => log::Debug(&adapter.info.device_type),
        "type_id" => adapter.info.device,
        "vendor_id" => adapter.info.vendor,
        "name" => &adapter.info.name,
      );
    }

    // Sort adapters from most powerful type to least.
    adapters.sort_by_key(|adapter| match adapter.info.device_type {
      gfx_hal::adapter::DeviceType::DiscreteGpu => 3,
      gfx_hal::adapter::DeviceType::IntegratedGpu => 2,
      gfx_hal::adapter::DeviceType::Cpu => 1,
      _ => 0,
    });

    // Select the first adapter in the sorted list.
    let adapter = adapters
      .into_iter()
      .next()
      .ok_or(NewContextError::NoDevice)?;

    log::debug!(logger,
      "selected graphics adapter";
      "type" => log::Debug(&adapter.info.device_type),
      "type_id" => adapter.info.device,
      "vendor_id" => adapter.info.vendor,
      "name" => &adapter.info.name,
    );

    // Get all queue families.
    let queue_families = adapter.queue_families.clone();

    // Open the physical device and one queue in every family.
    let queue_requests = queue_families
      .iter()
      .map(|family| {
        use gfx_hal::QueueFamily as _;

        log::debug!(logger,
          "found graphics queue family";
          "max_queues" => family.max_queues(),
          "type" => log::Debug(family.queue_type()),
          "id" => family.id().0,
        );

        (family, &[1.0][..])
      })
      .collect::<Vec<_>>();

    let gfx_hal::Gpu { device, queues } = unsafe {
      use gfx_hal::PhysicalDevice as _;

      adapter
        .physical_device
        .open(&queue_requests[..], gfx_hal::Features::empty())?
    };

    log::debug!(logger, "opened graphics device");

    // Extract backend command queues into a `Queues` struct.
    let queues = cmd::Queues::new(queue_families, queues);

    // Encapsulate memory state based on the memory properties of the adapter.
    let memory = Memory::new(adapter.physical_device.memory_properties());

    Ok(Self(Arc::new(ContextInner {
      backend,
      adapter,
      device,
      queues,
      memory,
      logger: logger.clone(),
    })))
  }

  /// Returns a reference to the command queues of the graphics device.
  pub fn queues(&self) -> &cmd::Queues {
    &self.0.queues
  }

  /// Gets a reference to the `log::Logger` of the graphics context.
  pub fn logger(&self) -> &log::Logger {
    &self.0.logger
  }

  /// Waits for the graphics device to be idle, meaning no command buffers are
  /// being executed.
  pub fn wait_idle(&self) {
    let _ = self.0.device.wait_idle();
  }

  /// Returns an `Allocator` structure for allocating and freeing memory.
  pub fn allocator(&self) -> Allocator {
    Allocator::new(self)
  }
}

impl ops::Deref for Context {
  type Target = ContextInner;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// Error occurring during the creation of a new graphics context.
#[derive(Debug)]
pub enum NewContextError {
  /// There is no suitable graphics device.
  NoDevice,
  /// An error occurred during device creation.
  DeviceCreation(DeviceCreationError),
}

impl std::error::Error for NewContextError {}

impl From<DeviceCreationError> for NewContextError {
  fn from(cause: DeviceCreationError) -> Self {
    NewContextError::DeviceCreation(cause)
  }
}

impl fmt::Display for NewContextError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      NewContextError::NoDevice => write!(f, "no suitable graphics device"),
      NewContextError::DeviceCreation(cause) => {
        write!(f, "failed to create graphics device: {}", cause)
      }
    }
  }
}