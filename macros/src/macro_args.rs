use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(no_std_io))]
pub struct MacroArgs {
    pub pad_before: usize,
}
