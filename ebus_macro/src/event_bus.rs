use data::WiringData;
use expansion::expand_wiring_logic;
use parsing::EbusMacroInput;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod data;
mod expansion;
mod parsing;

pub fn event_bus_function(attr: TokenStream, _item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as EbusMacroInput);
    let data: WiringData = input.into();
    expand_wiring_logic(data)
}
