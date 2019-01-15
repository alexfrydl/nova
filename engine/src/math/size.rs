use super::{Num, Scalar, Vector2};
use derive_more::*;
use std::fmt;

/// Two-dimensional size with width and height.
#[derive(Clone, Copy, PartialEq, Eq, From)]
pub struct Size<T: Scalar> {
  /// Vector whose components are the width and height of the size.
  pub vector: Vector2<T>,
}

impl<T: Scalar> Size<T> {
  /// Creates a new size with the given width and height.
  pub fn new(width: T, height: T) -> Self {
    Size {
      vector: Vector2::new(width, height),
    }
  }

  /// Gets the width component of the size.
  pub fn width(&self) -> T {
    self.vector.x
  }

  /// Gets the height component of the size.
  pub fn height(&self) -> T {
    self.vector.y
  }
}

// Implement `From` to convert to and from equivalent types.
impl From<(u32, u32)> for Size<u32> {
  fn from(size: (u32, u32)) -> Self {
    Size::new(size.0, size.1)
  }
}

// Implement `Default` to provide a zero size.
impl<T: Scalar + Num> Default for Size<T> {
  fn default() -> Self {
    Size::new(T::zero(), T::zero())
  }
}

impl<T: Scalar + Num> fmt::Debug for Size<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({:?}, {:?})", self.width(), self.height())
  }
}
