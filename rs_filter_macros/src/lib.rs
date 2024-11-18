extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct};

#[proc_macro_attribute]
pub fn filter_for(args: TokenStream, input: TokenStream) -> TokenStream {
    let type_arg = parse_macro_input!(args as Ident);

    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as ItemStruct);

    // Get the filter type.
    let filter_type = input.ident.clone();

    // Get all the field names.
    let fields: Vec<&Ident> = input
        .fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect();

    // Construct the filter method body.
    let q = quote! {
        #input
        impl Filterable<#filter_type> for #type_arg {
            fn is_match(&self, filter: &#filter_type) -> bool {
                #(
                    self.#fields.is_match(&filter.#fields)
                )&&*
            }
        }
    };

    // panic!("{}", q);
    q.into()
}
