// Function composition:
//    - Function composition is the process of combining two or more functions to produce a new function.
//    - The result of each function is passed as an argument to the next function.
//
// The macro invocation should look like so:
//    - compose!(h -> g -> f)(x) == f(g(h(x)))
//
// This module defines a procedural macro for function composition.

use proc_macro::TokenStream;
use syn::{parse::Parse, Ident};

// The `Composed` struct represents the parsed input for the `compose!` macro.
// It contains the first function and a list of additional functions to compose.
struct Composed {
    function: Ident,    // The first function in the composition chain.
    others: Vec<Ident>, // A vector of additional functions to apply in sequence.
}

// Implement the `Parse` trait for `Composed` to enable parsing the macro input.
impl Parse for Composed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse the first function identifier.
        let function = input.parse()?;
        let mut others = Vec::new();

        // Parse additional functions separated by the `->` token.
        while input.parse::<syn::Token![->]>().is_ok() {
            let other: Ident = input.parse()?;
            others.push(other);
        }

        // Return the parsed `Composed` struct.
        Ok(Composed { function, others })
    }
}

// Implement the `ToTokens` trait for `Composed` to generate the composed function.
impl quote::ToTokens for Composed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let func = &self.function;

        // Start with the first function applied to the input `x`.
        let mut composed = quote::quote! {
            #func(x)
        };

        // Chain the remaining functions in reverse order (right-to-left composition).
        for other in &self.others {
            composed = quote::quote! {
                #other(#composed)
            };
        }

        // Output the generated code.
        composed.to_tokens(tokens);
    }
}

// The `compose` procedural macro entry point.
#[proc_macro]
pub fn compose(input: TokenStream) -> TokenStream {
    // Parse the input into a `Composed` struct.
    let composed = syn::parse_macro_input!(input as Composed);

    // Generate a closure that takes an input `x` and applies the composed functions.
    let out = quote::quote! {
        |x| #composed
    };

    // Return the generated code as a `TokenStream`.
    out.into()
}
