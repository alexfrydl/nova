// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::processes;
use super::Context;

/// One of the phases of the engine loop.
#[derive(Eq, PartialEq)]
pub enum LoopPhase {
  Early,
  Normal,
  Late,
}

/// Runs the engine loop until `engine::exit` is called.
///
/// Each iteration of the main engine loop is split into three sequential
/// phases: early, regular, and late. In each phase, first the systems for that
/// phase are dispatched, then processes are updated in the order they were
/// added.
///
/// “Early” systems and processes implement “inputs” to the engine. For example:
/// time, player input, or updates from a multiplayer server.
///
/// “Late” systems and processes implement “outputs” from the engine. For
/// example: sending updates _to_ a multiplayer server, calculating UI layout,
/// or drawing graphics on the screen.
///
/// The majority of systems and processes are neither early nor late, and they
/// implement most game logic.
///
/// Systems are run in parallel in each of the three phases. After the systems
/// run in each phase, all processes are updated in sequence.
pub fn run(ctx: &mut Context) {
  let init_state = ctx
    .init_state
    .take()
    .expect("engine context is already running");

  let mut early_systems = init_state.early_systems.build();
  let mut systems = init_state.systems.build();
  let mut late_systems = init_state.late_systems.build();

  // Run the engine loop.
  while !ctx.exiting {
    // Update the window each loop if there is one.
    if ctx.window_handle.borrow().is_some() {
      super::update_window(ctx);
    }

    // Run all systems and processes.

    early_systems.dispatch(&mut ctx.world.res);
    processes::run_processes(ctx, LoopPhase::Early);

    systems.dispatch(&mut ctx.world.res);
    processes::run_processes(ctx, LoopPhase::Normal);

    late_systems.dispatch(&mut ctx.world.res);
    processes::run_processes(ctx, LoopPhase::Late);
  }
}

/// Exits the engine loop started with `engine::run`.
pub fn exit(ctx: &mut Context) {
  ctx.exiting = true;
}