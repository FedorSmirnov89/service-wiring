use std::{collections::VecDeque, time::Duration};

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
    event_queue: VecDeque<AnyEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(mut self, start_events: Vec<AnyEvent>) {
        // need some logic to start the thing up and introduce external events,
        // but let's ignore that right now
        self.event_queue.extend(start_events);

        loop {
            // we would rather async wait when queue empty (also need like a way of adding events during
            // runtime - ignoring all of that)
            let current_event = self.event_queue.pop_front().expect("queue empty");
            self.process_event(current_event);
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    fn process_event(&mut self, event: AnyEvent) {
        match event {
            AnyEvent::EventOne(event_one) => {
                let event = self.service_one.process(event_one.into()).into();
                self.event_queue.push_back(event);
            }
            AnyEvent::EventTwo(event_two) => {
                let event = self.service_two.process(event_two.clone().into()).into();
                self.event_queue.push_back(event);
                let event = self.service_three.process(event_two.clone().into()).into();
                self.event_queue.push_back(event);
            }
            AnyEvent::EventThree(event_three) => {
                let event = self.service_two.process(event_three.into()).into();
                self.event_queue.push_back(event);
            }
        };
    }
}

pub enum AnyEvent {
    EventOne(EventOne),
    EventTwo(EventTwo),
    EventThree(EventThree),
}

impl From<service_one::Output> for AnyEvent {
    fn from(value: service_one::Output) -> Self {
        match value {
            service_one::Output::EventTwo(event) => AnyEvent::EventTwo(event),
        }
    }
}

impl From<service_two::Output> for AnyEvent {
    fn from(value: service_two::Output) -> Self {
        match value {
            service_two::Output::EventOne(event) => AnyEvent::EventOne(event),
        }
    }
}
impl From<service_three::Output> for AnyEvent {
    fn from(value: service_three::Output) -> Self {
        match value {
            service_three::Output::EventThree(event) => AnyEvent::EventThree(event),
        }
    }
}
