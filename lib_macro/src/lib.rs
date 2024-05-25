use syn::Attribute;

pub fn get_attribute<'a>(attrs: &'a [Attribute], attribute: &'static str) -> Option<&'a Attribute> {
    attrs.iter().find(|&attr| attr.path().is_ident(attribute))
}

pub const PRIMARY_KEY: &'static str = "PrimaryKey";
pub const FOREIGN_KEY: &'static str = "ForeignKey";
pub const TABLE_NAME: &'static str = "table_name";
pub const OPTION: &'static str = "std::option::Option";
pub const VEC: &'static str = "alloc::vec::Vec";

pub fn get_inner_type<'a>(full_name: &'static str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
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

    if !full_name.ends_with(full_name_found.as_str()) {
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
