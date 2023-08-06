mod sqlite;
mod utils;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use crate::sqlite::derive_sqlite_table;


/// 实现Table 的 Derive
#[proc_macro_derive(Table, attributes(table, column))]
pub fn table_derive(input: TokenStream) -> TokenStream {
    let input:  DeriveInput = parse_macro_input!(input);
    derive_sqlite_table(input)
}