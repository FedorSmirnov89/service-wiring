use std::time::Duration;

use events::{EventOne, EventThree, EventTwo};

#[derive(Default)]
pub struct ServiceTwo {}

#[ebus_macro::service_input]
pub enum Input {
    EventTwo(EventTwo),
    EventThree(EventThree),
}

pub enum Output {
    EventOne(EventOne),
}

impl ServiceTwo {
    pub fn process(&self, input: Input) -> Output {
        match input {
            Input::EventTwo(event) => {
                let EventTwo { count } = event;
                println!("service two doing stuff; Current count: {count}");
                std::thread::sleep(Duration::from_millis(150));
                let e = EventOne {
                    count: (count + 1) as i32,
                };
                Output::EventOne(e)
            }
            Input::EventThree(event) => {
                let EventThree { count } = event;
                println!("service two doing stuff; Current count: {count}");
                std::thread::sleep(Duration::from_millis(150));
                let e = EventOne {
                    count: (count + 1.0).round() as i32,
                };
                Output::EventOne(e)
            }
        }
    }
}
