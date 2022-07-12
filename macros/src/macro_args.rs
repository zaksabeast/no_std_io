use darling::FromMeta;
use syn::Attribute;

#[derive(Debug, FromMeta)]
pub struct MacroArgs {
    pub pad_before: Option<usize>,
    pub backtrace: Option<usize>,
}

impl MacroArgs {
    pub fn from_attribute(attr: &Attribute) -> Option<Self> {
        attr.parse_meta()
            .ok()
            .and_then(|meta| MacroArgs::from_meta(&meta).ok())
    }

    pub fn from_attributes(attrs: &[Attribute]) -> Option<Self> {
        attrs.iter().find_map(Self::from_attribute)
    }
}
