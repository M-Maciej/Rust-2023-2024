use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    if let Ok(ast) = syn::parse(input) {
        impl_hello_macro(&ast)
    } else {
        panic! {"error deriving Hello_macro"};
    }
}
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote!(
        impl HelloMacro for #name{
            fn hello_macro(){
            println!("Hello Macro, my name is {}",stringify!(#name));
        }}

    );
    gen.into()
}
