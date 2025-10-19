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
            let prefix = format!("NekoPrint{struct_name_string}{field_name_capitalized}");
            let printer_struct_name = Ident::new(&prefix, Span::call_site());

            quote! {
                #[derive(Debug, Clone)]
                pub struct #printer_struct_name {
                    pub target: Option<#field_type>,
                    pub message: Option<String>,
                }

                impl #printer_struct_name {
                    pub fn new() -> Self {
                        Self { target: None, message: None }
                    }

                    pub fn target<T: Into<#field_type>>(mut self, t: T) -> Self {
                        self.target = Some(t.into());
                        self
                    }

                    pub fn message<S: Into<String>>(mut self, m: S) -> Self {
                        self.message = Some(m.into());
                        self
                    }

                    pub async fn rust(self) {
                        let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                        let message_text = self.message.unwrap_or_default();
                        use nekoprint_imports::colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @RUST => {}.{}:{} = {:#?} {}",
                            nekoprint_imports::chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            std::any::type_name::<#field_type>(),
                            &target,
                            &message_text
                        ).rgb(255,165,0);
                        #(#transporter_stmts)*
                    }

                    pub async fn info(self) {
                        let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                        let message_text = self.message.unwrap_or_default();
                        use nekoprint_imports::colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @INFO => {}.{}:{} = {:#?} {}",
                            nekoprint_imports::chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            std::any::type_name::<#field_type>(),
                            &target,
                            &message_text
                        ).rgb(0,191,255);
                        #(#transporter_stmts)*
                    }

                    // outros níveis seguem o mesmo padrão...
                    pub async fn success(self) {
                        let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                        let message_text = self.message.unwrap_or_default();
                        use nekoprint_imports::colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @SUCCESS => {}.{}:{} = {:#?} {}",
                             nekoprint_imports::chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            std::any::type_name::<#field_type>(),
                            &target,
                            &message_text
                        ).green();
                        #(#transporter_stmts)*
                    }

                    pub async fn warning(self) {
                        let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                        let message_text = self.message.unwrap_or_default();
                        use nekoprint_imports::colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @WARNING => {}.{}:{} = {:#?} {}",
                             nekoprint_imports::chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            std::any::type_name::<#field_type>(),
                            &target,
                            &message_text
                        ).yellow();
                        #(#transporter_stmts)*
                    }

                    pub async fn err(self) {
                        let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                        let message_text = self.message.unwrap_or_default();
                        use nekoprint_imports::colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @ERROR => {}.{}:{} = {:#?} {}",
                             nekoprint_imports::chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            std::any::type_name::<#field_type>(),
                            &target,
                            &message_text
                        ).rgb(255, 49, 49);
                        #(#transporter_stmts)*
                    }

                    pub async fn critical(self) {
                        let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                        let message_text = self.message.unwrap_or_default();
                        use nekoprint_imports::colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @CRITICAL => {}.{}:{} = {:#?} {}",
                             nekoprint_imports::chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            std::any::type_name::<#field_type>(),
                            &target,
                            &message_text
                        ).red();
                        #(#transporter_stmts)*
                    }

                    pub async fn panic(self) {
                        let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                        let message_text = self.message.unwrap_or_default();
                        use nekoprint_imports::colorful::Colorful;
                        let message = format!(
                            "({} {} {}:{}) @PANIC => {}.{}:{} = {:#?} {}",
                             nekoprint_imports::chrono::Local::now(),
                            file!(),
                            line!(),
                            column!(),
                            #struct_name_string,
                            #field_name_string,
                            std::any::type_name::<#field_type>(),
                            &target,
                            &message_text
                        ).rgb(225,32,254);
                        #(#transporter_stmts)*
                    }
                }

                impl #impl_block {
                    pub fn #method_name(&self) -> #printer_struct_name {
                        #printer_struct_name::new().target(self.#field_name.clone())
                    }
                }
            }
        })
        .collect();

    quote! { #(#tokens)* }
}
