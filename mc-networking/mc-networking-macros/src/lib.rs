use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

#[proc_macro_derive(VarintEnum)]
pub fn derive_varint_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("VarintEnum can only be derived for enums"),
    };

    let mut decodes = Vec::new();
    let mut encodes = Vec::new();
    let mut i: usize = 0;
    for variant in variants {
        if let Some((_, expr)) = variant.clone().discriminant {
            // This lets the A = 1 syntax work (will yell loudly if it's not a number)
            let expr = syn::parse2::<syn::Expr>(expr.to_token_stream()).unwrap();
            let lit = syn::parse2::<syn::Lit>(expr.to_token_stream()).unwrap();
            let int = syn::parse2::<syn::LitInt>(lit.to_token_stream()).unwrap();
            let value = int.base10_parse::<usize>().unwrap();
            i = value;
        }
        let ident = variant.ident.clone();

        let idx = syn::Index::from(i);
        decodes.push(quote_spanned! {variant.span()=>
            #idx => Ok(#name::#ident),
        });

        encodes.push(quote_spanned! {variant.span()=>
            #name::#ident => Varint::from(#idx).encode(buf),
        });
        i += 1;
    }

    let expanded = quote! {
        impl crate::traits::McEncodable for #name {
            fn decode(buf: &mut impl std::io::Read) -> color_eyre::Result<Self> {
                match Varint::decode(buf)?.value() {
                    #(#decodes)*
                    _ => Err(color_eyre::eyre::eyre!("Invalid state")),
                }
            }

            fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
                match self {
                    #(#encodes)*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(McEncodable)]
pub fn derive_mcencodable_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => panic!("VarintEnum can only be derived for enums"),
    };

    let mut decodes = Vec::new();
    let mut encodes = Vec::new();
    for field in fields {
        let ident = field.clone().ident.unwrap();
        decodes.push(quote_spanned! {field.span()=>
            #ident: crate::traits::McEncodable::decode(buf)?,
        });

        encodes.push(quote_spanned! {field.span()=>
            self.#ident.encode(buf)?;
        });
    }

    let expanded = quote! {
        impl crate::traits::McEncodable for #name {
            fn decode(buf: &mut impl std::io::Read) -> color_eyre::Result<Self> {
                Ok(Self {
                    #(#decodes)*
                })
            }

            fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
                #(#encodes)*
                Ok(())
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Version)]
pub fn derive_version_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("Version can only be derived for enums"),
    };

    let mut to_ids = Vec::new();
    let mut from_ids = Vec::new();
    for variant in variants {
        let value = match variant.clone().discriminant {
            Some((_, expr)) => {
                // This lets the A = 1 syntax work (will yell loudly if it's not a number)
                let expr = syn::parse2::<syn::Expr>(expr.to_token_stream()).unwrap();
                let lit = syn::parse2::<syn::Lit>(expr.to_token_stream()).unwrap();
                let int = syn::parse2::<syn::LitInt>(lit.to_token_stream()).unwrap();
                int.base10_parse::<i32>().unwrap()
            }
            _ => panic!("Version enum variants must have discriminants"),
        };

        let ident = variant.ident.clone();

        to_ids.push(quote_spanned! {variant.span()=>
            #name::#ident => Some(#value),
        });
        from_ids.push(quote_spanned! {variant.span()=>
            #value => Some(#name::#ident),
        });
    }

    let expanded = quote! {
        impl #name {
            pub fn from_id(id: i32) -> Option<Self> {
                match id {
                    #(#from_ids)*
                    _ => None,
                }
            }

            pub fn to_id(&self) -> Option<i32> {
                match self {
                    #(#to_ids)*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
