use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_str, Path};
use crate::sqlite::meta::{TableInfo};

/// 实现Table
pub fn impl_table(_info: TableInfo, input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let table_name = _info.table.clone();
    eprintln!("Call {:#?}", _info);
    let _call_token_result = parse_str::<Path>("hello");
    eprintln!("Call {:#?}", _call_token_result);
    let mut columns = quote!();
    let fields = Vec::from_iter(_info.fields.iter().filter(|c| {!c.exclude}).map(|m|{m.clone()}));
    for _column_info in fields {
        let field = &_column_info.field;
        let column = &_column_info.column;
        columns.extend(quote!{
            vec.push(rick_core::sqlite::TableColumn {
            field: #field,
            column: #column
            });
        })
    }
    if let Err(_err) = _call_token_result {
        return quote!().into();
        // return TokenStream::from((Error::custom(format!("conn 语法有误 {}", _err.to_string()))).write_errors())
    }
    let tokens = _call_token_result.unwrap();
    (quote! {
        impl rick_core::sqlite::Table for #struct_name {
            fn table_name() -> &'static str {
                #table_name
            }
            fn conn() -> rick_core::sqlite::Connection {
                #tokens()
            }
            /// 字段内容
            fn columns() -> Vec<rick_core::sqlite::TableColumn> {
                let mut vec = Vec::<rick_core::sqlite::TableColumn>::new();
                #columns
                vec
            }
        }
    }).into()
}