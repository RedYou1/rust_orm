extern crate proc_macro;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use rust_query_lib_macro::{
    get_attribute, get_inner_type, FOREIGN_KEY, OPTION, PRIMARY_KEY, TABLE_NAME, VEC,
};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Ident, Token};

/// # Panics
/// Will panic if cant parse the input
#[proc_macro_derive(Table, attributes(PrimaryKey, ForeignKey, table_name))]
pub fn table_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(data_struct) = &ast.data {
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        let table_name = get_attribute(&ast.attrs, TABLE_NAME)
            .and_then(|attr| {
                attr.parse_args::<syn::LitStr>()
                    .map(|lit_str| lit_str.value())
                    .ok()
            })
            .unwrap_or(name.to_string());

        let (primary_keys, eqs, columns) = get_primary_keys_and_columns(data_struct);

        let references = impl_references(name, data_struct);

        quote! {
            impl #impl_generics Table for #name #ty_generics #where_clause {
                fn identifiers() -> Vec<Column> {
                    vec![#(#primary_keys),*]
                }
                fn id_eq(&self, b: &Self) -> bool {
                    let mut eq = true;
                    #(#eqs)*
                    eq
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
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let mut primary_keys = Vec::new();
    let mut eqs = Vec::new();
    let mut columns = Vec::new();

    for field in &data_struct.fields {
        let field_name = field.ident.as_ref().expect("Not in a Tuple");
        let foreign = get_attribute(&field.attrs, FOREIGN_KEY).is_some();

        if get_attribute(&field.attrs, PRIMARY_KEY).is_some() {
            primary_keys.push(quote! { Column::new(stringify!(#field_name), #foreign) });
            eqs.push(quote! { eq = eq && self.#field_name == b.#field_name; });
        }

        columns.push(quote! { Column::new(stringify!(#field_name), #foreign) });
    }

    (primary_keys, eqs, columns)
}

enum ReferenceJoin {
    OneOrNone = 1,
    ExaclyOne = 2,
    Multiple = 4,
    MultipleOption = 8,
    MultipleEmpty = 16,
    MultipleOptionEmpty = 32,
}

fn impl_references(name: &Ident, data_struct: &syn::DataStruct) -> Vec<TokenStream> {
    let mut references = Vec::new();

    for field in &data_struct.fields {
        if let Some(att) = get_attribute(&field.attrs, FOREIGN_KEY).map(|attr| {
            attr.parse_args_with(Punctuated::parse_terminated)
                .map(|arr: Punctuated<Literal, Token![,]>| {
                    let a = arr
                        .into_iter()
                        .map(|a: Literal| {
                            let a = a.to_string();
                            a.chars().skip(1).take(a.len() - 2).collect()
                        })
                        .collect::<Vec<String>>();
                    assert!(
                        !(a.is_empty() || a.len() % 2 == 1),
                        "ForeignKey attribute has the alternate table_id and foreign_id"
                    );
                    a.chunks(2)
                        .map(|chunk| {
                            let a = format!("{};{}", chunk[0], chunk[1]);
                            quote! {(#a).to_owned()}
                        })
                        .collect::<Vec<TokenStream>>()
                })
                .expect("Need the list of the table column")
        }) {
            let mut field_type = &field.ty;

            let join = match field_type {
                syn::Type::Slice(ty) => {
                    if let Some(ty) = get_inner_type(OPTION, ty.elem.as_ref()) {
                        field_type = ty;
                        ReferenceJoin::OneOrNone
                    } else {
                        ReferenceJoin::ExaclyOne
                    }
                }
                syn::Type::Path(_) => {
                    if let Some(ty) = get_inner_type(OPTION, field_type) {
                        field_type = ty;
                        if let Some(ty) = get_inner_type(VEC, field_type) {
                            field_type = ty;
                            if let Some(ty) = get_inner_type(OPTION, field_type) {
                                field_type = ty;
                                ReferenceJoin::MultipleOptionEmpty
                            } else {
                                ReferenceJoin::MultipleEmpty
                            }
                        } else {
                            ReferenceJoin::OneOrNone
                        }
                    } else if let Some(ty) = get_inner_type(VEC, field_type) {
                        field_type = ty;
                        if let Some(ty) = get_inner_type(OPTION, field_type) {
                            field_type = ty;
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
                    to_table_name: #field_type::table_name(),
                    from_table_name: #name::table_name(),
                    identifiers: vec![#(#att),*],
                    all_columns: #field_type::all_columns(),
                    references: #field_type::references(),
                }
            });
        }
    }

    references
}
