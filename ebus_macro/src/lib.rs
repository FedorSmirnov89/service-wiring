use proc_macro::TokenStream;

mod event_bus;
mod service_input;

use event_bus::event_bus_function;
use service_input::service_input_function;

#[proc_macro_attribute]
pub fn event_bus(attr: TokenStream, item: TokenStream) -> TokenStream {
    event_bus_function(attr, item)
}

#[proc_macro_attribute]
pub fn service_input(attr: TokenStream, item: TokenStream) -> TokenStream {
    service_input_function(attr, item)
}
