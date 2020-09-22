//! generates the boiler place rust code for 'FFI::Platypus::Lang::Rust':
//! see https://metacpan.org/pod/FFI::Platypus::Lang::Rust#structs
use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn::{braced, parse_macro_input, Result };
use syn::parse::{Parse, ParseStream};

/// example:
/// ```no_run
/// expose!{
///     Person {
///         fn new(name: &str, lucky_number: i32) -> Person;
///         fn name(&self) -> String;
///         fn lucky_number(&self) -> i32;
///     }
/// }
/// ```
#[proc_macro]
pub fn expose(input: TokenStream) -> TokenStream {
	let exports = parse_macro_input!(input as Exports);
	let prefix = exports.prefix;
	let type_t = format_ident!("C{}", prefix);

    let items : Vec<_> = exports.items.into_iter().map(|item| {
        let sig = item.sig;
        let fn_name = format_ident!("{}_{}", prefix, sig.ident);
        let mut inputs_in_param = vec![];
        let mut inputs_in_arg = vec![];
        let mut inputs_in_body = vec![];
        for input in sig.inputs.into_iter() {
            match input {
                syn::FnArg::Receiver(_) => {
                    inputs_in_arg.push(quote! { self_: *mut #type_t });
                    inputs_in_body.push(quote! { let self_ = unsafe {
                        &*(self_ as *mut #prefix)};
                    });
                },
                syn::FnArg::Typed(r) => {
                    let ty = *r.ty;
                    let mut ty = quote!{#ty};
                    let name = *r.pat;

                    let mut body = quote! {};

                    if format!("{}", &ty) == "& str" {
                        ty = quote!{ *const i8 };
                        body = quote! {
                            let #name = unsafe { std::ffi::CStr::from_ptr(#name) };
                            let #name = #name.to_string_lossy().into_owned();
                            let #name = &#name;
                        };
                    }
                    inputs_in_arg.push(quote! { #name: #ty });
                    inputs_in_body.push(body);
                    inputs_in_param.push(quote!(#name));
                }
            }
        };
        if sig.ident.to_string() == "new".to_string() {
            let destroy_fn_name = format_ident!("{}_DESTROY", prefix);
            quote! {
                #[no_mangle]
                pub extern "C" fn #fn_name(_class: *const i8, #(#inputs_in_arg),*)
                -> *mut #type_t {
                    #(#inputs_in_body)*

                    Box::into_raw(
                        Box::new(
                            #prefix::new(#(#inputs_in_param,)*)
                        ) 
                    ) as *mut #type_t
                }

                #[allow(non_snake_case)]
                #[no_mangle]
                pub extern "C" fn #destroy_fn_name(self_: *mut #type_t) {
                    unsafe { drop(Box::from_raw(self_ as *mut #prefix)) };
                }
            }
        } else {
            let mut output = match sig.output {
                syn::ReturnType::Default => quote! { () },
                syn::ReturnType::Type(_, ty) => quote! { #ty }
            };
            let mut output_body = quote! { };
            if format!("{}", &output) == "String" {
                output = quote! { *const i8 };
                output_body = quote! {
                    thread_local! (
                        static KEEP: std::cell::RefCell<Option<std::ffi::CString>> = std::cell::RefCell::new(None);
                    );

                    let result_c = std::ffi::CString::new(result_).unwrap();
                    let ptr = result_c.as_ptr();
                    KEEP.with(|k| {
                        *k.borrow_mut() = Some(result_c);
                    });
                    let result_ = ptr;
                }
            }
            let method_name = sig.ident;
            quote! {
                #[no_mangle]
                pub extern "C" fn #fn_name(#(#inputs_in_arg),*) -> #output {
                    #(#inputs_in_body)*
                    let result_ = self_.#method_name(#(#inputs_in_param,)*);
                    #output_body
                    result_
                }
            }
        }
    }).collect();


    let expanded = quote! {
		#[allow(non_camel_case_types)]
		type #type_t = std::ffi::c_void;

        #(#items)*
    };

    // let r = format!("{}", &expanded);
    // eprintln!("{}", r);

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
