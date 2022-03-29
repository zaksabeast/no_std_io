use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput,
    Field, Fields,
};

fn create_field(
    field: &Field,
    field_method: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_ident = field.ident.as_ref().expect("Field should have identity");
    quote! {
        #field_ident: ::no_std_io::StreamReader::#field_method(&mut stream)?
    }
}

fn create_method_impl(
    fields: &Punctuated<Field, Comma>,
    impl_method: proc_macro2::TokenStream,
    field_method: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_tokens = fields
        .iter()
        .map(|field| create_field(field, &field_method))
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        fn #impl_method(bytes: &[u8]) -> Result<::no_std_io::ReadOutput<Self>, ::no_std_io::Error> {
            let mut stream = ::no_std_io::StreamContainer::new(bytes);
            let result = Self {
                #(#field_tokens),*
            };
            let bytes_read = stream.get_index();

            Ok(::no_std_io::ReadOutput::new(result, bytes_read))
        }
    }
}

pub fn impl_endian_read(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let named_fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs can derive EndianRead"),
    };

    let try_read_le = create_method_impl(
        &named_fields,
        quote! { try_read_le },
        quote! { read_stream_le },
    );

    let try_read_be = create_method_impl(
        &named_fields,
        quote! { try_read_be },
        quote! { read_stream_be },
    );

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let modified = quote! {
        impl #impl_generics ::no_std_io::EndianRead for #name #ty_generics #where_clause {
            #try_read_le
            #try_read_be
        }
    };

    modified.into()
}
