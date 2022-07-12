use super::macro_args::MacroArgs;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput,
    Field, Fields,
};

fn create_get_size_field(field: &Field) -> proc_macro2::TokenStream {
    let field_ident = field.ident.as_ref().expect("Field should have identity");
    let (pad_before, backtrace) = match MacroArgs::from_attributes(&field.attrs) {
        Some(MacroArgs {
            pad_before,
            backtrace,
        }) => (
            pad_before.unwrap_or_default(),
            backtrace.unwrap_or_default(),
        ),
        _ => (0, 0),
    };

    quote! {
      size += #pad_before;
      size -= #backtrace;
      size += ::no_std_io::EndianWrite::get_size(&self.#field_ident);
    }
}

fn create_write_field(
    field: &Field,
    field_method: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_ident = field.ident.as_ref().expect("Field should have identity");
    let attributes = match MacroArgs::from_attributes(&field.attrs) {
        Some(MacroArgs {
            pad_before,
            backtrace,
        }) => {
            let pad_before = if let Some(pad_before) = pad_before {
                quote! { ::no_std_io::Cursor::increment_by(&mut stream, #pad_before); }
            } else {
                quote! {}
            };

            let backtrace = if let Some(backtrace) = backtrace {
                quote! {
                    let current_index = ::no_std_io::Cursor::get_index(&stream);
                    ::no_std_io::Cursor::set_index(&mut stream, current_index - #backtrace);
                }
            } else {
                quote! {}
            };

            quote! {
                #backtrace
                #pad_before
            }
        }
        _ => quote! {},
    };

    quote! {
      #attributes
      ::no_std_io::StreamWriter::#field_method(&mut stream, &self.#field_ident)?;
    }
}

fn create_write_method_impl(
    fields: &Punctuated<Field, Comma>,
    impl_method: proc_macro2::TokenStream,
    field_method: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_tokens = fields
        .iter()
        .map(|field| create_write_field(field, &field_method))
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
      #[inline(always)]
      fn #impl_method(&self, dst: &mut [u8]) -> Result<usize, ::no_std_io::Error> {
        let mut stream = ::no_std_io::StreamContainer::new(dst);
        #(#field_tokens)*
        let bytes_written = ::no_std_io::Cursor::get_index(&stream);
        Ok(bytes_written)
      }
    }
}

pub fn impl_endian_write(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let named_fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs can derive EndianWrite"),
    };

    let get_size_fields = named_fields
        .iter()
        .map(create_get_size_field)
        .collect::<Vec<proc_macro2::TokenStream>>();

    let try_write_le = create_write_method_impl(
        &named_fields,
        quote! { try_write_le },
        quote! { write_stream_le },
    );

    let try_write_be = create_write_method_impl(
        &named_fields,
        quote! { try_write_be },
        quote! { write_stream_be },
    );

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let modified = quote! {
        impl #impl_generics ::no_std_io::EndianWrite for #name #ty_generics #where_clause {
          fn get_size(&self) -> usize {
            let mut size = 0;
            #(#get_size_fields)*
            size
          }

          #try_write_le
          #try_write_be
        }
    };

    modified.into()
}
