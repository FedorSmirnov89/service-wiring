//! Specifies the logic used for the parsing of the `event_bus` macro annotation

use syn::{parse::Parse, punctuated::Punctuated, Ident, Path, Token};

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
