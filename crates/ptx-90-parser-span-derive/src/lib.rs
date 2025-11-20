use proc_macro::TokenStream;
use proc_macro2::Span as ProcSpan;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Spanned)]
pub fn derive_spanned(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_spanned(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn impl_spanned(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (span_arms, set_arms) = match &input.data {
        Data::Struct(data) => {
            let (span_arm, set_arm) = build_match_arm(quote! { Self }, &data.fields)?;
            (vec![span_arm], vec![set_arm])
        }
        Data::Enum(data) => {
            let mut span_arms = Vec::new();
            let mut set_arms = Vec::new();
            for variant in &data.variants {
                let ident = &variant.ident;
                let path = quote! { Self::#ident };
                let (span_arm, set_arm) = build_match_arm(path, &variant.fields)?;
                span_arms.push(span_arm);
                set_arms.push(set_arm);
            }
            (span_arms, set_arms)
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "Spanned cannot be derived for unions",
            ));
        }
    };

    let span_ty = quote! { crate::parser::Span };
    let trait_path = quote! { crate::span::Spanned };
    let span_match = quote! {
        match self {
            #(#span_arms)*
        }
    };
    let set_match = quote! {
        match self {
            #(#set_arms)*
        }
    };

    Ok(quote! {
        impl #impl_generics #trait_path for #name #ty_generics #where_clause {
            fn span(&self) -> #span_ty {
                #span_match
            }

            fn set_span(&mut self, span: #span_ty) {
                #set_match
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn span(&self) -> #span_ty {
                <Self as #trait_path>::span(self)
            }

            pub fn with_span(mut self, span: #span_ty) -> Self {
                <Self as #trait_path>::set_span(&mut self, span);
                self
            }
        }
    })
}

fn build_match_arm(
    path: proc_macro2::TokenStream,
    fields: &Fields,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    match fields {
        Fields::Named(named) => build_named_arm(path, named),
        _ => Err(syn::Error::new(
            ProcSpan::call_site(),
            "Spanned derive only supports structs/enums with named `span` fields",
        )),
    }
}

fn build_named_arm(
    path: proc_macro2::TokenStream,
    fields: &FieldsNamed,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let has_span = fields
        .named
        .iter()
        .any(|field| field.ident.as_ref().is_some_and(|ident| ident == "span"));
    if !has_span {
        return Err(syn::Error::new(
            ProcSpan::call_site(),
            "Spanned derive requires a field named `span`",
        ));
    }

    let binding = format_ident!("__span_field");
    let span_arm = quote! {
        #path { span: #binding, .. } => #binding.clone(),
    };
    let set_arm = quote! {
        #path { span: #binding, .. } => {
            *#binding = span;
        }
    };

    Ok((span_arm, set_arm))
}
