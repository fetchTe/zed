mod derive_path_str;

use proc_macro::TokenStream;

/// Derives the `path` method for an enum.
///
/// This macro generates a `path` method for each variant of the enum, which returns a string
/// representation of the enum variant's path. The path is constructed using a prefix and
/// optionally a suffix, which are specified using attributes.
///
/// # Attributes
///
/// - `#[path_str(prefix = "...")]`: Required. Specifies the prefix for all paths.
/// - `#[path_str(suffix = "...")]`: Optional. Specifies a suffix for all paths.
/// - `#[strum(serialize_all = "...")]`: Optional. Specifies the case conversion for variant names.
///
/// # Example
///
/// ```
/// use strum::EnumString;
/// use ui_macros::{path_str, DerivePathStr};
///
/// #[derive(EnumString, DerivePathStr)]
/// #[path_str(prefix = "my_prefix", suffix = ".txt")]
/// #[strum(serialize_all = "snake_case")]
/// enum MyEnum {
///     VariantOne,
///     VariantTwo,
/// }
///
/// // These assertions would work if we could instantiate the enum
/// // assert_eq!(MyEnum::VariantOne.path(), "my_prefix/variant_one.txt");
/// // assert_eq!(MyEnum::VariantTwo.path(), "my_prefix/variant_two.txt");
/// ```
///
/// # Panics
///
/// This macro will panic if used on anything other than an enum.
#[proc_macro_derive(DerivePathStr, attributes(path_str))]
pub fn derive_path_str(input: TokenStream) -> TokenStream {
    derive_path_str::derive_path_str(input)
}

/// A marker attribute for use with `DerivePathStr`.
///
/// This attribute is used to specify the prefix and suffix for the `path` method
/// generated by `DerivePathStr`. It doesn't modify the input and is only used as a
/// marker for the derive macro.
#[proc_macro_attribute]
pub fn path_str(_args: TokenStream, input: TokenStream) -> TokenStream {
    // This attribute doesn't modify the input, it's just a marker
    input
}