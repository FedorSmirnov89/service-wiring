use events::{EventThree, EventTwo};

#[derive(Default)]
pub struct ServiceTwo {}

impl ServiceTwo {
    pub fn process(&self, event: EventTwo) -> EventThree {
        let EventTwo { count } = event;
        println!("service two doing stuff; Current count: {count}");
        EventThree {
            count: (count + 1) as f64,
        }
    }
}
