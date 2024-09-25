use target_structure::{EventBus, EventOne};

fn main() {
    let event_bus = EventBus::default();
    event_bus.run(EventOne {});
}
