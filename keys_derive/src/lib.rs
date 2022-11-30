use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(SharedKeys)]
pub fn shared(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_shared(&ast)
}

fn impl_shared(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SharedKeys for #name {
            fn shared() -> std::rc::Rc<Self> {
                std::rc::Rc::new(Self::default())
            }
        }
    };
    gen.into()
}
