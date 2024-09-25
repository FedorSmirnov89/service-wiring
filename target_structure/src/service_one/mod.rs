use crate::events::{EventOne, EventTwo};

#[derive(Default)]
pub(crate) struct ServiceOne {}

impl ServiceOne {
    pub(crate) fn process(&self, event: EventOne) -> EventTwo {
        let EventOne { count } = event;
        println!("service one doing stuff; current count: {count}");
        EventTwo {
            count: (count + 1) as u32,
        }
    }
}
