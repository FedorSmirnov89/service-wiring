use crate::events::{EventOne, EventThree};

#[derive(Default)]
pub(crate) struct ServiceThree {}

impl ServiceThree {
    pub(crate) fn process(&self, event: EventThree) -> EventOne {
        let EventThree { count } = event;
        println!("service three doing stuff; Current count: {count}");
        EventOne {
            count: (count + 1.0).round() as i32,
        }
    }
}
