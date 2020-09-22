use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

#[proc_macro]
pub fn expose(input: TokenStream) -> TokenStream {
	let exports = parse_macro_input!(input as Exports);
	let prefix = exports.prefix;

	let type_t = format_ident!("C{}", prefix);

    let expanded = quote! {
		#[allow(non_camel_case_types)]
		type #type_t = std::ffi::c_void;
    };

	expanded.into()
}

struct Exports {
	prefix: syn::Ident,
	items: Vec<syn::TraitItemMethod>
}

impl Parse for Exports {
	fn parse(input: ParseStream) -> Result<Self> {
        let _ = input.call(syn::Attribute::parse_inner)?;
		let prefix : syn::Ident = input.parse()?;

        let content;
        let _ = braced!(content in input);
        let _ = content.call(syn::Attribute::parse_inner)?;

        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
        }

		Ok(Exports { items, prefix })
	}
}
