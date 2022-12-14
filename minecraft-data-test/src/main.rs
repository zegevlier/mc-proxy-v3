#![allow(dead_code, unused_variables)]

use convert_case::{Case, Casing};
use minecraft_data_rs::{
    api::{versions_by_minecraft_version, Api},
    models::protocol::{NativeType, PacketDataType},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn main() -> anyhow::Result<()> {
    let api = Api::new(
        versions_by_minecraft_version()?
            .get("1.19.2")
            .unwrap()
            .clone(),
    );

    let protocol = api.protocols.get_protocol()?;
    // for pdt in protocol.types.types {
    //     println!("{:?}", pdt);
    //     println!("{}", pdt_to_code(&pdt));
    // }
    for packet in protocol.handshaking.to_server.types.iter() {
        let c = pdt_to_code(&packet.data);
        let generated_code: String = quote!(structstruck::strike! {
            #c
        })
        .to_string();
        let mut output = Vec::new();
        let (_summary, out, _global) = rustfmt::format_input(
            rustfmt::Input::Text(generated_code),
            &Default::default(),
            Some(&mut output),
        )
        .unwrap();
        println!("{}", out[0].1);
    }
    Ok(())
}

fn pdt_to_code(pdt: &PacketDataType) -> TokenStream {
    match pdt {
        PacketDataType::Native(native_type) => native_type_to_token_stream(native_type),
        PacketDataType::UnknownNativeType(_) => todo!(),
        PacketDataType::Built { name, value } => {
            let name = format_ident!("{}", name.to_string().to_case(Case::Pascal));
            let value = native_type_to_token_stream(value);
            quote! {
                struct #name {
                    #value
                }
            }
        }
        PacketDataType::Other { name, value } => {
            if name.is_some() {
                match name.as_ref().unwrap() {
                    minecraft_data_rs::models::protocol::types::TypeName::Anonymous => todo!(),
                    minecraft_data_rs::models::protocol::types::TypeName::Named(name) => {
                        if name == "string" {
                            quote!(String)
                        } else if name == "restBuffer" {
                            quote!(Vec<u8>)
                        } else if name == "UUID" {
                            quote!(Uuid)
                        } else {
                            todo!("\"{}\" is not implemented for PacketDataType::Other", name)
                        }
                    }
                }
            } else {
                todo!()
            }
        }
    }
}

fn native_type_to_token_stream(native_type: &NativeType) -> TokenStream {
    match native_type {
        NativeType::VarInt => quote!(VarInt),
        NativeType::PString { count_type } => {
            let count_type = native_type_to_token_stream(count_type);
            quote!(PString<#count_type>)
        }
        NativeType::Buffer { count_type } => todo!(),
        NativeType::Bool => quote!(bool),
        NativeType::U8 => quote!(u8),
        NativeType::U16 => quote!(u16),
        NativeType::U32 => quote!(u32),
        NativeType::U64 => quote!(u64),
        NativeType::I8 => quote!(i8),
        NativeType::I16 => quote!(i16),
        NativeType::I32 => quote!(i32),
        NativeType::I64 => quote!(i64),
        NativeType::F32 => quote!(f32),
        NativeType::F64 => quote!(f64),
        NativeType::Uuid => quote!(Uuid),
        NativeType::Option(inner) => {
            let inner = pdt_to_code(inner);
            quote!(Option<#inner>)
        }
        NativeType::EntityMetadataLoop {
            end_val,
            metadata_type,
        } => todo!(),
        NativeType::TopBitSetTerminatedArray(_) => todo!(),
        NativeType::BitField(_) => todo!(),
        NativeType::Container(value) => {
            let mut fields = TokenStream::new();
            for (name, ptd) in value {
                let name = format_ident!("{}", name.to_string().to_case(Case::Snake));
                let ptd = pdt_to_code(ptd);
                fields.extend(quote! {
                    #name: #ptd,
                });
            }
            quote! {
                #fields
            }
        }
        NativeType::Switch {
            compare_to,
            fields,
            default,
        } => {
            let mut cases = TokenStream::new();
            for (case, st) in fields {
                let case = format_ident!("{}", case);
                let compare_to = format_ident!("{}", compare_to);
                let st = match st {
                    minecraft_data_rs::models::protocol::types::SwitchType::Packet(_) => todo!(),
                    minecraft_data_rs::models::protocol::types::SwitchType::Type(pdt) => {
                        nameless_pdt_to_code(pdt)
                    }
                    minecraft_data_rs::models::protocol::types::SwitchType::Unknown(_) => todo!(),
                };
                cases.extend(quote! {
                    #compare_to::#case => #st,
                });
            }
            assert!(default.is_none());
            cases
        }
        NativeType::Void => todo!(),
        NativeType::Array {
            count_type,
            array_type,
        } => {
            let count_type = native_type_to_token_stream(count_type);
            let array_type = nameless_pdt_to_code(array_type);
            quote!(
                Array<#count_type, (#array_type)>
            )
        }
        NativeType::RestBuffer => todo!(),
        NativeType::NBT => todo!(),
        NativeType::OptionalNBT => todo!(),
        _ => todo!(),
    }
}

fn nameless_pdt_to_code(pdt: &PacketDataType) -> TokenStream {
    match pdt {
        PacketDataType::Native(native_type) => match native_type {
            NativeType::Container(value) => {
                let mut fields = TokenStream::new();
                for (name, ptd) in value {
                    let name = format_ident!("{}", name.to_string().to_case(Case::Snake));
                    let ptd = pdt_to_code(ptd);
                    fields.extend(quote! {
                        #name: #ptd,
                    });
                }
                quote! {
                    struct {
                        #fields
                    }
                }
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
}
