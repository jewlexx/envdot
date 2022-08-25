use std::{fs::File, io::Read};

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

fn dotenv_inner(item: TokenStream) -> TokenStream {
    let item_str = {
        let string = item.to_string().replace('"', "");

        if string.is_empty() {
            ".env".to_string()
        } else {
            string
        }
    };

    let path = std::env::current_dir().unwrap().join(item_str);

    let item_span = item.span();

    if !path.exists() {
        return quote_spanned! {
            item_span => compile_error!("Env file does not exists")
        };
    }

    let mut file_bytes = vec![];

    File::open(path)
        .unwrap()
        .read_to_end(&mut file_bytes)
        .unwrap();

    let file_string = match String::from_utf8(file_bytes) {
        Ok(f) => f,
        Err(_) => {
            return quote! {
                compile_error!("Invalid env file. Is not valid utf8!");
            }
        }
    };

    let mut var_vec = vec![];

    for line in file_string.lines() {
        let mut var = line.split('=');

        let decl_opt = var.next();
        let value_opt = var.next();

        if let Some(decl) = decl_opt {
            if let Some(value) = value_opt {
                if decl.contains(' ') || value.contains(' ') {
                    panic!("Invalid .env file")
                }

                let decl_str = decl.replace('"', "").replace('\'', "");

                let value_str = value.replace('"', "").replace('\'', "");

                var_vec.push((decl_str, value_str));
            }
        }
    }

    let var_vec_len = var_vec.len();

    let var_vec_tokens = var_vec
        .iter()
        .map(|x| {
            let x0 = &x.0;
            let x1 = &x.1;

            quote! {(#x0, #x1)}
        })
        .collect::<Vec<_>>();

    quote! {
        {
            const ENV_FILE: [(&str, &str); #var_vec_len] = [#(#var_vec_tokens),*];

            for (decl, value) in ENV_FILE {
                std::env::set_var(decl, value);
            }
        }
    }
}

#[proc_macro]
pub fn dotenv(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv_inner(item.into()).into()
}
