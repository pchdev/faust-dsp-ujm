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

    let mut title = None;
    let mut descr = None;
    let mut layout = None;

    // Parse attributes:
    for attr in &input.attrs {
        // Get attribute name:
        let ident = attr.path().get_ident().unwrap();
        match ident.to_string().as_str() {
            "screen" => {
                let args = attr.parse_args::<ScreenArgs>().expect(
                    "Failed to parse #[screen(...)] attribute arguments"
                );
                if args.title.is_some() {
                    title = args.title;
                }
                if args.description.is_some() {
                    descr = args.description;
                }
                if args.layout.is_some() {
                    layout = args.layout;
                }
            }
            _ => {
                unreachable!("nope")
            }
        }
    }
    let title_str = title.expect(
        "Missing #[screen(title = ...)]"
    );
    let descr_str = descr.expect(
        "Missing #[screen(description = ...)]"
    );
    let layout_expr = layout
        .map(|expr| quote! { #expr })
        .unwrap_or_else(|| quote! { LayoutEnum::None })
    ;
    
    let expanded = quote! { 
        impl Screen for #struct_name {
            fn title(&self) -> &'static str {
                #title_str
            }
            fn description(&self) -> &'static str {
                #descr_str
            }
            fn build() -> (Box<dyn Screen>, Option<Box<dyn Layout>>) {
                let layout: Option<Box<dyn Layout>> = match #layout_expr {
                    LayoutEnum::None => None,
                    LayoutEnum::SideBySide => {
                        Some(
                            Box::new(
                                SideBySide::default()
                            )
                        )
                    }
                    LayoutEnum::Plainfull => {
                        Some(
                            Box::new(
                                PlainFull::default()
                            )
                        )
                    }
                };
                (Box::new(#struct_name::default()), layout)
            }
        }
    };
    TokenStream::from(expanded)
}
