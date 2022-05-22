extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{ItemEnum, ItemStruct, ItemType};

fn get_name(item: &proc_macro2::TokenStream) -> Option<Ident> {
    let ident = if let Ok(ItemStruct { ident, .. }) = syn::parse2(item.clone()) {
        ident
    } else if let Ok(ItemEnum { ident, .. }) = syn::parse2(item.clone()) {
        ident
    } else if let Ok(ItemType { ident, .. }) = syn::parse2(item.clone()) {
        ident
    } else {
        return None;
    };

    Some(ident)
}

#[allow(clippy::too_many_lines)]
fn stateroom_wasm_impl(item: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let name =
        get_name(item).expect("Can only use #[stateroom_wasm] on a struct, enum, or type alias.");

    quote! {
        #item

        mod _stateroom_wasm_macro_autogenerated {
            extern crate alloc;

            use super::*;
            use stateroom_wasm::prelude::{GlobalStateroomContext, ROOM_FUTURE};

            #[no_mangle]
            extern "C" fn initialize() {
                initialize_room::<#name>();
            }
        }
    }
}

/// Exposes a `stateroom_wasm::SimpleStateroomService`-implementing trait as a WebAssembly module.
#[proc_macro_attribute]
pub fn stateroom_wasm(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[allow(clippy::needless_borrow)]
    stateroom_wasm_impl(&item.into()).into()
}

#[cfg(test)]
mod test {
    use super::get_name;
    use quote::quote;

    #[test]
    fn test_parse_name() {
        assert_eq!(
            "MyStruct",
            get_name(&quote! {
                struct MyStruct {}
            })
            .unwrap()
            .to_string()
        );

        assert_eq!(
            "AnotherStruct",
            get_name(&quote! {
                struct AnotherStruct;
            })
            .unwrap()
            .to_string()
        );

        assert_eq!(
            "ATupleStruct",
            get_name(&quote! {
                struct ATupleStruct(u32, u32, u32);
            })
            .unwrap()
            .to_string()
        );

        assert_eq!(
            "AnEnum",
            get_name(&quote! {
                enum AnEnum {
                    Option1,
                    Option2(u32),
                }
            })
            .unwrap()
            .to_string()
        );

        assert_eq!(
            "ATypeDecl",
            get_name(&quote! {
                type ATypeDecl = u32;
            })
            .unwrap()
            .to_string()
        );

        assert!(get_name(&quote! {
            impl Foo {}
        })
        .is_none());
    }
}
