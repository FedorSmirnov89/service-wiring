use ebus_hard_coded::{AnyEvent, EventBus};
use events::EventOne;

fn main() {
    let event_bus = EventBus::default();
    let start_event = AnyEvent::EventOne(EventOne { count: 1 });
    event_bus.run(vec![start_event]);
}
