use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn service_input_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;

    // Ensure the input is an enum
    let enum_data = match &input.data {
        Data::Enum(e) => e,
        _ => return TokenStream::from(quote! { compile_error!("Only enums are supported"); }),
    };

    // Collect From implementations for each variant
    let from_impls = enum_data.variants.iter().filter_map(|variant| {
        if let Fields::Unnamed(fields) = &variant.fields {
            let variant_ident = &variant.ident;
            let field_type = &fields.unnamed.first()?.ty;
            Some(quote! {
                impl From<#field_type> for #name {
                    fn from(value: #field_type) -> Self {
                        #name::#variant_ident(value)
                    }
                }
            })
        } else {
            None
        }
    });

    // Generate the final output code
    let expanded = quote! {
        #input

        #(#from_impls)*
    };

    TokenStream::from(expanded)
}
