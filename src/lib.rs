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
        vec![
            generate_printers_by_field(&input),
            generate_printers(&input),
            generate_printers_critical_by_field(&input),
            generate_printers_panic_by_field(&input),
            generate_printers_err_by_field(&input),
            generate_printers_info_by_field(&input),
            generate_printers_rust_by_field(&input),
            generate_printers_warning_by_field(&input),
            generate_printers_success_by_field(&input),
        ],
    );
    expanded.into()
}
