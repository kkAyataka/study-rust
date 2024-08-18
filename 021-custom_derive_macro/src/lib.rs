use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, Stmt};

#[proc_macro_attribute]
pub fn trace(attr: TokenStream, item: TokenStream) -> TokenStream {
    dbg!(&attr);
    dbg!(&item);

    let mut ast = parse_macro_input!(item as ItemFn);
    dbg!(&ast);
    let _ident = &ast.sig.ident;
    let new_stmt = quote! {
        {
            println!("trace {}::{} {}", module_path!(), stringify!(#_ident), line!());
            println!("test");
        }
    };
    let new_stmt: TokenStream = new_stmt.into();
    let new_stmt = parse_macro_input!(new_stmt as Stmt);

    //ast.block.stmts.clear();
    ast.block.stmts.insert(0, new_stmt);

    let gen = quote! {
        #ast
    };

    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    //dbg!(&input);

    let ast: DeriveInput = syn::parse(input).unwrap();
    //dbg!(&ast);

    // Build the trait implementation
    let name = ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
            fn hello_macro1(&self) {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
