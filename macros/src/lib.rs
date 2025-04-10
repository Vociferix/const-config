#[allow(unused)]
use litrs::Literal;
#[allow(unused)]
use proc_macro::TokenStream;
#[allow(unused)]
use quote::quote;

#[allow(unused)]
macro_rules! parse_str {
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

#[allow(unused)]
macro_rules! parse_bytes {
    ($args:expr) => {{
        let mut toks = $args.into_iter();
        let Some(tok) = toks.next() else {
            return quote! { ::core::compile_error!("exepcted one bytes literal argument") }.into();
        };
        if toks.next().is_some() {
            return quote! { ::core::compile_error!("exepcted one bytes literal argument") }.into();
        }

        match Literal::try_from(tok) {
            Ok(Literal::ByteString(lit)) => Vec::<u8>::from(lit.value()),
            _ => {
                return quote! { ::core::compile_error!("exepcted one bytes literal argument") }
                    .into();
            }
        }
    }};
}

#[allow(unused)]
macro_rules! parse_path_str {
    ($args:expr) => {{
        let path = parse_str!($args);
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

#[allow(unused)]
macro_rules! parse_path_bytes {
    ($args:expr) => {{
        let path = parse_str!($args);
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

        let mut b = Vec::<u8>::new();
        let mut f = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(err) => {
                let err = err.to_string();
                return quote! { ::core::compile_error!(#err); }.into();
            }
        };

        if let Err(err) = f.read_to_end(&mut b) {
            let err = err.to_string();
            return quote! { ::core::compile_error!(#err); }.into();
        }

        b
    }};
}

#[allow(unused)]
macro_rules! gen_impl {
    ($value:expr, $gen:expr) => {{
        let mut code: Vec<u8> = Vec::new();
        if let Err(err) = ($gen)(as_bytes($value), &mut code) {
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
    }};
    ($value:expr, $gen:expr,) => {
        impl_str!($value, $gen)
    };
}

fn as_bytes<T: AsRef<[u8]>>(data: &T) -> &[u8] {
    data.as_ref()
}

#[cfg(feature = "toml")]
#[proc_macro]
pub fn from_toml(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_str!(args), const_config_gen::toml::generate)
}

#[cfg(feature = "toml")]
#[proc_macro]
pub fn include_toml(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_str!(args), const_config_gen::toml::generate)
}

#[cfg(feature = "json")]
#[proc_macro]
pub fn from_json(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_str!(args), const_config_gen::json::generate)
}

#[cfg(feature = "json")]
#[proc_macro]
pub fn include_json(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_str!(args), const_config_gen::json::generate)
}

#[cfg(feature = "yaml")]
#[proc_macro]
pub fn from_yaml(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_str!(args), const_config_gen::yaml::generate)
}

#[cfg(feature = "yaml")]
#[proc_macro]
pub fn include_yaml(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_str!(args), const_config_gen::yaml::generate)
}

#[cfg(feature = "ron")]
#[proc_macro]
pub fn from_ron(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_str!(args), const_config_gen::ron::generate)
}

#[cfg(feature = "ron")]
#[proc_macro]
pub fn include_ron(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_str!(args), const_config_gen::ron::generate)
}

#[cfg(feature = "json5")]
#[proc_macro]
pub fn from_json5(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_str!(args), const_config_gen::json5::generate)
}

#[cfg(feature = "json5")]
#[proc_macro]
pub fn include_json5(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_str!(args), const_config_gen::json5::generate)
}

#[cfg(feature = "s-expr")]
#[proc_macro]
pub fn from_s_expr(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_str!(args), const_config_gen::s_expr::generate)
}

#[cfg(feature = "s-expr")]
#[proc_macro]
pub fn include_s_expr(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_str!(args), const_config_gen::s_expr::generate)
}

#[cfg(feature = "hjson")]
#[proc_macro]
pub fn from_hjson(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_str!(args), const_config_gen::hjson::generate)
}

#[cfg(feature = "hjson")]
#[proc_macro]
pub fn include_hjson(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_str!(args), const_config_gen::hjson::generate)
}

#[cfg(feature = "csv")]
#[proc_macro]
pub fn from_csv(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_str!(args), const_config_gen::csv::generate)
}

#[cfg(feature = "csv")]
#[proc_macro]
pub fn include_csv(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_str!(args), const_config_gen::csv::generate)
}

#[cfg(feature = "cbor")]
#[proc_macro]
pub fn from_cbor(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_bytes!(args), const_config_gen::cbor::generate)
}

#[cfg(feature = "cbor")]
#[proc_macro]
pub fn include_cbor(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_bytes!(args), const_config_gen::cbor::generate)
}

#[cfg(feature = "msgpack")]
#[proc_macro]
pub fn from_msgpack(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_bytes!(args), const_config_gen::msgpack::generate)
}

#[cfg(feature = "msgpack")]
#[proc_macro]
pub fn include_msgpack(args: TokenStream) -> TokenStream {
    gen_impl!(
        &parse_path_bytes!(args),
        const_config_gen::msgpack::generate
    )
}

#[cfg(feature = "pickle")]
#[proc_macro]
pub fn from_pickle(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_bytes!(args), const_config_gen::pickle::generate)
}

#[cfg(feature = "pickle")]
#[proc_macro]
pub fn include_pickle(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_bytes!(args), const_config_gen::pickle::generate)
}

#[cfg(feature = "bson")]
#[proc_macro]
pub fn from_bson(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_bytes!(args), const_config_gen::bson::generate)
}

#[cfg(feature = "bson")]
#[proc_macro]
pub fn include_bson(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_path_bytes!(args), const_config_gen::bson::generate)
}

#[cfg(feature = "flexbuffers")]
#[proc_macro]
pub fn from_flexbuffers(args: TokenStream) -> TokenStream {
    gen_impl!(&parse_bytes!(args), const_config_gen::flexbuffers::generate)
}

#[cfg(feature = "flexbuffers")]
#[proc_macro]
pub fn include_flexbuffers(args: TokenStream) -> TokenStream {
    gen_impl!(
        &parse_path_bytes!(args),
        const_config_gen::flexbuffers::generate
    )
}
