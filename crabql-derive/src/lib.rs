use proc_macro::TokenStream;
use quote::quote;
use syn::*;

/// Implements `IntoMulti<Box<dyn CoreExpression>>` for the given type
#[proc_macro_derive(IntoMultiCore)]
pub fn intomulticore_derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    let Item::Struct(item_struct) = item else {
        panic!("`IntoMulti` can only be derived on structs.")
    };

    let name = &item_struct.ident;
    let expanded = quote! {
        impl std::ops::Shl<#name> for () {
            type Output = MultiExpr<Box<dyn CoreExpression>>;

            fn shl(self, rhs: #name) -> Self::Output {
                MultiExpr::new(Box::new(rhs))
            }
        }

        impl std::ops::Shl<#name> for MultiExpr<Box<dyn CoreExpression>> {
            type Output = MultiExpr<Box<dyn CoreExpression>>;

            fn shl(self, rhs: #name) -> Self::Output {
                let mut multi = self;
                multi.push(Box::new(rhs));
                multi
            }
        }

        impl IntoMulti<Box<dyn CoreExpression>> for #name {}
        impl IntoMultiCore for #name {}
    };

    expanded.into()
}

/// Implements `IntoMulti<Self>` for the given type
#[proc_macro_derive(IntoMultiMisc)]
pub fn intomultimisc_derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    let Item::Struct(item_struct) = item else {
        panic!("`IntoMulti` can only be derived on structs.")
    };

    let name = &item_struct.ident;
    let expanded = quote! {
        #item_struct

        impl std::ops::Shl<#name> for () {
            type Output = MultiExpr<#name>;

            fn shl(self, rhs: #name) -> Self::Output {
                MultiExpr::new(Box::new(rhs))
            }
        }

        impl std::ops::Shl<#name> for MultiExpr<#name> {
            type Output = MultiExpr<#name>;

            fn shl(self, rhs: #name) -> Self::Output {
                let mut multi = self;
                multi.push(Box::new(rhs));
                multi
            }
        }

        impl IntoMulti<#name> for #name {}
        impl IntoMultiMisc for #name {}
    };

    expanded.into()
}
