mod sqlite;
mod utils;

use crate::sqlite::derive_sqlite_table;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// 实现Table 的 Derive
#[proc_macro_derive(ITable, attributes(table, column))]
pub fn table_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_sqlite_table(input)
}
