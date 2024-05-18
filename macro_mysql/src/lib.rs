extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput};

/// # Panics
/// Will panic if cant parse the input
#[proc_macro_derive(MySQLRow, attributes(ForeignKey, table_name))]
pub fn mysqlrow_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let from_row = from_row(table_name.as_str(), &ast.data);

    quote! {
        impl #impl_generics MySQLRow for #name #ty_generics #where_clause {
            fn from_row_ref(row: &Row) -> Result<Self, FromRowError> {
                Ok(Self{
                    #(#from_row),*
                })
            }
        }
        impl #impl_generics FromRow for #name #ty_generics #where_clause {
            fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
                Self::from_row_ref(&row)
            }
        }
    }
    .into()
}

fn from_row(table_name: &str, data: &syn::Data) -> Vec<TokenStream> {
    let mut columns = Vec::new();

    if let syn::Data::Struct(data_struct) = data {
        for field in &data_struct.fields {
            let field_type = &field.ty;
            let field_name = field.ident.as_ref().expect("");
            let col_name = format!("{table_name}_{field_name}");
            let expect = format!("Column '{col_name}' not found");

            columns.push(if get_attribute(&field.attrs, "ForeignKey").is_some() {
                quote! { #field_name: #field_type::from_row_ref(row)? }
            } else {
                quote! { #field_name: row.get(#col_name).expect(#expect) }
            });
        }
    }

    columns
}

fn get_attribute<'a>(
    attrs: &'a [syn::Attribute],
    attribute: &'static str,
) -> Option<&'a Attribute> {
    attrs.iter().find(|&attr| attr.path().is_ident(attribute))
}
