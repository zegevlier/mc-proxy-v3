use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    fold::Fold, parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, ItemStatic, Token,
    Visibility,
};

// struct Packet;

// impl Fold for Packet {
//     fn fold_field(&mut self, i: syn::Field) -> syn::Field {
//         let mut field = i;
//         field.vis = Visibility::Public(syn::VisPublic {
//             pub_token: ItemStatic,
//         });
//         field
//     }
// }

#[proc_macro]
pub fn packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let mut fields = quote!();
    match input.data {
        Data::Struct(ds) => {
            for field in ds.fields {
                let name = field.ident;
                let ty = field.ty;
                fields.extend(quote!(pub #name: #ty,));
            }
        }
        Data::Enum(_) | Data::Union(_) => todo!(),
    }

    let expanded = quote! {
        pub struct #ident {
            #fields
        }
    };

    proc_macro::TokenStream::from(expanded)
}
