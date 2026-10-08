use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
  Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument,
  Ident, PathArguments, PathSegment, Type,
};

#[proc_macro_derive(Pixmap)]
pub fn pixmap_derive(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as DeriveInput);

  let name = input.ident;
  let fields = get_named_fields(input.data);

  let mut has_width = false;
  let mut has_height = false;

  let mut pixel_type = None;

  for field in fields {
    let field_name = field.ident.unwrap();

    if field_name == Ident::new("width", Span::call_site()) {
      has_width = true;
    } else if field_name == Ident::new("height", Span::call_site()) {
      has_height = true;
    } else if field_name == Ident::new("pixels", Span::call_site()) {
      let Type::Path(path_type) = field.ty else {
        unimplemented!()
      };

      let PathSegment { arguments, .. } =
        path_type.path.segments.first().unwrap();

      let PathArguments::AngleBracketed(path_args) = arguments else {
        unimplemented!()
      };

      let generic = path_args.args.first().unwrap();

      pixel_type = match generic {
        GenericArgument::Type(ty) => Some(ty.clone()),
        _ => unimplemented!(),
      };
    }
  }

  if !has_width {
    panic!("`width` field does not exist")
  }
  if !has_height {
    panic!("`height` field does not exist")
  }

  let Some(pixel_type) = pixel_type else {
    panic!("`pixels` field does not exist")
  };

  let generated = quote! {
    impl Pixmap for #name {
      type Pixel = #pixel_type;

      fn width(&self) -> u32 {
        self.width
      }

      fn height(&self) -> u32 {
        self.height
      }

      fn pixels(&self) -> &[#pixel_type] {
        &self.pixels
      }

      fn pixels_mut(&mut self) -> &mut [#pixel_type] {
        &mut self.pixels
      }

      fn scale_up(&mut self, factor: usize) {
        crate_pixmap_core::ops::scale_up_in_place(
          &mut self.width,
          &mut self.height,
          &mut self.pixels,
          factor,
        )
      }
    }

    impl IntoIterator for #name {
      type Item = u8;
      type IntoIter = crate_pixmap_core::IntoIter<
        #pixel_type,
        { size_of::<<#pixel_type as Pixel>::ByteArray>() },
      >;

      fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.pixels)
      }
    }
  };

  generated.into()
}

#[proc_macro_derive(Pixel)]
pub fn pixel_derive(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as DeriveInput);

  let name = input.ident;
  let fields = get_named_fields(input.data);

  let num_fields = fields.len();
  let field_names: Vec<_> = fields
    .into_iter()
    .map(|field| field.ident.unwrap())
    .collect();

  let generated = quote! {
    impl Pixel for #name {
      type ByteArray = [u8; #num_fields];

      fn from_bytes([#(#field_names),*]: Self::ByteArray) -> Self {
        Self { #(#field_names),* }
      }

      fn into_bytes(self) -> Self::ByteArray {
        let Self { #(#field_names),* } = self;
        [#(#field_names),*]
      }
    }
  };

  generated.into()
}

fn get_named_fields(data: Data) -> Punctuated<Field, Comma> {
  match data {
    Data::Struct(DataStruct { fields, .. }) => match fields {
      Fields::Named(FieldsNamed { named, .. }) => named,
      _ => unimplemented!(),
    },
    _ => unimplemented!(),
  }
}
