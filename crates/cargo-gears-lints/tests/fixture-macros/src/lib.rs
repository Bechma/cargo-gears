use proc_macro::TokenStream;

/// No-op attribute macro used by UI tests to simulate `toolkit_macros::domain_model`.
#[proc_macro_attribute]
pub fn domain_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
