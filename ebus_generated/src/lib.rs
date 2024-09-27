use events::{EventOne, EventThree, EventTwo};

#[ebus_macro::event_bus(
    service_one::ServiceOne(cons = [EventOne], prod =[EventTwo]),
    service_two::ServiceTwo(cons = [EventTwo, EventThree], prod = [EventOne]),
    service_three::ServiceThree(cons = [EventTwo], prod = [EventThree])
)]
pub struct EventBus {}
