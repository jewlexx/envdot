use proc_macro2::TokenStream;
use quote::quote;

fn dotenv_inner(item: TokenStream) -> TokenStream {
    let item_str = item.to_string();
    let current_dir = std::env::current_dir().unwrap();

    let item_path = quote! {};

    TokenStream::new()
}

#[proc_macro]
pub fn dotenv(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv_inner(item.into()).into()
}
