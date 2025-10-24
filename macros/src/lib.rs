use proc_macro::{TokenStream};

use syn::{
    parse::{Parse, ParseStream}, 
    parse_macro_input, 
    DeriveInput, 
    Expr, 
    Lit, 
    Meta, 
    Token, 
    TypeTuple
};

use quote::{quote, ToTokens};

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
                        description = Some(
                            String::from(s.value())
                        );
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

fn parse_field_doc_lines(field: &syn::Field) -> Vec<String> {
    let mut doc_lines = Vec::new();
    for attr in &field.attrs {
        if attr.path().is_ident("doc") {
            let meta = attr.meta.require_name_value().unwrap();
            if let syn::Expr::Lit(l) = &meta.value {
                if let syn::Lit::Str(str) = &l.lit {
                    doc_lines.push(str.value().trim().to_owned());
                }
            }
        }
    }
    doc_lines
}

fn parse_field_doc_lines_as_paragraph(field: &syn::Field) -> Lit {
    let doc_lines = parse_field_doc_lines(field);
    let doc_text = doc_lines.join(" ");
    syn::parse_quote!(#doc_text)
}

fn parse_field_doc_lines_as_list(field: &syn::Field) -> Vec<Lit> {
    let mut doc_lines = Vec::new();
    for attr in &field.attrs {
        if attr.path().is_ident("doc") {
            let meta = attr.meta.require_name_value().unwrap();
            if let syn::Expr::Lit(l) = &meta.value {
                doc_lines.push(l.lit.clone());
            }
        }
    }
    doc_lines
}

#[proc_macro_derive(Screen, attributes(screen, faust))]
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
                unreachable!()
            }
        }
    }
    // Parse struct fields if any:
    let mut mthchain = Vec::new();
    match &input.data {
        syn::Data::Struct(s) => {
            for field in &s.fields {
                // Match the type of the field:
                match &field.ty {
                    // If tuple: 
                    // first field should have Paragraph type
                    // second should be derived from InteractiveWidget
                    syn::Type::Tuple(t) => {
                        if let Some(f) = t.elems.first() {
                            if let syn::Type::Path(p) = f {
                                if let Some(ident) = p.path.get_ident() {
                                    match ident.to_string().as_str() {
                                        "ScreenParagraph"  => {
                                            let lit = parse_field_doc_lines_as_paragraph(field);
                                            mthchain.push(quote! {
                                                .add_paragraph(crate::leafy!(#lit))
                                            });
                                        }
                                        "ScreenList" => {
                                            let vec = parse_field_doc_lines_as_list(field);
                                            mthchain.push(quote! {
                                                .add_list(vec![
                                                    #(#vec)*
                                                ])
                                            });
                                        }
                                        _ => ()
                                    }
                                }
                            }
                        }
                        if let Some(s) = t.elems.last() {
                            if let syn::Type::Path(p) = s {
                                let ident = p.path.get_ident().unwrap();
                                // TODO: Get the type, generate compile time assertion
                                // if it doesn't implement InteractiveWidget trait
                                match ident.to_string().as_str() {
                                    "FaustWidget" => {
                                        // Parse field attribute 'faust'
                                        for attr in &field.attrs {
                                            let ident = attr.path().get_ident().unwrap();
                                            match ident.to_string().as_str() {
                                                "faust" => {
                                                    let file: Expr = attr.parse_args().unwrap();
                                                    mthchain.push(quote! {
                                                        .add_widget(Box::new(
                                                            FaustWidget::new(#file)
                                                        ))
                                                    });                                                    
                                                }
                                                _ => ()
                                            }
                                        }
                                    }
                                    _ => {
                                        mthchain.push(quote! {
                                            .add_widget(Box::new(#ident::default()))
                                        });
                                    }
                                }

                            }
                        }
                    }
                    syn::Type::Path(p) => {
                        match p.path.get_ident().unwrap().to_string().as_str() {
                            "ScreenParagraph" => {
                                let lit = parse_field_doc_lines_as_paragraph(field);
                                mthchain.push(quote! {
                                    .add_paragraph(#lit)
                                });                                
                            }
                            "ScreenList" => {
                                let vec = parse_field_doc_lines_as_list(field);
                                mthchain.push(quote! {
                                    .add_list(vec![
                                        #(#vec),*
                                    ])
                                });
                            }
                            _ => ()
                        }
                    }
                    _ => ()
                }
            }
        }
        _ => ()
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
                println!("Building");
                let layout: Option<Box<dyn Layout>> = match #layout_expr {
                    LayoutEnum::None => None,
                    LayoutEnum::SideBySide => {
                        Some(
                            Box::new(
                                SideBySide::default()
                                .add_title(#title_str)
                                #(#mthchain)*
                            )
                        )
                    }
                    LayoutEnum::Plainfull => {
                        Some(
                            Box::new(
                                PlainFull::default()
                                .add_title(#title_str)
                                #(#mthchain)*
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
