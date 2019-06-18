// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::buffer::Access as BufferAccess;

use crate::{backend, Buffer};
use std::{ops, mem};

pub struct Barrier<'a>(backend::Barrier<'a>);

impl<'a> Barrier<'a> {
  pub(crate) fn as_backend(&self) -> &backend::Barrier<'a> {
    &self.0
  }
}

pub fn buffer_barrier<T>(
  buffer: &Buffer<T>,
  range: impl ops::RangeBounds<u64>,
  access: ops::Range<BufferAccess>,
) -> Barrier {
  let size_of = mem::size_of::<T>() as u64;

  let start = match range.start_bound() {
    ops::Bound::Unbounded => None,
    ops::Bound::Included(i) => Some(*i * size_of),
    ops::Bound::Excluded(i) => Some((*i + 1) * size_of),
  };

  let end = match range.start_bound() {
    ops::Bound::Unbounded => None,
    ops::Bound::Included(i) => Some(*i * size_of),
    ops::Bound::Excluded(i) => Some((*i - 1) * size_of),
  };

  Barrier(gfx_hal::memory::Barrier::Buffer {
    target: buffer.as_backend(),
    states: access,
    families: None,
    range: start..end,
  })
}