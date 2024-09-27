use std::collections::{HashMap, HashSet};

use data::WiringData;
use expansion::expand_wiring_logic;
use parsing::EbusMacroInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Token, Ident, Path, Token,
};

mod data;
mod expansion;
mod parsing;

pub fn event_bus_function(attr: TokenStream, _item: TokenStream) -> TokenStream {
    // parse the attributes passed to the macro (the services to add into the bus)
    let input = parse_macro_input!(attr as EbusMacroInput);
    let data: WiringData = input.into();
    // eprintln!("DEBUGGING");
    // eprintln!("{data:?}");
    // eprintln!("DEBUGGING");
    expand_wiring_logic(data)

    // let mut service_entries = vec![];

    // for service_entry in input.services {
    //     let ServiceEntry {
    //         service_path,
    //         // service_type,
    //         // in_events,
    //         // out_events,
    //     } = service_entry;

    //     let s = quote! {
    //         path = #service_path
    //     };

    //     service_entries.push(s);
    // }

    // let expanded_input = quote! {
    //     #(#service_entries),*
    // };

    // let mut fields = vec![];
    // let mut defaults = vec![];
    // let mut event_types: HashSet<Ident> = HashSet::new();
    // let mut event_to_consumer: HashMap<Ident, Vec<String>> = HashMap::new(); // maps an (out) event to the services which consume it
    // let mut producer_to_event: HashMap<String, Vec<Ident>> = HashMap::new();

    // for service in input.services {
    //     let service_type = service.service_type;
    //     let field_name = to_snake_case(&service_type);
    //     let field_ident = format_ident!("{field_name}");

    //     event_types.insert(service.event_in.clone());
    //     event_types.insert(service.event_out.clone());

    //     fields.push(quote! {
    //         #field_ident: #service_type
    //     });
    //     defaults.push(quote! {
    //         #field_ident: Default::default()
    //     });

    //     event_to_consumer
    //         .entry(service.event_in)
    //         .or_insert_with(Vec::new)
    //         .push(field_name.clone());

    //     producer_to_event
    //         .entry(field_name.clone())
    //         .or_insert_with(Vec::new)
    //         .push(service.event_out);
    // }

    // let mut match_arms = vec![];
    // for (event_out, consumers) in event_to_consumer {
    //     let mut process_statements = vec![];
    //     for service in consumers {
    //         let service_ident = format_ident!("{service}");
    //         let event_out_service = producer_to_event
    //             .get(&service)
    //             .expect("service does not produce")
    //             .first()
    //             .expect("no entries for producer");
    //         process_statements.push(quote! {
    //             let out_event = self.#service_ident.process(event.clone());
    //             AnyEvent::#event_out_service(out_event)
    //         });
    //     }

    //     let match_arm = quote! {
    //         AnyEvent::#event_out(event) => {
    //             #(#process_statements)*
    //         }
    //     };
    //     match_arms.push(match_arm);
    // }

    // let expanded_struct = quote! {
    //     pub struct EventBus{
    //         event_queue: std::collections::VecDeque<AnyEvent>,
    //         #(#fields),*
    //     }

    //     impl EventBus{
    //         pub fn new() -> Self{
    //             EventBus{
    //                 event_queue: Default::default(),
    //                 #(#defaults),*
    //             }
    //         }

    //         pub fn run(mut self, start_events: Vec<AnyEvent>){
    //             self.event_queue.extend(start_events);

    //             loop{
    //                 let current_event = self.event_queue.pop_front().expect("queue empty");
    //                 self.process_event(current_event);
    //             }
    //         }

    //         fn process_event(&mut self, event: AnyEvent){
    //             let event = match event{
    //                 #(#match_arms),*
    //             };
    //             self.event_queue.push_back(event);
    //         }
    //     }

    // };

    // let event_variants: Vec<_> = event_types
    //     .iter()
    //     .map(|event_type| {
    //         quote! {
    //             #event_type(#event_type)
    //         }
    //     })
    //     .collect();

    // let expanded_enum = quote! {
    //     pub enum AnyEvent{
    //         #(#event_variants),*
    //     }
    // };

    // let expanded = quote! {
    //     #expanded_struct
    //     #expanded_enum
    // };

    // TokenStream::from(expanded)
}
