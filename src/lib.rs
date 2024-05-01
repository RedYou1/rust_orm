extern crate proc_macro;

use proc_macro::TokenStream;

/// # Panics
/// Will panic if cant parse the input
#[proc_macro_derive(Table)]
pub fn table_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("failed to parse the input");
    let struct_name = &ast.ident;

    let syn::Data::Struct(ref data_struct) = ast.data else {
        panic!("Binary derive only supports structs.")
    };

    let mut field_declarations = Vec::new();

    for field in &data_struct.fields {
        let field_name = field
            .ident
            .as_ref()
            .expect("Field must have an identifier")
            .to_string();
        //let field_type = &field.ty;

        field_declarations.push(quote::quote! {
            Column::new(formatcp!("{}",#field_name)),
        });
    }

    let struct_name_str = struct_name.to_string();

    quote::quote! {
        impl Table for #struct_name {
            fn table_name() -> &'static str {
                formatcp!("{}",#struct_name_str)
            }
            fn columns() -> Vec<Column> {
                vec![
                    #(#field_declarations)*
                ]
            }
        }
    }
    .into()
}
