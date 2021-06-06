use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Block, ImplItemMethod, Stmt, Type};
use quote::*;
use std::iter::FromIterator;

#[proc_macro_derive(Parse, attributes(TokenSet))]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut token_set = None;
    assert!(input.attrs.len() == 1);
    for attr in &input.attrs {
        let t = attr.tokens.clone();
        let t = t.into();
        let t = parse_macro_input!(t as Type);
        token_set = Some(t);
    }
    let token_set = token_set.unwrap();
    let data = input.data;
    let ident = input.ident;
    let res = match data {
        Data::Struct(data_struct) => {
            let mut stmts = TokenStream::new();
            let stmt = TokenStream::new();
            let stmt = TokenStream::from(quote! {
                println!("a");
            });
            let stmt = parse_macro_input!(stmt as Stmt);
            let base = TokenStream::from(quote! {
                fn parse(tokens: &mut Tokens<#token_set>) -> ParserResult<#ident> {
                    unimplemented!();
                }
            });
            let mut base = parse_macro_input!(base as ImplItemMethod);
            // base.block.stmts.push(stmt);
            /*
            let res = TokenStream::new();
            */

            /*
            quote! {
                impl Parse<#TokenSet> for #ident {
                    fn parse(tokens: TokenSet<#TokenSet>) -> ParserResult<#ident> {
                        #stmts
                        unimplemented!()
                    }
                }
            }
            */
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
                            fn token_match(set: &#ident) -> bool {
                                matches!(set, #ident::#variant_ident(_))
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
