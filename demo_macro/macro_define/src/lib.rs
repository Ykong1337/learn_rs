use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn test_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("{:#?}", attr);
    eprintln!("{:#?}", item);
    item
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_driver(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro(){
                println!("hello_macro,trait: {}",stringify!(#name));
            }
        }
    };
    gen.into()
}