use proc_macro2::TokenStream;
use quote::quote;

fn dotenv_inner(item: TokenStream) -> TokenStream {
    let item_str = {
        let string = item.to_string().replace('\"', "");

        if string.is_empty() {
            ".env".to_string()
        } else {
            string
        }
    };

    let path = format!("\"../{}\"", item_str);

    let path_lit = litrs::StringLit::parse(path).unwrap();
    let path_lit_val = path_lit.value();

    quote! {
        // {
            const ENV_FILE: &str = include_str!(#path_lit_val);

            for line in ENV_FILE.lines() {
                let mut var = line.split('=');

                let decl_opt = var.next();
                let value_opt = var.next();

                if let Some(decl) = decl_opt {
                    if let Some(value) = value_opt {
                        if decl.contains(" ") || value.contains(" ") {
                            panic!("Invalid .env file")
                        }

                        std::env::set_var(decl, value).unwrap();
                    }
                }
            }
        // }
    }
}

#[proc_macro]
pub fn dotenv(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv_inner(item.into()).into()
}
