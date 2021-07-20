// Commenting this out makes RA properly find `settings::Settings` in
// `$ROOT/src/lib.rs`.
mod settings {}

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn settings(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
