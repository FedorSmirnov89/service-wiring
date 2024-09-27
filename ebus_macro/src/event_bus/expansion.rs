use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use super::data::WiringData;

type QuoteStream = proc_macro2::TokenStream;

pub(super) fn expand_wiring_logic(data: WiringData) -> TokenStream {
    let struct_def = struct_definition(&data);
    let enum_def = enum_definition(&data);

    let full_expand = quote! {
        #enum_def
        #struct_def
    };

    TokenStream::from(full_expand)
}

fn enum_definition(data: &WiringData) -> QuoteStream {
    let mut variants = vec![];
    for event in &data.events {
        variants.push(quote! {
            #event(#event)
        });
    }
    quote! {
        enum AnyEvent{
            #(#variants),*
        }
    }
}

fn struct_definition(data: &WiringData) -> QuoteStream {
    let mut service_fields = vec![];
    for service in &data.services {
        let f_type = &service.type_name;
        let f_path = &service.path;
        let f_name = &service.field_name;
        let f_name_ident = Ident::new(&f_name, Span::call_site());
        let field = quote! {
            #f_name_ident:  #f_path #f_type
        };
        eprintln!("{field:?}");
        service_fields.push(field);
    }

    quote! {
        #[derive(Default)]
        pub struct EventBus{
            event_queue: std::collections::VecDeque<AnyEvent>,
            #(#service_fields),*
        }
    }
}
