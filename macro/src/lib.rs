extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput};

/// # Panics
/// Will panic if cant parse the input
#[proc_macro_derive(Table, attributes(PrimaryKey, ForeignKey, table_name))]
pub fn table_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(data_struct) = &ast.data {
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        let table_name = get_attribute(&ast.attrs, "table_name")
            .and_then(|attr| {
                attr.parse_args::<syn::LitStr>()
                    .map(|lit_str| lit_str.value())
                    .ok()
            })
            .unwrap_or(name.to_string());

        let (primary_keys, columns) = get_primary_keys_and_columns(data_struct);

        let references = impl_references(data_struct);

        quote! {
            impl #impl_generics Table for #name #ty_generics #where_clause {
                fn identifiers() -> Vec<Column> {
                    vec![#(#primary_keys),*]
                }
                fn table_name() -> &'static str {
                    #table_name
                }
                fn columns() -> Vec<Column> {
                    vec![#(#columns),*]
                }
                fn references() -> Vec<Reference> {
                    vec![#(#references),*]
                }
            }
        }
        .into()
    } else {
        panic!("Only on Struct");
    }
}

fn get_primary_keys_and_columns(
    data_struct: &syn::DataStruct,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut primary_keys = Vec::new();
    let mut columns = Vec::new();

    for field in &data_struct.fields {
        let field_name = field.ident.as_ref().expect("");
        let foreign = get_attribute(&field.attrs, "ForeignKey").is_some();

        if get_attribute(&field.attrs, "PrimaryKey").is_some() {
            primary_keys.push(quote! { Column::new(stringify!(#field_name), #foreign) });
        }

        columns.push(quote! { Column::new(stringify!(#field_name), #foreign) });
    }

    (primary_keys, columns)
}

fn get_attribute<'a>(
    attrs: &'a [syn::Attribute],
    attribute: &'static str,
) -> Option<&'a Attribute> {
    attrs.iter().find(|&attr| attr.path().is_ident(attribute))
}

enum ReferenceJoin {
    OneOrNone = 1,
    ExaclyOne = 2,
    Multiple = 4,
    MultipleOption = 8,
    MultipleEmpty = 16,
    MultipleOptionEmpty = 32,
}

fn impl_references(data_struct: &syn::DataStruct) -> Vec<TokenStream> {
    let mut references = Vec::new();

    for field in &data_struct.fields {
        if get_attribute(&field.attrs, "ForeignKey").is_some() {
            let field_type = &field.ty;
            let field_name = field.ident.as_ref().expect("");

            let join = match field_type {
                syn::Type::Slice(ty) => {
                    if get_inner_type("std::option::Option", ty.elem.as_ref()).is_some() {
                        ReferenceJoin::OneOrNone
                    } else {
                        ReferenceJoin::ExaclyOne
                    }
                }
                syn::Type::Path(_) => {
                    if let Some(ty) = get_inner_type("std::option::Option", field_type) {
                        if let Some(ty2) = get_inner_type("alloc::vec::Vec", ty) {
                            if get_inner_type("std::option::Option", ty2).is_some() {
                                ReferenceJoin::MultipleOptionEmpty
                            } else {
                                ReferenceJoin::MultipleEmpty
                            }
                        } else {
                            ReferenceJoin::OneOrNone
                        }
                    } else if let Some(ty) = get_inner_type("alloc::vec::Vec", field_type) {
                        if get_inner_type("std::option::Option", ty).is_some() {
                            ReferenceJoin::MultipleOption
                        } else {
                            ReferenceJoin::Multiple
                        }
                    } else {
                        ReferenceJoin::ExaclyOne
                    }
                }
                _ => ReferenceJoin::ExaclyOne,
            } as u8;

            references.push(quote! {
                Reference {
                    join: #join.into(),
                    column_prefix: stringify!(#field_name),
                    table_name: #field_type::table_name(),
                    identifiers: #field_type::identifiers(),
                    other_columns: #field_type::columns(),
                }
            });
        }
    }

    references
}

fn get_inner_type<'a>(full_name: &'static str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    let syn::Type::Path(ty) = ty else {
        return None;
    };
    if ty.qself.is_some() {
        return None;
    }

    let segments = &ty.path.segments;

    let full_name_found = segments
        .iter()
        .map(|a| a.ident.to_string())
        .collect::<Vec<String>>()
        .join("::");
    if full_name_found != full_name {
        return None;
    }

    let last_segment = segments.last()?;
    let syn::PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
        return None;
    };
    if generics.args.len() != 1 {
        return None;
    }
    let syn::GenericArgument::Type(inner_type) = &generics.args[0] else {
        return None;
    };

    Some(inner_type)
}
