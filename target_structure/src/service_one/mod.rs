use crate::events::{EventOne, EventTwo};

#[derive(Default)]
pub(crate) struct ServiceOne {}

impl ServiceOne {
    pub(crate) fn process(&self, event: EventOne) -> EventTwo {
        println!("service one doing stuff");
        EventTwo {}
    }
}
