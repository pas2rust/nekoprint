use mokuya::components::prelude::*;
use quote::quote;
use syn::{DeriveInput, ItemFn};

pub fn generate_printers(input: &DeriveInput) -> proc_macro2::TokenStream {
    let attributes = get_attributes(input);
    let impl_block = get_impl(input);
    let transporter = match get_attr::<ItemFn>(&attributes, "transporter") {
        Ok(transporter) => transporter.block.stmts,
        Err(_) => Vec::new(),
    };

    quote! {
        impl #impl_block {
            pub async fn print(&self, custom: &str) {
                let message = format!(
                    "({} {} {}:{}) @PRINT => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self,
                    custom,
                );
                #(#transporter)*(message);
            }

            pub async fn print_rust(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!(
                  "({} {} {}:{}) @RUST => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self,
                    custom,
                ).rgb(255,165,0);
                #(#transporter)*(message);
            }

            pub async fn print_info(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!(
                  "({} {} {}:{}) @INFO => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self,
                    custom,
                ).rgb(0,191,255);
                #(#transporter)*(message);
            }

            pub async fn print_success(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!(
                    "({} {} {}:{}) @SUCCESS => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self,
                    custom,
                ).green();
                #(#transporter)*(message);
            }

            pub async fn print_warning(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!(
                  "({} {} {}:{}) @WARNING => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self,
                    custom,
                ).yellow();
                #(#transporter)*(message);
            }

            pub async fn print_err(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!(
                    "({} {} {}:{}) @ERROR => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self,
                    custom,
                ).rgb(255, 49, 49);
                #(#transporter)*(message);
            }

            pub async fn print_critical(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!(
                   "({} {} {}:{}) @CRITICAL => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self,
                    custom,
                ).red();
                #(#transporter)*(message);
            }

            pub async fn print_panic(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!(
                   "({} {} {}:{}) @PANIC => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self,
                    custom,
                ).rgb(225,32,254);
                #(#transporter)*(message);
            }
        }
    }
}
