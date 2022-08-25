use proc_macro2::TokenStream;
use quote::quote;

fn dotenv_inner(item: TokenStream) -> TokenStream {
    let item_str = item.to_string();

    let path = format!("../{}", item_str);

    let path_lit = litrs::StringLit::parse(path).unwrap();

    quote! {
        fn your_mother() -> &'static str { #path_lit }
    }
}

#[proc_macro]
pub fn dotenv(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv_inner(item.into()).into()
}
