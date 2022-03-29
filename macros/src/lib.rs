use proc_macro::TokenStream;

mod endian_read;
mod endian_write;

#[proc_macro_derive(EndianRead)]
pub fn impl_endian_read(tokens: TokenStream) -> TokenStream {
    endian_read::impl_endian_read(tokens)
}

#[proc_macro_derive(EndianWrite)]
pub fn impl_endian_write(tokens: TokenStream) -> TokenStream {
    endian_write::impl_endian_write(tokens)
}
