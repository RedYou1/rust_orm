extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use rust_query_lib_macro::{get_attribute, get_inner_type, FOREIGN_KEY, OPTION, TABLE_NAME, VEC};
use syn::{parse_macro_input, DeriveInput};

/// # Panics
/// Will panic if cant parse the input
#[proc_macro_derive(MySQLRow, attributes(ForeignKey, table_name))]
pub fn mysqlrow_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

        let (flattens, from_row) = from_row(table_name.as_str(), data_struct);

        let flatten = quote! {
            impl #impl_generics RowFlatten<#name #ty_generics> for [#name #ty_generics] #where_clause {
                fn row_flatten(&self) -> Vec<#name #ty_generics> {
                    let mut result: Vec<#name #ty_generics> = Vec::new();
                    for s in self {
                        let mut toadd = true;
                        for r in &mut result {
                            if r.id_eq(s) {
                                #(#flattens)*
                                toadd = false;
                                break;
                            }
                        }
                        if toadd {
                            result.push(s.clone());
                        }
                    }
                    result
                }
            }
        };
        let rows = quote! {
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
        };

        if flattens.is_empty() {
            rows
        } else {
            quote! {
                #flatten
                #rows
            }
        }
        .into()
    } else {
        panic!("Only on Struct")
    }
}

fn from_row(table_name: &str, data_struct: &syn::DataStruct) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut columns = Vec::new();

    let mut flattens = Vec::new();

    for field in &data_struct.fields {
        let mut field_type = &field.ty;
        let field_name = field.ident.as_ref().expect("Not in a Tuple");
        let col_name = format!("{table_name}_{field_name}");
        let expect = format!("Column '{col_name}' not found");

        columns.push(if get_attribute(&field.attrs, FOREIGN_KEY).is_some() {
            let mut has_vec = false;
            while let Some(ty) = get_inner_type(OPTION, field_type) {
                field_type = ty;
            }
            while let Some(ty) = get_inner_type(VEC, field_type) {
                field_type = ty;
                has_vec = true;
            }
            while let Some(ty) = get_inner_type(OPTION, field_type) {
                field_type = ty;
            }
            if has_vec {
                flattens.push(quote!{
                    r.#field_name.extend(s.#field_name.clone());
                });
                quote! { #field_name: vec![#field_type::from_row_ref(row)?] }
            } else {
                quote! { #field_name: #field_type::from_row_ref(row)? }
            }
        } else {
            quote! { #field_name: row.get(#col_name).expect(#expect) }
        });
    }

    (flattens, columns)
}
