use mokuya::components::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, parse_macro_input};
mod components;

#[proc_macro_derive(NekoPrint, attributes(transporter))]
pub fn nekoprint(input: TokenStream) -> TokenStream {
    use components::prelude::*;

    let mut input = parse_macro_input!(input as DeriveInput);
    add_traits_to_generics(&mut input);

    let mut expanded = TokenStream2::new();
    for_extend_token_stream(
        &mut expanded,
        vec![generate_print_by_field(&input), generate_printers(&input)],
    );
    expanded.into()
}
