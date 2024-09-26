use ebus_generated::{AnyEvent, EventBus};
use events::EventOne;

pub fn main() {
    let ebus = EventBus::new();
    ebus.run(vec![AnyEvent::EventOne(EventOne { count: 0 })]);
}
