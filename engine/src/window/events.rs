use derive_more::*;

/// One of the possible window events.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
  Resized,
  CloseRequested,
}

/// A resource containing the latest events received by an engine window.
#[derive(Default, From, Deref, DerefMut)]
pub struct Events {
  pub latest: Vec<Event>,
}

/// A structure that can be used to poll events for a window and store them in
/// a `Vec`.
#[derive(From)]
pub struct EventSource {
  events_loop: winit::EventsLoop,
}

impl EventSource {
  /// Polls events from the events loop and adds them to the given `Vec`.
  pub fn poll_into(&mut self, events: &mut Vec<Event>) {
    self.events_loop.poll_events(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        match event {
          winit::WindowEvent::CloseRequested => {
            events.push(Event::CloseRequested);
          }

          winit::WindowEvent::Resized(_) => {
            events.push(Event::Resized);
          }

          _ => {}
        }
      }
    });
  }
}
