#[cfg(any(feature = "toml", feature = "json", feature = "yaml"))]
use litrs::Literal;
#[cfg(any(feature = "toml", feature = "json", feature = "yaml"))]
use proc_macro::TokenStream;
#[cfg(any(feature = "toml", feature = "json", feature = "yaml"))]
use quote::quote;

#[cfg(any(feature = "toml", feature = "json", feature = "yaml"))]
macro_rules! parse_args {
    ($args:expr) => {{
        let mut toks = $args.into_iter();
        let Some(tok) = toks.next() else {
            return quote! { ::core::compile_error!("exepcted one string literal argument") }
                .into();
        };
        if toks.next().is_some() {
            return quote! { ::core::compile_error!("exepcted one string literal argument") }
                .into();
        }

        match Literal::try_from(tok) {
            Ok(Literal::String(lit)) => String::from(lit.value()),
            _ => {
                return quote! { ::core::compile_error!("exepcted one string literal argument") }
                    .into();
            }
        }
    }};
}

#[cfg(any(feature = "toml", feature = "json", feature = "yaml"))]
macro_rules! parse_path {
    ($args:expr) => {{
        let path = parse_args!($args);
        let path = std::path::Path::new(&path);
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            let base_dir = match std::env::var("CARGO_MANIFEST_DIR") {
                Ok(base_dir) => base_dir,
                Err(err) => {
                    let err = err.to_string();
                    return quote! { ::core::compile_error!(#err) }.into();
                }
            };
            std::path::Path::new(&base_dir).join(path)
        };

        use std::io::Read;

        let mut s = String::new();
        let mut f = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(err) => {
                let err = err.to_string();
                return quote! { ::core::compile_error!(#err); }.into();
            }
        };

        if let Err(err) = f.read_to_string(&mut s) {
            let err = err.to_string();
            return quote! { ::core::compile_error!(#err); }.into();
        }

        s
    }};
}

#[cfg(feature = "toml")]
fn toml_impl(s: &str) -> TokenStream {
    let mut code: Vec<u8> = Vec::new();
    if let Err(err) = const_config_gen::toml::generate(s.as_bytes(), &mut code) {
        let err = err.to_string();
        return quote! { ::core::compile_error!(#err) }.into();
    }

    match String::from_utf8(code) {
        Ok(code) => match code.parse::<TokenStream>() {
            Ok(code) => code,
            Err(err) => {
                let err = err.to_string();
                quote! { ::core::compile_error!(#err) }.into()
            }
        },
        Err(err) => {
            let err = err.to_string();
            quote! { ::core::compile_error!(#err) }.into()
        }
    }
}

#[cfg(feature = "toml")]
#[proc_macro]
pub fn from_toml(args: TokenStream) -> TokenStream {
    toml_impl(&parse_args!(args))
}

#[cfg(feature = "toml")]
#[proc_macro]
pub fn include_toml(args: TokenStream) -> TokenStream {
    toml_impl(&parse_path!(args))
}

#[cfg(feature = "json")]
fn json_impl(s: &str) -> TokenStream {
    let mut code: Vec<u8> = Vec::new();
    if let Err(err) = const_config_gen::json::generate(s.as_bytes(), &mut code) {
        let err = err.to_string();
        return quote! { ::core::compile_error!(#err) }.into();
    }

    match String::from_utf8(code) {
        Ok(code) => match code.parse::<TokenStream>() {
            Ok(code) => code,
            Err(err) => {
                let err = err.to_string();
                quote! { ::core::compile_error!(#err) }.into()
            }
        },
        Err(err) => {
            let err = err.to_string();
            quote! { ::core::compile_error!(#err) }.into()
        }
    }
}

#[cfg(feature = "json")]
#[proc_macro]
pub fn from_json(args: TokenStream) -> TokenStream {
    json_impl(&parse_args!(args))
}

#[cfg(feature = "json")]
#[proc_macro]
pub fn include_json(args: TokenStream) -> TokenStream {
    json_impl(&parse_path!(args))
}

#[cfg(feature = "yaml")]
fn yaml_impl(s: &str) -> TokenStream {
    let mut code: Vec<u8> = Vec::new();
    if let Err(err) = const_config_gen::yaml::generate(s.as_bytes(), &mut code) {
        let err = err.to_string();
        return quote! { ::core::compile_error!(#err) }.into();
    }

    match String::from_utf8(code) {
        Ok(code) => match code.parse::<TokenStream>() {
            Ok(code) => code,
            Err(err) => {
                let err = err.to_string();
                quote! { ::core::compile_error!(#err) }.into()
            }
        },
        Err(err) => {
            let err = err.to_string();
            quote! { ::core::compile_error!(#err) }.into()
        }
    }
}

#[cfg(feature = "yaml")]
#[proc_macro]
pub fn from_yaml(args: TokenStream) -> TokenStream {
    yaml_impl(&parse_args!(args))
}

#[cfg(feature = "yaml")]
#[proc_macro]
pub fn include_yaml(args: TokenStream) -> TokenStream {
    yaml_impl(&parse_path!(args))
}
