extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate case;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::MacroInput;
use syn::Body::{Struct, Enum};
use case::CaseExt;

#[proc_macro_derive(Error)]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let error = expand_error(&ast);
    error.parse().unwrap()
}

fn expand_error(ast: &MacroInput) -> Tokens {
    let name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let (display, description, cause, from_impls) = match ast.body {
        Struct(_) => {
            panic!("Using structs to define errors is not (yet?) supported. Use an enum instead.");
        }
        Enum(ref variants) => {
            let mut display = Tokens::new();
            let mut description = Tokens::new();
            let mut cause = Tokens::new();
            let mut from_impls = Tokens::new();

            if variants.is_empty() {
                let doc = title(&ast.attrs).unwrap_or_else(|| label_str(&name.to_string()));
                display.append_all(&[quote!{ write!(f, #doc) }]);
                description.append_all(&[quote!{ #doc }]);
                cause.append_all(&[quote!{ None }]);
            } else {
                for var in variants {
                    let var_name = &var.ident;
                    let doc = title(&var.attrs).unwrap_or_else(|| label_str(&var_name.to_string()));
                    match var.data {
                        syn::VariantData::Unit => {
                            display.append_all(&[quote!{ #name::#var_name => write!(f, #doc), }]);
                            description.append_all(&[quote!{ #name::#var_name => #doc, }]);
                            cause.append_all(&[quote!{ #name::#var_name => None, }]);
                        }
                        syn::VariantData::Tuple(ref fields) => {
                            display.append_all(&[quote!{ #name::#var_name(_) => write!(f, #doc), }]);
                            description.append_all(&[quote!{ #name::#var_name(ref err) => err.description(), }]);
                            cause.append_all(&[quote!{ #name::#var_name(ref err) => Some(err), }]);
                            let field = fields.clone().into_iter().next().expect("A tuple must have at least 1 field");
                            let typ = field.ty;
							from_impls.append_all(&[quote!{
								impl #impl_generics From<#typ> for #name #ty_generics #where_clause {
									fn from(err: #typ) -> #name #ty_generics {
										#name::#var_name(err)
									}
								}
							}]);
                        }
                        syn::VariantData::Struct(ref fields) => {
                            let field = fields.clone().into_iter().next().expect("An enum struct must have at least 1 field");
                            let field_name = field.ident.expect("A struct field must have an identifier");
                            let typ = field.ty;
                            display.append_all(&[quote!{ #name::#var_name{..} => write!(f, #doc), }]);
                            description.append_all(&[quote!{ #name::#var_name{ref #field_name} => #field_name.description(), }]);
                            cause.append_all(&[quote!{ #name::#var_name{ref #field_name} => Some(#field_name), }]);
							from_impls.append_all(&[quote!{
								impl #impl_generics From<#typ> for #name #ty_generics #where_clause {
									fn from(err: #typ) -> #name #ty_generics {
										#name::#var_name{
                                            #field_name: err,
                                        }
									}
								}
							}]);
                        }
                    }
                }
                display = quote!{ match *self { #display } };
                description = quote!{ match *self { #description } };
                cause = quote!{ match *self { #cause } };
            }

            (display, description, cause, from_impls)
        }
    };

    // https://doc.rust-lang.org/stable/book/error-handling.html#error-handling-with-a-custom-type
    quote! {
        impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                #display
            }
        }

        impl #impl_generics ::std::error::Error for #name #ty_generics #where_clause {
            fn description(&self) -> &str {
                #description
            }

            fn cause(&self) -> Option<&::std::error::Error> {
                #cause
            }
        }

        #from_impls
    }
}

fn title(attributes: &Vec<syn::Attribute>) -> Option<String> {
    for attr in attributes {
        if attr.is_sugared_doc {
            if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref doc, _)) = attr.value {
                for line in doc.lines() {
                    let doc = line.trim_left_matches("///").trim();
                    if !doc.is_empty() {
                        return Some(doc.to_lowercase());
                    }
                }
            }
        }
    }
    None
}

fn label_str(label: &str) -> String {
    label
        .to_snake()
        .replace("_", " ")
        .trim()
        .to_lowercase()
}
