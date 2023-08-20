use std::{fmt::Debug, mem::size_of};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Token,  punctuated::Punctuated, Meta};


#[proc_macro_attribute]
pub fn cfg_matrix(attr: TokenStream, item: TokenStream) -> TokenStream {
    
    

    // Parse the input as a trait
    let input = syn::parse_macro_input!(item as syn::ItemTrait);

    // Parse the attrs as an array of Type: TokenTree
    let attr = syn::parse_macro_input!(attr as CfgMatrixInputs).0;

    // Maximum of usize
    if attr.len() > 4 {
        panic!("Cfg Matrix only supports 4 attributes for now. Read the notes section in the readme to learn more..")
    }

    // Get the number of combinations we will need
    let num_combs = 2 ^ (attr.len() - 1);

    // Extract the cfg feature
    let cfgs = attr.iter().map(|v| {
        &v.cfg
    }).collect::<Vec<_>>();

    // Create each trait
    let mut traits = TokenStream::new();

    for i in 0..=num_combs {

        // Generate the cfg from a bitmap of the current i:
        // 0 means not(), 1 means enabled.
        let flagged = cfgs.iter().enumerate().map(|(j, v)| {
            // Shift and check
            if (i >> j) & 1 == 1 {
                // If it is enabled, just return v
                (*v).clone()
            } else {
                // Otherwise, return not(v)
                syn::parse(quote! {
                    not(#v)
                }.into()).unwrap()
            }
        }).collect::<Vec<Meta>>();

        // Get a iterator of supertraits
        let supertraits = attr.iter().enumerate().filter_map(|(j, v)| {
            if (i >> j) & 1 == 1 {
                Some(v.typename.clone())
            } else {
                None
            }
        });

        // Clone the input
        let mut input = input.clone();

        // Update the supertraits
        input.supertraits.extend(supertraits);

        let out = quote! {
            #[cfg(all(#(#flagged),*))]

            #input
        }.into();

        

        // Add the new permutation
        traits.extend::<TokenStream>(out);
    }

    traits
}

#[derive(Debug)]
struct CfgMatrixInputs(Vec<CfgMatrixInput>);

impl Parse for CfgMatrixInputs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let res = Punctuated::<CfgMatrixInput, Token![,]>::parse_terminated(&input)?;
        
        Ok(Self(res.into_iter().collect()))
    }
}


struct CfgMatrixInput {
    typename: syn::TypeParamBound,
    cfg: syn::Meta
}

impl Debug for CfgMatrixInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CfgMatrixInput")
        .field("typename", &self.typename.to_token_stream())
        .field("cfg", &self.cfg.to_token_stream()).finish()
    }
}

impl Parse for CfgMatrixInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse a type name
        let typename: syn::TypeParamBound = input.parse()?;
        input.parse::<Token![:]>()?;
        let cfg: syn::Meta = input.parse()?;

        Ok(CfgMatrixInput {
            typename, cfg
        })
    }
}