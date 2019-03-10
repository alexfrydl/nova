// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::ImageId;
use nova_core::math::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageSlice {
  pub image_id: ImageId,
  pub rect: Rect<f32>,
}

impl From<ImageId> for ImageSlice {
  fn from(id: ImageId) -> Self {
    Self {
      image_id: id,
      rect: Rect::unit(),
    }
  }
}
