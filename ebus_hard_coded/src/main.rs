use ebus_hard_coded::EventBus;
use events::EventOne;

fn main() {
    let event_bus = EventBus::default();
    event_bus.run(EventOne { count: 1 });
}
