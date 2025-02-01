#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use syn::{parse_macro_input, ItemFn, LitStr};

/// Define data-file-driven tests using JSON/YAML files.
///
/// This attribute macro reads a JSON/YAML file at compile time and generates a test function for each
/// test case in the file. The test function must take a single argument, which is a structured type
/// that implements `serde::Deserialize`.
/// The file is read from the file system relative to the current working directory of the
/// compiler.
///
/// Note that `serde` and `serde_json` crate is required in caller's `Cargo.toml`.
///
/// # Example
/// ```rust
/// use datafile_test::datafile_test;
///
/// #[derive(Debug, serde::Deserialize)]
/// struct TestCaseInput {
///     a: i32,
///     b: i32,
/// }
///
/// #[derive(Debug, serde::Deserialize)]
/// struct TestCase {
///     input: TestCaseInput,
///     output: String,
/// }
///
/// #[datafile_test("tests/testcase.yml")]
/// fn test(testcase: TestCase) {
///     assert_eq!(testcase.input.a + testcase.input.b, testcase.output);
/// }
/// ```
///
/// The yaml file should look like this:
///
/// ```yaml
/// - input:
///     a: 1
///     b: 2
///   expect: 3
/// - input:
///     a: 2
///     b: 3
///   expect: 5
/// ```
///
#[proc_macro_attribute]
pub fn datafile_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute
    let attr = parse_macro_input!(attr as LitStr);
    let file_path = attr.value();

    // Parse the function item
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let fn_args = &input_fn.sig.inputs;

    // Ensure the function has exactly one argument
    if fn_args.len() != 1 {
        return syn::Error::new_spanned(
            &input_fn.sig,
            "datafile_test function must have exactly one argument",
        )
        .to_compile_error()
        .into();
    }

    let test_case_type = match fn_args.first().unwrap() {
        syn::FnArg::Typed(pat_type) => &pat_type.ty,
        _ => {
            return syn::Error::new_spanned(
                &input_fn.sig,
                "datafile_test function must take a structured argument",
            )
            .to_compile_error()
            .into();
        }
    };

    // Load JSON/YAML file at compile time
    let data_text = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            return syn::Error::new_spanned(
                &attr,
                format!("Failed to read data file '{:?}': {}", &file_path, e),
            )
            .to_compile_error()
            .into();
        }
    };

    let ext = std::path::Path::new(&file_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default()
        .to_lowercase();
    // Parse JSON/YAML into Vec<serde_json::Value>
    let test_cases: Vec<serde_json::Value> = match ext.as_str() {
        "json" => match serde_json::from_str(&data_text) {
            Ok(cases) => cases,
            Err(e) => {
                return syn::Error::new_spanned(
                    &attr,
                    format!("Failed to parse JSON file '{:?}': {}", &file_path, e),
                )
                .to_compile_error()
                .into();
            }
        },
        "yaml" | "yml" => match serde_yaml::from_str(&data_text) {
            Ok(cases) => cases,
            Err(e) => {
                return syn::Error::new_spanned(
                    &attr,
                    format!("Failed to parse YAML file '{:?}': {}", &file_path, e),
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &attr,
                format!("Unsupported file extension: {:?}", ext),
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate test functions for each case
    let test_fns: Vec<_> = test_cases
        .iter()
        .enumerate()
        .map(|(i, test_case)| {
            let test_fn_name = format_ident!("{}_case_{}", fn_name, i);

            // Convert serde_yaml::Value to JSON string
            let json_str = match serde_json::to_string(test_case) {
                Ok(s) => s,
                Err(e) => {
                    return syn::Error::new_spanned(
                        &attr,
                        format!("Failed to convert test case to JSON: {}", e),
                    )
                    .to_compile_error();
                }
            };

            // Convert JSON string to Rust expression
            let test_case_expr: syn::Expr = match syn::parse_str(&format!(
                "serde_json::from_str::<{}>({:?}).unwrap()",
                quote!(#test_case_type),
                json_str
            )) {
                Ok(expr) => expr,
                Err(e) => {
                    return syn::Error::new_spanned(
                        &attr,
                        format!("Failed to parse test case JSON as Rust expression: {}", e),
                    )
                    .to_compile_error();
                }
            };

            quote! {
                #[test]
                fn #test_fn_name() {
                    let testcase: #test_case_type = #test_case_expr;
                    #fn_body
                }
            }
        })
        .collect();

    let output = quote! {
        #(#test_fns)*
    };

    output.into()
}
