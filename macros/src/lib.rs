use proc_macro::TokenStream;

mod endian_read;
mod endian_write;
mod macro_args;

#[proc_macro_derive(EndianRead, attributes(no_std_io))]
pub fn impl_endian_read(tokens: TokenStream) -> TokenStream {
    endian_read::impl_endian_read(tokens)
}

#[proc_macro_derive(EndianWrite, attributes(no_std_io))]
pub fn impl_endian_write(tokens: TokenStream) -> TokenStream {
    endian_write::impl_endian_write(tokens)
}
