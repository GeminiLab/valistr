use proc_macro::TokenStream as TokenStream1;
use proc_macro2::Span;
use quote::{quote, quote_spanned, TokenStreamExt};
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    spanned::Spanned,
    Field, Fields, FieldsNamed, Ident, ItemStruct,
};

mod utils;

struct ValistrArgs {
    regex: String,
    // for future features
    // container_field: Option<String>,
    // hook_fn: Option<String>,
}

impl Parse for ValistrArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let regex = input.parse::<syn::LitStr>()?.value();
        Ok(ValistrArgs { regex })
    }
}

#[proc_macro_attribute]
pub fn valistr(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let mut input = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as ValistrArgs);

    if input.fields.len() > 0 {
        return quote_spanned!(input.fields.span() => compile_error!("Only unit structs are supported");).into();
    }

    // collect regex
    let regex_str = utils::ensure_regex_anchors(&args.regex);
    let regex_lit = syn::LitStr::new(&regex_str, Span::call_site());
    let regex = regex::Regex::new(&regex_str).unwrap();

    // collect named groups with simple identifiers
    let named_groups = regex
        .capture_names()
        .enumerate()
        .filter_map(|(index, name)| {
            name.filter(|name| utils::is_simple_ident(*name))
                .map(|name| (index, name.to_string()))
        })
        .collect::<Vec<_>>();

    // create the field `value: String` to store the value
    let mut fields = FieldsNamed::parse.parse2(quote!({value: String})).unwrap();

    // create the fields to store the capture groups, [`regex::Captures`] cannot be used directly here
    for (_, group_name) in &named_groups {
        let group_name_ident = Ident::new(group_name, Span::call_site());
        fields.named.push(
            Field::parse_named
                .parse2(quote!(#group_name_ident: Option<(usize, usize)>))
                .unwrap(),
        );
    }

    // set the new fields
    input.fields = Fields::Named(fields);

    // create the `validator` method
    let validator = quote!(
        pub fn validator() -> &'static regex::Regex {
            static VALIDATOR: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            VALIDATOR.get_or_init(|| regex::Regex::new(#regex_lit).unwrap())
        }
    );

    // create the getter methods
    let mut getter_methods = quote!();
    for (_, group_name) in &named_groups {
        let group_name_ident = Ident::new(group_name, Span::call_site());
        let get_method_ident = Ident::new(&format!("get_{}", group_name), Span::call_site());
        let get_method = quote!(
            pub fn #get_method_ident(&self) -> Option<&str> {
                self.#group_name_ident.map(|(start, end)| &self.value[start..end])
            }
        );
        getter_methods.append_all(get_method);
    }

    // create the `new` method
    let mut new_fn_capture_group_mappers = quote!();
    for (index, group_name) in &named_groups {
        let group_name_ident = Ident::new(group_name, Span::call_site());
        let capture_group_mapper = quote!(
            let #group_name_ident = captures.get(#index).map(|m| (m.start(), m.end()));
        );
        new_fn_capture_group_mappers.append_all(capture_group_mapper);
    }

    let named_group_names: Vec<_> = named_groups
        .iter()
        .map(|(_, name)| Ident::new(name, Span::call_site()))
        .collect();
    let new = quote!(
        pub fn new(value: impl Into<String>) -> Option<Self> {
            let validator = Self::validator();
            let value = value.into();

            if let Some(captures) = validator.captures(&value) {
                #new_fn_capture_group_mappers

                Some(Self {
                    value,
                    #(#named_group_names,)*
                })
            } else {
                None
            }
        }
    );

    let struct_name = &input.ident;

    quote!(
        #input

        impl #struct_name {
            #validator
            #getter_methods
            #new
        }

        #[automatically_derived]
        impl std::ops::Deref for #struct_name {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        #[automatically_derived]
        impl std::ops::DerefMut for #struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.value
            }
        }

        #[automatically_derived]
        impl std::convert::TryFrom<&str> for #struct_name {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(())
            }
        }

        #[automatically_derived]
        impl std::convert::TryFrom<String> for #struct_name {
            type Error = ();

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(())
            }
        }

        #[automatically_derived]
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.value, f)
            }
        }

        #[automatically_derived]
        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.value, f)
            }
        }
    )
    .into()
}
