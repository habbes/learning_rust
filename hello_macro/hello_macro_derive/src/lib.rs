use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)] // the allows the HelloMacro to be called when user specified #[derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // ast.ident returns an Ident struct instance containing the name of
    // the annotated type
    let name = &ast.ident;
    let gen = quote! {
        // the quote macro has some templating capabalities, like we can use #name here
        impl HelloMacro for #name {
            fn hello_macro() {
                // the built-in stringify macro converts the expression into a string literal
                // Rust doesn't have runtime reflection capabilities. Macros
                // allow us to get the type name at compile-time.
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // converts the intermediate representation from quote! into a TokenStream
    gen.into()
}
