use crate::events::{EventThree, EventTwo};

#[derive(Default)]
pub(crate) struct ServiceTwo {}

impl ServiceTwo {
    pub(crate) fn process(&self, event: EventTwo) -> EventThree {
        println!("service two doing stuff");
        EventThree {}
    }
}
