//! Specifies the logic used for the parsing of the `event_bus` macro annotation
//! as well as how the info there is converted to the data used for the macro generation

use syn::{parse::Parse, punctuated::Punctuated, Ident, Path, Token};

use crate::event_bus::data::ServiceData;

use super::data::WiringData;

///
/// Info annotated per service
///
pub(super) struct ServiceEntry {
    service_path: Path,
    service_type: Ident,
    in_events: Punctuated<Ident, Token![,]>,
    out_events: Punctuated<Ident, Token![,]>,
}

impl Parse for ServiceEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut service_path: Path = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Failed to parse service. You have to provide it in the form [module_path::service_name]"))?;

        let service_type = service_path
            .segments
            .pop()
            .take()
            .expect("no service type")
            .value()
            .ident
            .clone();

        // parse the parens around the event spec
        let service_events;
        syn::parenthesized!(service_events in input);

        // Parse the in-events
        let cons_keyword: Ident = service_events.parse()?; // Parse the "in" keyword as Ident
        if cons_keyword != "cons" {
            return Err(syn::Error::new(cons_keyword.span(), "expected `cons`"));
        }
        service_events.parse::<Token![=]>()?;
        let in_events;
        syn::bracketed!(in_events in service_events);
        let in_events: Punctuated<Ident, Token![,]> =
            in_events.parse_terminated(Ident::parse, Token![,])?;

        // parse the out-events
        service_events.parse::<Token![,]>()?;
        let prod_keyword: Ident = service_events.parse()?; // Parse the "out" keyword as Ident
        if prod_keyword != "prod" {
            return Err(syn::Error::new(prod_keyword.span(), "expected `prod`"));
        }
        service_events.parse::<Token![=]>()?;
        let out_events;
        syn::bracketed!(out_events in service_events);
        let out_events: Punctuated<Ident, Token![,]> =
            out_events.parse_terminated(Ident::parse, Token![,])?;

        Ok(ServiceEntry {
            service_path,
            service_type,
            in_events,
            out_events,
        })
    }
}

///
/// The info from all the annotations
///
pub(super) struct EbusMacroInput {
    services: syn::punctuated::Punctuated<ServiceEntry, Token![,]>,
}

impl Parse for EbusMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(EbusMacroInput {
            services: input.parse_terminated(ServiceEntry::parse, Token![,])?,
        })
    }
}

impl From<EbusMacroInput> for WiringData {
    fn from(value: EbusMacroInput) -> Self {
        let mut services = vec![];

        for service in value.services {
            let type_name = service.service_type.clone();
            let field_name = to_snake_case(&type_name);
            let path = service.service_path;

            let service_data = ServiceData {
                type_name,
                field_name,
                path,
            };

            services.push(service_data);
        }

        Self { services }
    }
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
