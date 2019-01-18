#![feature(async_await, futures_api, await_macro)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::ecs;
use nova::log;
use nova::thread;
use nova::time;
use nova::window::{self, Window};

pub fn main() {
  log::set_as_default();

  let mut res = ecs::Resources::new();

  ecs::setup(&mut res);

  let thread_pool = thread::create_pool();
  let mut updater = ecs::Dispatcher::new(update(), &thread_pool);

  updater.setup(&mut res);

  res.fetch_mut::<Window>().open();

  while !res.fetch::<Window>().is_closed() {
    updater.dispatch(&res);

    ecs::maintain(&mut res);

    thread::sleep(time::Duration::from_millis(1));
  }
}

fn update() -> impl for<'a> ecs::Dispatchable<'a> {
  ecs::seq![window::update(), time::elapse(),]
}
