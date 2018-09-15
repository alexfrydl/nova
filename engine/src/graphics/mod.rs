// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub mod atlas;
pub mod sprite;

pub use self::atlas::Atlas;
pub use self::sprite::Sprite;

pub fn setup<'a, 'b>(core: &mut Core, _dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<Sprite>();
}
