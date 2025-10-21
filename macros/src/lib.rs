use proc_macro::{TokenStream};

use syn::{
    parse::{Parse, ParseStream}, 
    parse_macro_input, 
    DeriveInput, 
    Expr, 
    Lit, Token
};

use quote::quote;

struct ScreenArgs {
    title: Option<Expr>,
    description: Option<String>,
    layout: Option<Expr>,
}

impl Parse for ScreenArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut layout = None;
        let mut description = None;
        let mut title = None;

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            match key.to_string().as_str() {
                "title" => {
                    let expr: Expr = input.parse()?;
                    title = Some(expr);
                }
                "description" => {
                    let lit: Lit = input.parse()?;
                    if let Lit::Str(s) = lit {
                        description = Some(String::from(s.value()));
                    } else {
                        return Err(
                            syn::Error::new_spanned(
                                lit, "Expected string literal for attribute 'description'."
                            )
                        )
                    }
                }
                "layout" => {
                    let expr: Expr = input.parse()?;
                    layout = Some(expr);
                }
                _ => {
                    return Err(
                        syn::Error::new_spanned(
                            key, "Unknown key in #[screen(...)] attribute"
                        )
                    )
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(ScreenArgs { layout, description, title })
    }
}

#[proc_macro_derive(Screen, attributes(screen))]
pub fn derive_screen(input: TokenStream) -> TokenStream {
    // Parse and retrieve derived struct's name:
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;    

    // Parse attributes:
    for attr in &input.attrs {
        // Get attribute name:
        let ident = attr.path().get_ident().unwrap();
        match ident.to_string().as_str() {
            "screen" => {
                let args = attr.parse_args::<ScreenArgs>().expect(
                    "Failed to parse #[screen(...)] attribute arguments"
                );
            }
            _ => {
                
            }
        }
    }
    let expanded = quote! { 
        impl Default for #struct_name {
            fn default() -> Self {
                #struct_name {
                    screen: 
                }
            }
        }
        // struct #struct_name {

        // }
    };
    TokenStream::from(expanded)
}
