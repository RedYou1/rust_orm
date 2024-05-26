use syn::Attribute;

#[must_use]
pub fn get_attribute<'a>(attrs: &'a [Attribute], attribute: &'static str) -> Option<&'a Attribute> {
    attrs.iter().find(|&attr| attr.path().is_ident(attribute))
}

pub const PRIMARY_KEY: &str = "PrimaryKey";
pub const FOREIGN_KEY: &str = "ForeignKey";
pub const TABLE_NAME: &str = "table_name";
pub const OPTION: &str = "std::option::Option";
pub const VEC: &str = "alloc::vec::Vec";

#[must_use]
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
