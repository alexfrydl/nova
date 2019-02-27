// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::context::{Context, NodeContext};
use super::hierarchy;
use super::spec::{self, Spec};
use super::{Element, ShouldRebuild};
use derive_more::*;
use std::any::Any;
use std::fmt;
use std::mem;

#[derive(Debug, Deref, DerefMut)]
pub(crate) struct Instance(Box<dyn InstanceLike>);

impl Instance {
  pub fn new<T: Element + 'static>(element: T, ctx: NodeContext) -> Self {
    Instance(Box::new(ElementInstance::new(element, ctx)))
  }
}

pub(crate) trait InstanceLike: Any + Send + Sync + fmt::Debug {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: NodeContext,
  ) -> Result<ShouldRebuild, Box<dyn Any>>;

  fn awake(&mut self, ctx: NodeContext);
  fn sleep(&mut self, ctx: NodeContext);

  fn on_message(
    &mut self,
    payload: Box<dyn Any>,
    ctx: NodeContext,
  ) -> Result<ShouldRebuild, Box<dyn Any>>;

  fn build(&mut self, children: &hierarchy::Children, ctx: NodeContext) -> Spec;
}

#[derive(Debug)]
struct ElementInstance<T: Element> {
  element: T,
  state: T::State,
  awake: bool,
}

impl<T: Element> ElementInstance<T> {
  fn new(element: T, ctx: NodeContext) -> Self {
    use super::ElementState;

    let state = T::State::new(ctx);

    ElementInstance {
      element,
      state,
      awake: false,
    }
  }
}

impl<T: Element + 'static> InstanceLike for ElementInstance<T> {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: NodeContext,
  ) -> Result<ShouldRebuild, Box<dyn Any>> {
    let mut element = (element as Box<dyn Any>).downcast::<T>()?;

    if *element == self.element {
      return Ok(ShouldRebuild(false));
    }

    mem::swap(&mut self.element, &mut *element);

    Ok(self
      .element
      .on_change(*element, Context::new(&mut self.state, ctx)))
  }

  fn awake(&mut self, ctx: NodeContext) {
    if self.awake {
      return;
    }

    self.awake = true;
    self.element.on_awake(Context::new(&mut self.state, ctx));
  }

  fn sleep(&mut self, ctx: NodeContext) {
    if !self.awake {
      return;
    }

    self.element.on_sleep(Context::new(&mut self.state, ctx));
    self.awake = false;
  }

  fn on_message(
    &mut self,
    msg: Box<dyn Any>,
    ctx: NodeContext,
  ) -> Result<ShouldRebuild, Box<dyn Any>> {
    let msg = msg.downcast::<T::Message>()?;

    Ok(self
      .element
      .on_message(*msg, Context::new(&mut self.state, ctx)))
  }

  fn build(&mut self, children: &hierarchy::Children, ctx: NodeContext) -> Spec {
    self.element.build(
      spec::Children {
        entities: children.entities.iter(),
      },
      Context::new(&mut self.state, ctx),
    )
  }
}