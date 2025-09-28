use mokuya::components::prelude::*;
use quote::quote;
use syn::{DeriveInput, ItemFn};

pub fn generate_printers(input: &DeriveInput) -> proc_macro2::TokenStream {
    let attributes = get_attributes(input);
    let impl_block = get_impl(input);
    let struct_name = get_struct_name(input);
    let transporter = match get_attr::<ItemFn>(&attributes, "transporter") {
        Ok(transporter) => transporter.block.stmts,
        Err(_) => Vec::new(),
    };
    let printer_struct_name = new_ident("Neko_Print_Printer", &struct_name);

    quote! {
        #[derive(Debug, Default, kenzu::Builder)]
        pub struct #printer_struct_name {
            pub target: #struct_name,
            pub message: String,
        }

        impl #printer_struct_name {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = format!(
                  "({} {} {}:{}) @RUST => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self.target,
                    &self.message,
                ).rgb(255,165,0);
                #(#transporter)*(message);
            }

            pub async fn info(&self) {
                use colorful::Colorful;
                let message = format!(
                  "({} {} {}:{}) @INFO => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self.target,
                    &self.message,
                ).rgb(0,191,255);
                #(#transporter)*(message);
            }

            pub async fn success(&self) {
                use colorful::Colorful;
                let message = format!(
                    "({} {} {}:{}) @SUCCESS => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self.target,
                    &self.message,
                ).green();
                #(#transporter)*(message);
            }

            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = format!(
                  "({} {} {}:{}) @WARNING => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self.target,
                    &self.message,
                ).yellow();
                #(#transporter)*(message);
            }

            pub async fn err(&self) {
                use colorful::Colorful;
                let message = format!(
                    "({} {} {}:{}) @ERROR => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self.target,
                    &self.message,
                ).rgb(255, 49, 49);
                #(#transporter)*(message);
            }

            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = format!(
                   "({} {} {}:{}) @CRITICAL => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self.target,
                    &self.message,
                ).red();
                #(#transporter)*(message);
            }

            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = format!(
                   "({} {} {}:{}) @PANIC => {:#?} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    &self.target,
                    &self.message,
                ).rgb(225,32,254);
                #(#transporter)*(message);
            }
        }

        impl #impl_block {
            pub fn print(&self) -> #printer_struct_name {
                #printer_struct_name::new().target(self.clone())
            }
        }
    }
}
