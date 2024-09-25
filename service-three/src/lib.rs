use events::{EventOne, EventThree};

#[derive(Default)]
pub struct ServiceThree {}

impl ServiceThree {
    pub fn process(&self, event: EventThree) -> EventOne {
        let EventThree { count } = event;
        println!("service three doing stuff; Current count: {count}");
        EventOne {
            count: (count + 1.0).round() as i32,
        }
    }
}
