// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Context;

pub use specs::shred::Fetch as FetchResource;
pub use specs::shred::FetchMut as FetchResourceMut;
pub use specs::shred::Read as ReadResource;
pub use specs::shred::SetupHandler as ResourceSetup;
pub use specs::shred::Write as WriteResource;
pub use specs::shred::{Resource, Resources};

/// Adds a resource to the ECS context.
pub fn add_resource(ctx: &mut Context, resource: impl Resource) {
  ctx.world.add_resource(resource)
}

/// Fetches a resource from the ECS context.
pub fn fetch_resource<'a, T: Resource + Send + 'a>(ctx: &'a Context) -> FetchResource<'a, T> {
  ctx.world.read_resource()
}

/// Mutably fetches a resource from the ECS context.
pub fn fetch_resource_mut<'a, T: Resource + Send + 'a>(
  ctx: &'a Context,
) -> FetchResourceMut<'a, T> {
  ctx.world.write_resource()
}

/// Checks whether the ECS context has a resource of type `T`.
pub fn has_resource<T: Resource + Send>(ctx: &Context) -> bool {
  ctx.world.res.has_value::<T>()
}
