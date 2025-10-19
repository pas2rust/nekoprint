use mokuya::components::prelude::*;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident, ItemFn};

pub fn generate_printers(input: &DeriveInput) -> proc_macro2::TokenStream {
    let attributes = get_attributes(input);
    let impl_block = get_impl(input);
    let struct_name = get_struct_name(input);
    let transporter_stmts = match get_attr::<ItemFn>(&attributes, "transporter") {
        Ok(transporter) => transporter.block.stmts.clone(),
        Err(_) => Vec::new(),
    };
    let printer_struct_name = Ident::new(&format!("NekoPrint{struct_name}"), Span::call_site());

    quote! {
        #[derive(Debug, Clone)]
        pub struct #printer_struct_name {
            pub target: Option<#struct_name>,
            pub message: Option<String>,
        }

        impl #printer_struct_name {
            pub fn new() -> Self {
                Self { target: None, message: None }
            }

            pub fn target<T: Into<#struct_name>>(mut self, t: T) -> Self {
                self.target = Some(t.into());
                self
            }

            pub fn message<S: Into<String>>(mut self, m: S) -> Self {
                self.message = Some(m.into());
                self
            }

            pub async fn rust(self) {
                use nekoprint_imports::colorful::Colorful;
                let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                let message_text = self.message.unwrap_or_default();
                let message = format!(
                  "({} {} {}:{}) @RUST => {:#?} {}",
                    nekoprint_imports::chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &target,
                    &message_text,
                ).rgb(255,165,0);
                #(#transporter_stmts)*
            }

            pub async fn info(self) {
                use nekoprint_imports::colorful::Colorful;
                let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                let message_text = self.message.unwrap_or_default();
                let message = format!(
                  "({} {} {}:{}) @INFO => {:#?} {}",
                    nekoprint_imports::chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &target,
                    &message_text,
                ).rgb(0,191,255);
                #(#transporter_stmts)*
            }

            pub async fn success(self) {
                use nekoprint_imports::colorful::Colorful;
                let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                let message_text = self.message.unwrap_or_default();
                let message = format!(
                    "({} {} {}:{}) @SUCCESS => {:#?} {}",
                    nekoprint_imports::chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &target,
                    &message_text,
                ).green();
                #(#transporter_stmts)*
            }

            pub async fn warning(self) {
                use nekoprint_imports::colorful::Colorful;
                let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                let message_text = self.message.unwrap_or_default();
                let message = format!(
                  "({} {} {}:{}) @WARNING => {:#?} {}",
                    nekoprint_imports::chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &target,
                    &message_text,
                ).yellow();
                #(#transporter_stmts)*
            }

            pub async fn err(self) {
                use nekoprint_imports::colorful::Colorful;
                let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                let message_text = self.message.unwrap_or_default();
                let message = format!(
                    "({} {} {}:{}) @ERROR => {:#?} {}",
                    nekoprint_imports::chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &target,
                    &message_text,
                ).rgb(255, 49, 49);
                #(#transporter_stmts)*
            }

            pub async fn critical(self) {
                use nekoprint_imports::colorful::Colorful;
                let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                let message_text = self.message.unwrap_or_default();
                let message = format!(
                   "({} {} {}:{}) @CRITICAL => {:#?} {}",
                    nekoprint_imports::chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &target,
                    &message_text,
                ).red();
                #(#transporter_stmts)*
            }

            pub async fn panic(self) {
                use nekoprint_imports::colorful::Colorful;
                let target = self.target.expect(concat!("NekoPrint: target for ", stringify!(#printer_struct_name), " is required"));
                let message_text = self.message.unwrap_or_default();
                let message = format!(
                   "({} {} {}:{}) @PANIC => {:#?} {}",
                    nekoprint_imports::chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &target,
                    &message_text,
                ).rgb(225,32,254);
                #(#transporter_stmts)*
            }
        }

        impl #impl_block {
            pub fn print(&self) -> #printer_struct_name {
               #printer_struct_name::new().target(self.clone())
            }
        }
    }
}
