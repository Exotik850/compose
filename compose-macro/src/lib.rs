use proc_macro::TokenStream;
use syn::{parse::Parse, punctuated::Punctuated, UseName};

// Function composition:
//    - Function composition is the process of combining two or more functions to produce a new function.
//    - The result of each function is passed as an argument to the next function.
// 
// The macro invocation should look like so:
//    - compose!(h -> g -> f)(x) == f(g(h(x)))
// 

struct Composed {
    function: syn::Ident,
    others: Vec<syn::Ident>,
}

impl Parse for Composed {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let function = input.parse()?;
    let mut others = Vec::new();
    while let Ok(_) = input.parse::<syn::Token![->]>() {
      let other: syn::Ident = input.parse()?;
      others.push(other);
    }
    Ok(Composed { function, others })
  }
}

impl quote::ToTokens for Composed {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {

      let func = &self.function;

      let mut composed = quote::quote! {
          #func(x)
      };

      for other in &self.others {
          composed = quote::quote! {
              #other(#composed)
          };
      }
 
      composed.to_tokens(tokens);
      
  }
}

#[proc_macro]
pub fn compose(input: TokenStream) -> TokenStream {
    let composed = syn::parse_macro_input!(input as Composed);
    let out = quote::quote! {
        |x| #composed
    };
    out.into()
}