use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Data, Fields, ImplItemMethod, Stmt, Type, ExprStruct, FieldValue};
use quote::*;

#[proc_macro_derive(Parse, attributes(TokenSet))]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut token_set = None;
    assert!(input.attrs.len() == 1);
    for attr in &input.attrs {
        let t = attr.tokens.clone();
        let t = t.into();
        let t = parse_macro_input!(t as Type);
        if let Type::Paren(type_paren) = t {
            token_set = Some(type_paren.elem.as_ref().clone());
        } else {
            panic!("not paren?");
        }
    }
    let token_set = token_set.unwrap();
    let data = input.data;
    let ident = input.ident;
    let res = match data {
        Data::Struct(data_struct) => {
            let base = TokenStream::from(quote! {
                fn parse(tokens: &mut Tokens<#token_set>) -> ParserResult<#ident> {
                }
            });
            let mut base = parse_macro_input!(base as ImplItemMethod);

            let stmt = TokenStream::from(quote! {
                let initial_i = tokens.get_i();
            });
            base.block.stmts.push(parse_macro_input!(stmt as Stmt));
            if let Fields::Named(fields_named) = data_struct.fields {
                for item in &fields_named.named {
                    let ident = item.ident.clone().unwrap();
                    let ty = item.ty.clone();
                    let stmt = TokenStream::from(quote! {
                        let #ident = if let ParserResult::Ok(t) = tokens.parse::<#ty>() {
                            t
                        } else {
                            tokens.set_i(initial_i);
                            return ParserResult::Fail;
                        };
                    });
                    base.block.stmts.push(parse_macro_input!(stmt as Stmt));
                }
                let res = TokenStream::from(quote! {
                    #ident {
                    }
                });
                let mut res = parse_macro_input!(res as ExprStruct);
                for item in &fields_named.named {
                    let ident = item.ident.clone().unwrap();
                    let field = TokenStream::from(quote! {
                        #ident
                    });
                    res.fields.push(parse_macro_input!(field as FieldValue));
                }
                let stmt = TokenStream::from(quote! {
                    let res = ParserResult::Ok(#res);
                });
                base.block.stmts.push(parse_macro_input!(stmt as Stmt));
                let stmt = TokenStream::from(quote! {
                    return res;
                });
                base.block.stmts.push(parse_macro_input!(stmt as Stmt));

            } else {
                panic!("not named field");
            }
            quote! {
                impl Parse<#token_set> for #ident {
                    #base
                }
            }
        }
        Data::Enum(data_enum) => {
            let base = TokenStream::from(quote! {
                fn parse(tokens: &mut Tokens<#token_set>) -> ParserResult<#ident> {
                }
            });
            let mut base = parse_macro_input!(base as ImplItemMethod);

            let stmt = TokenStream::from(quote! {
                let initial_i = tokens.get_i();
            });
            base.block.stmts.push(parse_macro_input!(stmt as Stmt));
            for item in &data_enum.variants {
                let ident2 = item.ident.clone();
                if let Fields::Unnamed(fields_unnamed) = &item.fields {
                    let first_ty = fields_unnamed.unnamed.iter().next().unwrap().clone().ty;
                    let stmt = TokenStream::from(quote! {
                        if let ParserResult::Ok(t) = tokens.parse::<#first_ty>() {
                            return ParserResult::Ok(#ident::#ident2(t));
                        } else {
                            tokens.set_i(initial_i);
                        };
                    });
                    base.block.stmts.push(parse_macro_input!(stmt as Stmt));
                } else {
                    panic!("not Fields::Unnamed");
                }
            }
            let stmt = TokenStream::from(quote! {
                return ParserResult::Fail;
            });
            base.block.stmts.push(parse_macro_input!(stmt as Stmt));
            quote! {
                impl Parse<#token_set> for #ident {
                    #base
                }
            }
        }
        _ => panic!(),
    };
    TokenStream::from(res)
}

#[proc_macro_derive(TokenSet)]
pub fn derive_token_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = input.data;
    let ident = input.ident;
    let res = match data {
        Data::Enum(data_enum) => {
            let mut res = vec![];
            res.push(TokenStream::from(quote!{
                impl TokenSet for #ident {}
            }));
            for variant in data_enum.variants {
                let variant_ident = variant.ident.clone();
                if let Fields::Unnamed(fields_unnamed) = variant.fields {
                    let first = fields_unnamed.unnamed.iter().next().unwrap().clone();
                    res.push(TokenStream::from(quote!{
                        impl TokenSetMatch<#ident> for #first {
                            fn token_match(set: &#ident) -> Option<#first> {
                                match set {
                                    #ident::#variant_ident(t) => Some(t.clone()),
                                    _ => None
                                }
                            }
                        }
                    }));
                    res.push(TokenStream::from(quote!{
                        impl Parse<#ident> for #first {
                            fn parse(tokens: &mut Tokens<#ident>) -> ParserResult<#first> {
                                let initial_i = tokens.get_i();
                                let t = tokens.get_token();
                                if let Some(t) = t.token_match::<#first>() {
                                    let res = t.clone();
                                    tokens.next();
                                    ParserResult::Ok(res)
                                } else {
                                    ParserResult::Fail
                                }
                            }
                        }
                    }));
                }
            }
            res
        }
        _ => panic!(),
    };
    let mut res2 = TokenStream::new();
    res2.extend(res);
    res2
}
