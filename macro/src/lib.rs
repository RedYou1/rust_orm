extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput};

/// # Panics
/// Will panic if cant parse the input
#[proc_macro_derive(Table, attributes(PrimaryKey, ForeignKey, table_name))]
pub fn table_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let table_name = get_attribute(&ast.attrs, "table_name")
        .and_then(|attr| {
            attr.parse_args::<syn::LitStr>()
                .map(|lit_str| lit_str.value())
                .ok()
        })
        .unwrap_or(name.to_string());

    let (primary_keys, columns) = get_primary_keys_and_columns(&ast.data);

    let references = impl_references(&ast.data);

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
            fn references() -> Vec<(&'static str, &'static str, Vec<Column>, Vec<Column>)> {
                vec![#(#references),*]
            }
        }
    }
    .into()
}

fn get_primary_keys_and_columns(data: &syn::Data) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut primary_keys = Vec::new();
    let mut columns = Vec::new();

    if let syn::Data::Struct(data_struct) = data {
        for field in &data_struct.fields {
            let field_name = field.ident.as_ref().expect("");
            let foreign = get_attribute(&field.attrs, "ForeignKey").is_some();

            if get_attribute(&field.attrs, "PrimaryKey").is_some() {
                primary_keys.push(quote! { Column::new(stringify!(#field_name), #foreign) });
            }

            columns.push(quote! { Column::new(stringify!(#field_name), #foreign) });
        }
    }

    (primary_keys, columns)
}

fn get_attribute<'a>(
    attrs: &'a [syn::Attribute],
    attribute: &'static str,
) -> Option<&'a Attribute> {
    attrs.iter().find(|&attr| attr.path().is_ident(attribute))
}

fn impl_references(data: &syn::Data) -> Vec<TokenStream> {
    let mut references = Vec::new();

    if let syn::Data::Struct(data_struct) = data {
        for field in &data_struct.fields {
            if get_attribute(&field.attrs, "ForeignKey").is_some() {
                let field_type = &field.ty;
                let field_name = field.ident.as_ref().expect("");

                references.push(quote! {
                    (stringify!(#field_name), #field_type::table_name(), #field_type::identifiers(), #field_type::columns())
                });
            }
        }
    }

    references
}
