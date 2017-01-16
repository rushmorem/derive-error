//! Derive custom errors using macros 1.1

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate case;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{MacroInput, Ident};
use syn::Body::{Struct, Enum};
use case::CaseExt;

#[derive(Debug)]
struct Error {
    ast: MacroInput,
    display: Tokens,
    description: Tokens,
    cause: Tokens,
    from_impls: Tokens,
}

#[proc_macro_derive(Error)]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    Error::new(ast).derive().parse().unwrap()
}

impl Error {
    fn new(ast: MacroInput) -> Error {
        Error {
            ast: ast,
            display: Tokens::new(),
            description: Tokens::new(),
            cause: Tokens::new(),
            from_impls: Tokens::new(),
        }
    }

    // Derives a new error
    fn derive(mut self) -> Tokens {
        let name = self.ast.ident.clone();
        match self.ast.body.clone() {
            Struct(ref variants) => {
                match *variants {
                    syn::VariantData::Unit => {
                        self.struct_unit();
                    }
                    _ => {
                        panic!("Only deriving from unit structs is supported. Use an enum instead.");
                    }
                }
            }

            Enum(ref variants) => {
                if variants.is_empty() {
                    let msg = format!("{0} looks awkward with no variants. Please use a struct unit instead. Example: `struct {0};`", name);
                    panic!(msg);
                } else {
                    for var in variants {
                        let var_name = &var.ident;
                        let msg = self.title(&var.attrs).unwrap_or_else(|| self.label_str(&var_name.to_string()));
                        match var.data {
                            syn::VariantData::Unit => {
                                self.unit_variant(var_name, &msg);
                            }
                            syn::VariantData::Tuple(ref fields) => {
                                self.tuple_variant(var_name, &msg, fields);
                            }
                            syn::VariantData::Struct(ref fields) => {
                                self.struct_field(var_name, &msg, fields);
                            }
                        }
                    }
                    let display = self.display;
                    self.display = quote!{ match *self { #display } };
                    let description = self.description;
                    self.description = quote!{ match *self { #description } };
                    let cause = self.cause;
                    self.cause = quote!{ match *self { #cause } };
                }
            }
        };

        let (impl_generics, ty_generics, where_clause) = self.ast.generics.split_for_impl();
        let display = &self.display;
        let description = &self.description;
        let cause = &self.cause;
        let from_impls = &self.from_impls;

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

    // Configures a unit variant of an enum
    fn unit_variant(&mut self, var_name: &Ident, msg: &str) {
        let name = &self.ast.ident;
        self.display.append_all(&[quote!{ #name::#var_name => write!(f, #msg), }]);
        self.description.append_all(&[quote!{ #name::#var_name => #msg, }]);
        self.cause.append_all(&[quote!{ #name::#var_name => None, }]);
    }

    // Configures a tuple variant of an enum
    fn tuple_variant(&mut self, var_name: &Ident, msg: &str, fields: &Vec<syn::Field>) {
        let (impl_generics, ty_generics, where_clause) = self.ast.generics.split_for_impl();
        let name = &self.ast.ident;
        self.display.append_all(&[quote!{ #name::#var_name(_) => write!(f, #msg), }]);
        self.description.append_all(&[quote!{ #name::#var_name(ref err) => err.description(), }]);
        self.cause.append_all(&[quote!{ #name::#var_name(ref err) => Some(err), }]);
        let field = fields.clone().into_iter().next().unwrap_or_else(|| {
            let msg = format!("{0} looks awkward with no fields. Did you mean to add a type, eg. `{}(::std::io::Error)` but forgot?", var_name);
            panic!(msg);
        });
        let typ = field.ty;
        self.from_impls.append_all(&[quote!{
            impl #impl_generics From<#typ> for #name #ty_generics #where_clause {
                fn from(err: #typ) -> #name #ty_generics {
                    #name::#var_name(err)
                }
            }
        }]);
    }

    // Configures a struct field
    fn struct_field(&mut self, var_name: &Ident, msg: &str, fields: &Vec<syn::Field>) {
        let (impl_generics, ty_generics, where_clause) = self.ast.generics.split_for_impl();
        let field = fields.clone().into_iter().next().unwrap_or_else(|| {
            let msg = format!("{0} looks awkward with not fields in it. Please use a struct unit instead. Example: `struct {0};`", var_name);
            panic!(msg);
        });
        let field_name = field.ident.unwrap();
        let typ = field.ty;
        let name = &self.ast.ident;
        self.display.append_all(&[quote!{ #name::#var_name{..} => write!(f, #msg), }]);
        self.description.append_all(&[quote!{ #name::#var_name{ref #field_name} => #field_name.description(), }]);
        self.cause.append_all(&[quote!{ #name::#var_name{ref #field_name} => Some(#field_name), }]);
        self.from_impls.append_all(&[quote!{
            impl #impl_generics From<#typ> for #name #ty_generics #where_clause {
                fn from(err: #typ) -> #name #ty_generics {
                    #name::#var_name{
                        #field_name: err,
                    }
                }
            }
        }]);
    }

    // Creates an error from a unit struct or an enum without any variants
    fn struct_unit(&mut self) {
        let doc = self.title(&self.ast.attrs).unwrap_or_else(|| self.label_str(&self.ast.ident.to_string()));
        self.display.append_all(&[quote!{ write!(f, #doc) }]);
        self.description.append_all(&[quote!{ #doc }]);
        self.cause.append_all(&[quote!{ None }]);
    }

    // Extracts the title of an error from a doc comment
    fn title(&self, attributes: &Vec<syn::Attribute>) -> Option<String> {
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

    // Creates a human friendly string from the fieldname of an enum variant
    // or struct field
    fn label_str(&self, label: &str) -> String {
        label
            .to_snake()
            .replace("_", " ")
            .trim()
            .to_lowercase()
    }
}
