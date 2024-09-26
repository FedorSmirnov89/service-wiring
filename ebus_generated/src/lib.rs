use service_one::ServiceOne;
use service_three::ServiceThree;
use service_two::ServiceTwo;

use events::{EventOne, EventThree, EventTwo};

#[ebus_macro::event_bus(
    ServiceOne(EventOne, EventTwo),
    ServiceTwo(EventTwo, EventThree),
    ServiceThree(EventThree, EventOne)
)]
pub struct EventBus {}
