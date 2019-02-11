// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod assets;
pub mod clock;
pub mod ecs;
pub mod engine;
#[cfg(not(feature = "headless"))]
pub mod graphics;
pub mod log;
pub mod math;
#[cfg(not(feature = "headless"))]
pub mod ui;
pub mod utils;
#[cfg(not(feature = "headless"))]
pub mod window;

pub mod events {
  pub use shrev::{Event, EventChannel as Channel, EventIterator, ReaderId};
}

pub use self::engine::Engine;
pub use specs::{self, shred};

mod el;
