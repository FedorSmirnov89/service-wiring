use crate::events::{EventOne, EventThree, EventTwo};

#[derive(Default)]
pub(crate) struct ServiceThree {}

impl ServiceThree {
    pub(crate) fn process(&self, event: EventThree) -> EventOne {
        println!("service three doing stuff");
        EventOne {}
    }
}
