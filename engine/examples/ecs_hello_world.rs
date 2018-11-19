// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova::ecs;

fn main() {
  // Create a new engine instance. This contains a base set of ECS resources.
  let mut engine = nova::Engine::new();

  // Create a new dispatcher. A dispatcher creates an execution plan so that
  // systems that do not mutate the same resources can run simultaneously on
  // separate threads.
  let mut dispatcher = ecs::DispatcherBuilder::new()
    // Add a `Greeter` as a new system named `"Greeter"` with no dependencies.
    .system("Greeter", &[], Greeter)
    // Set up the dispatcher and all of its systems. Calls `Greeter::setup()`.
    .build(&mut engine);

  // Dispatches all systems once. In this case, it calls `Greeter::run` which
  // prints `"Hello world."`, the default message.
  dispatcher.dispatch(&mut engine);

  // Gets a mutable reference to the `Greeting` resource to change its message.
  let greeting: &mut Greeting = ecs::get_resource_mut(&mut engine);

  greeting.message = "Hallo Welt!".into();

  // This time, `Greeter::run` will print `"Hallo Welt!"`.
  dispatcher.dispatch(&mut engine);
}

// A resource read by `Greeter` to determine what to print on the screen.
struct Greeting {
  message: String,
}

// A system that prints a message on the screen.
struct Greeter;

impl<'a> ecs::System<'a> for Greeter {
  type Data = ecs::ReadResource<'a, Greeting>;

  fn setup(&mut self, engine: &mut nova::Engine) {
    // Add a `Greeting` resource with a default message.
    ecs::put_resource(
      engine,
      Greeting {
        message: "Hello world.".into(),
      },
    );
  }

  fn run(&mut self, greeting: Self::Data) {
    println!("{}", greeting.message);
  }
}
