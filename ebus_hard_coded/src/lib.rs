use std::time::Duration;

use events::{EventOne, EventThree, EventTwo};
use service_one::ServiceOne;
use service_three::ServiceThree;
use service_two::ServiceTwo;

///
/// So, in general we would want to be able to express that we consume/subscribe to
/// multiple different services
///
/// We also would want to say that we produce multiple different services
///
// trait Service{
//     type
// } //  let's define that one a bit later

// we probably will want to autogenerate this whole struct
#[derive(Default)]
pub struct EventBus {
    service_one: ServiceOne,
    service_two: ServiceTwo,
    service_three: ServiceThree,
}

impl EventBus {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(self, start_event: AnyEvent) {
        // need some logic to start the thing up and introduce external events,
        // but let's ignore that right now
        let mut current = start_event;

        loop {
            current = self.process_event(current);
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    fn process_event(&self, event: AnyEvent) -> AnyEvent {
        match event {
            AnyEvent::EventOne(event_one) => {
                let event = self.service_one.process(event_one);
                AnyEvent::EventTwo(event)
            }
            AnyEvent::EventTwo(event_two) => {
                let event = self.service_two.process(event_two);
                AnyEvent::EventThree(event)
            }
            AnyEvent::EventThree(event_three) => {
                let event = self.service_three.process(event_three);
                AnyEvent::EventOne(event)
            }
        }
    }
}

pub enum AnyEvent {
    EventOne(EventOne),
    EventTwo(EventTwo),
    EventThree(EventThree),
}
