use std::time::Duration;

use events::{EventOne, EventTwo};

#[derive(Default)]
pub struct ServiceOne {}

#[ebus_macro::service_input]
pub enum Input {
    EventOne(EventOne),
}

pub enum Output {
    EventTwo(EventTwo),
}

impl ServiceOne {
    pub fn process(&self, input: Input) -> Output {
        match input {
            Input::EventOne(event) => {
                let EventOne { count } = event;
                println!("service one doing stuff; current count: {count}");
                std::thread::sleep(Duration::from_millis(250));
                let e = EventTwo {
                    count: (count + 1) as u32,
                };
                Output::EventTwo(e)
            }
        }
    }
}
