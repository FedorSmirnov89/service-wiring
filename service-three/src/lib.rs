use std::time::Duration;

use events::{EventOne, EventThree};

#[derive(Default)]
pub struct ServiceThree {}

impl ServiceThree {
    pub fn process(&self, event: EventThree) -> EventOne {
        let EventThree { count } = event;
        println!("service three doing stuff; Current count: {count}");
        std::thread::sleep(Duration::from_millis(350));
        EventOne {
            count: (count + 1.0).round() as i32,
        }
    }
}
