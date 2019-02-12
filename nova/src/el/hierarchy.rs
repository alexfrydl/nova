// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Mount, Node, RebuildRequired, ShouldRebuild};
use crate::ecs;

#[derive(Debug)]
pub struct Hierarchy {
  pub roots: Vec<ecs::Entity>,
  pub sorted: Vec<ecs::Entity>,
}

#[derive(Debug, Default)]
pub struct BuildHierarchy {
  build_stack: Vec<ecs::Entity>,
  apply_stack: Vec<(Option<ecs::Entity>, ecs::Entity, Node)>,
  delete_stack: Vec<ecs::Entity>,
}

impl<'a> ecs::System<'a> for BuildHierarchy {
  type SystemData = (
    ecs::ReadResource<'a, ecs::Entities>,
    ecs::WriteResource<'a, Hierarchy>,
    ecs::WriteComponents<'a, Mount>,
    ecs::WriteComponents<'a, RebuildRequired>,
  );

  fn run(&mut self, (entities, mut hierarchy, mut mounts, mut rebuild_required): Self::SystemData) {
    // Clear the sorted hierarchy which is about to change.
    hierarchy.sorted.clear();

    // Push each hierarchy root entity onto the stack in reverse order.
    for root in hierarchy.roots.iter().rev() {
      self.build_stack.push(*root);
    }

    // For each entity on the stack…
    while let Some(entity) = self.build_stack.pop() {
      // By now, all entities before this one in the hierarchy have been
      // visited, so add this entity to the sorted vec.
      hierarchy.sorted.push(entity);

      // If this entity is a mounted element…
      if let Some(element) = mounts.get_mut(entity) {
        // Rebuild the element if needed.
        if rebuild_required.contains(entity) {
          rebuild_required.remove(entity);

          // Build the element and get the resulting children.
          let children = element.instance.build();

          // Flag any extra children for deletion.
          let current_len = element.real_children.len();
          let new_len = children.len();

          if new_len > current_len {
            let extras = element.real_children.drain(current_len - 1..new_len);

            self.delete_stack.extend(extras);
          }

          // Apply nodes to child elements and then build them.
          for (i, child) in children.into_iter().enumerate() {
            if i < element.real_children.len() {
              element.real_children.push(entities.create());
            }

            let child_entity = element.real_children[i];

            self.apply_stack.push((None, child_entity, child));
            self.build_stack.push(child_entity);
          }
        } else {
          // Add all of this element's children to the stack.
          for child in element.real_children.iter().rev() {
            self.build_stack.push(*child);
          }
        }
      }

      // Apply any changes to descendents.
      while let Some((parent, entity, node)) = self.apply_stack.pop() {
        let prototype = node.into_element_prototype();

        // Apply the prototype to the entity and get its element mount.
        let mut should_rebuild = ShouldRebuild::Yes;

        let mount = match mounts.get_mut(entity) {
          Some(mount) => {
            // If the mount already exists, update its props.
            match mount.instance.set_props(prototype.props) {
              Ok(rebuild) => {
                should_rebuild = rebuild;
              }

              Err(props) => {
                // The mounted element is a different type of element, so
                // replace it with a new instance based on the prototype.
                mount.instance = (prototype.new)(props);
              }
            };

            mount
          }

          None => {
            // If the mount doesn't exist, mount a new element instance based
            // on the prototype.
            mounts
              .insert(entity, Mount::new((prototype.new)(prototype.props)))
              .expect("Could not create element mount");

            mounts
              .get_mut(entity)
              .expect("Could not get newly created element mount")
          }
        };

        // Rebuild the element if needed.
        if let ShouldRebuild::Yes = should_rebuild {
          let children = mount.instance.build();

          // Flag any extra children for deletion.
          let current_len = mount.node_children.len();
          let new_len = children.len();

          if new_len > current_len {
            let extras = mount.node_children.drain(current_len - 1..new_len);

            self.delete_stack.extend(extras);
          }

          // Ensure enough child entities exist and flag each one for
          // application of the new node content.
          for (i, child) in children.into_iter().enumerate() {
            if i < mount.node_children.len() {
              mount.node_children.push(entities.create());
            }

            let child_entity = mount.node_children[i];

            self.apply_stack.push((Some(entity), child_entity, child));
          }

          if let Some(parent) = parent {
            let _ = rebuild_required.insert(parent, RebuildRequired);
          }
        } else {

        }
      }
    }

    // Delete all orphaned entities.
    while let Some(entity) = self.delete_stack.pop() {
      if let Some(mount) = mounts.get_mut(entity) {
        self.delete_stack.extend(mount.node_children.drain(..));
        self.delete_stack.extend(mount.real_children.drain(..));
      }

      mounts.remove(entity);

      let _ = entities.delete(entity); // Err is if the entity is already deleted. ¯\_(ツ)_/¯
    }
  }
}
