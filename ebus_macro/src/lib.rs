use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, token::Paren, Ident, Path, Token};

///
/// Info provided per one service
///
struct ServiceEntry {
    service_type: Ident,
    _paren_token: Paren,
    event_in: Ident,
    _comma_token: Token![,],
    event_out: Ident,
}

impl Parse for ServiceEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ServiceEntry {
            service_type: input.parse()?,
            _paren_token: syn::parenthesized!(content in input),
            event_in: content.parse()?,
            _comma_token: content.parse()?,
            event_out: content.parse()?,
        })
    }
}

///
/// The info from all the annotations
///
struct EbusMacroInput {
    services: syn::punctuated::Punctuated<ServiceEntry, Token![,]>,
}

impl Parse for EbusMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(EbusMacroInput {
            services: input.parse_terminated(ServiceEntry::parse, Token![,])?,
        })
    }
}

#[proc_macro_attribute]
pub fn event_bus(attr: TokenStream, _item: TokenStream) -> TokenStream {
    // parse the attributes passed to the macro (the services to add into the bus)
    let input = parse_macro_input!(attr as EbusMacroInput);

    let mut fields = vec![];
    let mut defaults = vec![];
    let mut event_types: HashSet<Ident> = HashSet::new();

    for service in input.services {
        let service_type = service.service_type;
        let field_name = to_snake_case(&service_type);
        let field_ident = format_ident!("{field_name}");

        event_types.insert(service.event_in);
        event_types.insert(service.event_out);

        fields.push(quote! {
            #field_ident: #service_type
        });
        defaults.push(quote! {
            #field_ident: Default::default()
        })
    }

    let expanded_struct = quote! {
        pub struct EventBus{
            #(#fields),*
        }

        impl EventBus{
            pub fn new() -> Self{
                EventBus{
                    #(#defaults),*
                }
            }
        }
    };

    let event_variants: Vec<_> = event_types
        .iter()
        .map(|event_type| {
            quote! {
                #event_type(#event_type)
            }
        })
        .collect();

    let expanded_enum = quote! {
        pub enum AnyEvent{
            #(#event_variants),*
        }
    };

    let expanded = quote! {
        #expanded_struct
        #expanded_enum
    };

    TokenStream::from(expanded)
}

fn to_snake_case(ident: &Ident) -> String {
    let ident = ident.to_string();
    let mut snake_case = String::new();
    for (i, c) in ident.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                snake_case.push('_');
            }
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}
