use std::time::Duration;

use events::{EventThree, EventTwo};

#[derive(Default)]
pub struct ServiceThree {}

pub enum Input {
    EventTwo(EventTwo),
}

pub enum Output {
    EventThree(EventThree),
}

impl ServiceThree {
    pub fn process(&self, input: Input) -> Output {
        match input {
            Input::EventTwo(event) => {
                let EventTwo { count } = event;
                println!("service three doing stuff; Current count: {count}");
                std::thread::sleep(Duration::from_millis(350));
                let e = EventThree {
                    count: (count + 1) as f64,
                };
                Output::EventThree(e)
            }
        }
    }
}
