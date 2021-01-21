use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(_item).unwrap();

    println!(">>> struct ident: {}", ast.ident);
    for attr in &ast.attrs {
        println!(">>> struct attr: {}{}", attr.path.get_ident().unwrap(), attr.tokens);
    }

    if let syn::Data::Struct(ref s) = ast.data {
        if let syn::Fields::Named(ref x) = s.fields {
            for field in &x.named {
                println!(">>> field ident: {}", field.ident.as_ref().unwrap());
                for a in &field.attrs {
                    println!(">>> field attr: {}{}", a.path.get_ident().unwrap(), a.tokens);
                }
            }

        }
    }
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn show_stream(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!(">>> attr: {}", attr.to_string());
    println!(">>> item: {}", item.to_string());
    item
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
