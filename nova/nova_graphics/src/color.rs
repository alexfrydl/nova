// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color {
  pub const TRANSPARENT: Self = Color::new(0.0, 0.0, 0.0, 0.0);
  pub const WHITE: Self = Color::new(1.0, 1.0, 1.0, 1.0);
  pub const BLACK: Self = Color::new(0.0, 0.0, 0.0, 1.0);

  /// Creates a new color with the given component values.
  pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Color { r, g, b, a }
  }
}

impl From<[f32; 4]> for Color {
  fn from(values: [f32; 4]) -> Self {
    Color::new(values[0], values[1], values[2], values[3])
  }
}

impl From<Color> for [f32; 4] {
  fn from(color: Color) -> Self {
    [color.r, color.g, color.b, color.a]
  }
}
