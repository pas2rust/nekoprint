use mokuya::components::prelude::*;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, ItemFn};

pub fn generate_print_by_field(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let struct_name_string = struct_name.to_string();
    let attributes = get_attributes(input);
    let impl_block = get_impl(input);
    let transporter_stmts = match get_attr::<ItemFn>(&attributes, "transporter") {
        Ok(transporter) => transporter.block.stmts.clone(),
        Err(_) => Vec::new(),
    };

    let tokens: Vec<proc_macro2::TokenStream> = get_named_fields(input)
        .unwrap()
        .named
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_name_string = field_name.to_string();
            let field_type = field.ty.clone();
            let field_name_capitalized = format!(
                "{}{}",
                field_name_string.chars().next().unwrap().to_uppercase(),
                &field_name_string[1..]
            );
            let method_name = new_ident("print", field_name);
            let prefix =
                format!("Neko_Print_Printer_{struct_name_string}_{field_name_capitalized}");
            let field_struct_name = Ident::new(&prefix, Span::call_site());

            quote! {
                #[derive(Debug, Default, kenzu::Builder)]
                pub struct #field_struct_name {
                    pub target: #field_type,
                    pub message: String,
                }

                impl #field_struct_name {
                    pub async fn rust(&self) {
                        use colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @RUST => {}.{} = {:#?} {}",
                            chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            &self.target,
                            &self.message
                        ).rgb(255,165,0);
                        #(#transporter_stmts)*
                    }

                    pub async fn info(&self) {
                        use colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @INFO => {}.{} = {:#?} {}",
                            chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            &self.target,
                            &self.message
                        ).rgb(0,191,255);
                        #(#transporter_stmts)*
                    }

                    pub async fn success(&self) {
                        use colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @SUCCESS => {}.{} = {:#?} {}",
                            chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            &self.target,
                            &self.message
                        ).green();
                        #(#transporter_stmts)*
                    }

                    pub async fn warning(&self) {
                        use colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @WARNING => {}.{} = {:#?} {}",
                            chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            &self.target,
                            &self.message
                        ).yellow();
                        #(#transporter_stmts)*
                    }

                    pub async fn err(&self) {
                        use colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @ERROR => {}.{} = {:#?} {}",
                            chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            &self.target,
                            &self.message
                        ).rgb(255, 49, 49);
                        #(#transporter_stmts)*
                    }

                    pub async fn critical(&self) {
                        use colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @CRITICAL => {}.{} = {:#?} {}",
                            chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            &self.target,
                            &self.message
                        ).red();
                        #(#transporter_stmts)*
                    }

                    pub async fn panic(&self) {
                        use colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @PANIC => {}.{} = {:#?} {}",
                            chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            &self.target,
                            &self.message
                        ).rgb(225,32,254);
                        #(#transporter_stmts)*
                    }
                }

                impl #impl_block {
                    pub fn #method_name(&self) -> #field_struct_name {
                        #field_struct_name::new()
                            .target(self.#field_name.clone())
                    }
                }
            }
        })
        .collect();

    quote! { #(#tokens)* }
}
