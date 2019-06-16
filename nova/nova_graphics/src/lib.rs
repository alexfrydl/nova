// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod backend;
mod color;
mod queues;

pub use self::color::Color;
pub use self::queues::{QueueId, Queues};
pub use gfx_hal::error::DeviceCreationError;

use gfx_hal::Instance as _;
use nova_log as log;
use std::fmt;

/// A self-contained instance of the graphics library.
///
/// There should only be one instance per application.
pub struct Instance {
  // Fields must be in this order so that they are dropped in this order.
  queues: Queues,
  _device: backend::Device,
  _adapter: backend::Adapter,
  _backend: backend::Instance,
}

impl Instance {
  /// Creates a new instance of the graphics library.
  pub fn new(logger: &log::Logger) -> Result<Self, NewInstanceError> {
    // Instantiate the backend.
    let backend = backend::Instance::create("nova", 1);

    log::debug!(logger, "instantiated graphics backend"; "backend" => backend::NAME);

    // Get and log all available adapters.
    let mut adapters = backend.enumerate_adapters();

    for adapter in &adapters {
      log::debug!(logger,
        "found graphics adapter";
        "type" => format!("{:?}", adapter.info.device_type),
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
      .ok_or(NewInstanceError::NoDevice)?;

    log::debug!(logger,
      "selected graphics adapter";
      "type" => format!("{:?}", adapter.info.device_type),
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
          "type" => format!("{:?}", family.queue_type()),
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

    // Extract backend queues into a `Queues` struct.
    let queues = Queues::new(queue_families, queues);

    Ok(Instance {
      queues,
      _device: device,
      _backend: backend,
      _adapter: adapter,
    })
  }

  /// Gets a `Queues` structure for accessing the graphics, compute, and
  /// transfer command queues on the device.
  pub fn queues(&self) -> &Queues {
    &self.queues
  }
}

/// An error occurring during new instance creation.
#[derive(Debug)]
pub enum NewInstanceError {
  /// There is no suitable graphics device.
  NoDevice,
  /// An error occurred during device creation.
  DeviceCreation(DeviceCreationError),
}

impl std::error::Error for NewInstanceError {}

impl From<DeviceCreationError> for NewInstanceError {
  fn from(cause: DeviceCreationError) -> Self {
    NewInstanceError::DeviceCreation(cause)
  }
}

impl fmt::Display for NewInstanceError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      NewInstanceError::NoDevice => write!(f, "no suitable graphics device"),
      NewInstanceError::DeviceCreation(cause) => {
        write!(f, "failed to create graphics device: {}", cause)
      }
    }
  }
}
