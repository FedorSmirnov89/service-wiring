//! this is a "mock" for the crate which would define all of our events

#[derive(Clone)]
pub struct EventOne {
    pub count: i32,
}
#[derive(Clone)]
pub struct EventTwo {
    pub count: u32,
}
#[derive(Clone)]
pub struct EventThree {
    pub count: f64,
}
