use proc_macro2::TokenStream;
use quote::quote;

fn dotenv_inner(item: TokenStream) -> TokenStream {
    let item_str = item.to_string();

    let item_path = quote! {
        "../#item_str"
    };

    quote! {
        fn your_mother() -> &'_ str { #item_path }
    }
}

#[proc_macro]
pub fn dotenv(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv_inner(item.into()).into()
}
