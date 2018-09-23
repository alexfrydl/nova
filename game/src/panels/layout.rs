// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use super::Hierarchy;

/// Component that stores layout state for a panel entity.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Layout {
  /// Dimension indicating distance between the top of the entity's parent and
  /// the top of the entity.
  pub top: Dimension,
  /// Dimension indicating distance between the left side of the entity's parent
  /// and the left side of the entity.
  pub left: Dimension,
  /// Dimension indicating distance between the bottom of the entity's parent
  /// and the bottom of the entity.
  pub bottom: Dimension,
  /// Dimension indicating distance between the right side of the entity's
  /// parent and the right side of the entity.
  pub right: Dimension,
  /// Dimension indicating the size of the entity in the x direction.
  pub width: Dimension,
  /// Dimension indicating the size of the entity in the y direction.
  pub height: Dimension,
  /// Rect describing the location of the entity relative to its parent's rect.
  rect: Rect,
  /// Rect describing the absolute location of the entity.
  absolute_rect: Rect,
}

impl Layout {
  /// Gets the rect describing the absolute location of the entity.
  pub fn absolute_rect(&self) -> &Rect {
    &self.absolute_rect
  }
}

impl Default for Layout {
  fn default() -> Self {
    Layout {
      top: Dimension::Auto,
      left: Dimension::Auto,
      bottom: Dimension::Auto,
      right: Dimension::Auto,
      width: Dimension::Auto,
      height: Dimension::Auto,
      rect: Rect::default(),
      absolute_rect: Rect::default(),
    }
  }
}

/// One of the possible dimension definitions for a panel measurement.
pub enum Dimension {
  /// Dimension is automatically calculated from available space.
  Auto,
  /// Dimension is fixed to a specific value.
  Fixed(f32),
}

/// Struct describing a rectangle in 2D space.
#[derive(Clone, Copy)]
pub struct Rect {
  /// Position of the rectangle's top left corner.
  pub position: Point2<f32>,
  /// Size of the rectangle.
  pub size: Vector2<f32>,
}

impl Default for Rect {
  fn default() -> Rect {
    Rect {
      position: Point2::origin(),
      size: Vector2::zeros(),
    }
  }
}

/// System that computes absolute location and size of every panel based on
/// layout dimensions.
pub struct LayoutSolver {
  /// Root entity to solve from.
  pub root: Entity,
  /// Stack used in traversing the hierarchy.
  stack: Vec<(Entity, Rect)>,
}

impl LayoutSolver {
  /// Creates a new solver for the given root entity.
  pub fn new(root: Entity) -> Self {
    LayoutSolver {
      root,
      stack: Vec::new(),
    }
  }
}

impl<'a> System<'a> for LayoutSolver {
  type SystemData = (
    ReadResource<'a, engine::Window>,
    ReadStorage<'a, Hierarchy>,
    WriteStorage<'a, Layout>,
  );

  fn run(&mut self, (window, hierarchy, mut layouts): Self::SystemData) {
    // Begin with a stack containing the root element and a rectangle the size
    // of the entire engine window.
    self.stack.clear();
    self.stack.push((
      self.root,
      Rect {
        position: Point2::origin(),
        size: window.size(),
      },
    ));

    // Pop an entity and parent rect off of the stack until it is empty.
    while let Some((entity, parent_rect)) = self.stack.pop() {
      // If the entity has a layout…
      if let Some(layout) = layouts.get_mut(entity) {
        // Compute its x and y dimensions relative to the parent rect.
        let x = solve_dimensions(
          parent_rect.size.x,
          &layout.left,
          &layout.width,
          &layout.right,
        );

        let y = solve_dimensions(
          parent_rect.size.y,
          &layout.top,
          &layout.height,
          &layout.bottom,
        );

        // Set its local rect with those dimensions.
        layout.rect = Rect {
          position: Point2::new(x.0, y.0),
          size: Vector2::new(x.1, y.1),
        };

        // Set the absolute rect to the parent rect + the local rect.
        layout.absolute_rect = Rect {
          position: parent_rect.position + layout.rect.position.coords,
          size: layout.rect.size,
        };

        // If this element has children, push them each onto the stack with that
        // absolute rect.
        if let Some(node) = hierarchy.get(entity) {
          for child in node.children() {
            self.stack.push((*child, layout.absolute_rect));
          }
        }
      }
    }
  }
}

/// Computes the position and size given a start, size, and end dimension.
///
/// For example, passing in the left, width, and right dimensions would return
/// the appopriate x-coordinate and width for those dimensions.
fn solve_dimensions(
  full: f32,
  start_dim: &Dimension,
  size_dim: &Dimension,
  end_dim: &Dimension,
) -> (f32, f32) {
  // Amount of remaining space.
  let mut remaining = full;

  // Values for start and size if they have been found.
  let mut start = None;
  let mut size = None;

  // Number of unknown values (start, size, and end).
  let mut unknowns = 3;

  // 1. First subtract all fixed dimensions from remaining space.
  if let Dimension::Fixed(value) = start_dim {
    remaining -= value;
    start = Some(*value);
    unknowns -= 1;
  }

  if let Dimension::Fixed(value) = size_dim {
    remaining -= value;
    size = Some(*value);
    unknowns -= 1;
  }

  if let Dimension::Fixed(value) = end_dim {
    remaining -= value;
    unknowns -= 1;
  }

  // 2. Then, if the size is `Auto`, it consumes all remaining space.
  if let Dimension::Auto = size_dim {
    size = Some(remaining);
    remaining = 0.0;
    unknowns -= 1;
  }

  // 3. Lastly, if the start dimension is `Auto`, it consumes an equal share of
  //    the remaining space (half if the end dimension is `Auto` or all of it
  //    otherwise).
  if let Dimension::Auto = start_dim {
    start = Some(remaining / unknowns as f32);
  }

  // Return start and size values which have been calculated by now.
  (start.unwrap(), size.unwrap())
}
