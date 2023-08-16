use proc_macro::{TokenStream};
use proc_macro2::Spacing::{Alone, Joint};
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_str, Path};
use crate::sqlite::meta::{TableInfo};

/// 实现Table
pub fn impl_table(_info: TableInfo, input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let table_name = _info.table.clone();
    let _call_token_result = parse_str::<Path>(_info.conn.as_str());
    let mut columns = quote!();
    let mut from_fields = quote!();
    let mut bind_fields = quote!();
    let mut bind_index_fields = quote!();
    let mut update_set_fields = quote!();
    let fields = Vec::from_iter(_info.fields.iter().map(|m|{m.clone()}));
    let mut column_index = 0usize;
    for i in 0..fields.len() {
        let _column_info = fields.get(i).unwrap();
        let field = &_column_info.field;
        let column = &_column_info.column;
        let id = &_column_info.id;
        let ty = &_column_info.ty;
        let f = format_ident!("{}", field);
        if !_column_info.exclude {
            columns.extend(quote!{
                vec.push(rick_core::sqlite::TableColumn {
                    field: #field,
                    column: #column,
                    id: #id
                });
            });
            let bind_index = format!(":{}", column);
            bind_fields.extend(quote!{
               let val = Into::<rick_core::sqlite::SqlValue>::into(&self.#f);
                if let Err(_err) = statement.bind((#bind_index, &val)) {
                    return Err(_err);
                }
            });
            bind_index_fields.extend(quote!{
                let val = Into::<rick_core::sqlite::SqlValue>::into(&self.#f);
                if let Err(_err) = statement.bind((start_index + #column_index, &val)) {
                    return Err(_err);
                }
            });
            let is_id = _column_info.id;
            update_set_fields.extend(quote!{
                if ! #is_id || include_id {
                    update = update.set(#column, &self.#f);
                }
            });
            column_index = column_index + 1;
        }
        if i > 0 {
            from_fields.extend(quote!{,
            });
        }
        if _column_info.exclude {
            let ty = process_generic(quote!{#ty});
            // let ty = process_generic(quote!{Option::<i32>::default()});
            // 处理泛型
            from_fields.extend(quote!{
                #f: #ty::default()
            });
        } else {
            from_fields.extend(quote!{
                #f: rick_core::sqlite::To::<#ty>::to(&(row.read::<rick_core::sqlite::SqlValue, _>(#column)))
            });
        }
    }
    if let Err(_err) = _call_token_result {
        return quote!().into();
    }
    let tokens = _call_token_result.unwrap();
    let mut output = quote! {
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
    };
    output.extend(quote! {
        impl From<sqlite::Row> for #struct_name {
            fn from(row: sqlite::Row) -> Self {
                Self {
                    #from_fields
                }
            }
        }

    });
    output.extend(quote! {
        impl rick_core::sqlite::SaveBind for #struct_name {
            fn bind(&self, statement: &mut sqlite::Statement) -> Result<(), rick_core::sqlite::SqlError> {
                #bind_fields
                Ok(())
            }
            fn bind_index(&self, statement: &mut sqlite::Statement, start_index: usize) -> Result<(), rick_core::sqlite::SqlError> {
                #bind_index_fields
                Ok(())
            }

            fn update_set<T: rick_core::sqlite::Table>(&self, mut update: rick_core::sqlite::UpdateWrapper<T>, include_id: bool) ->  rick_core::sqlite::UpdateWrapper<T> {
                #update_set_fields
                update
            }
        }
    });
    output.into()
}

fn process_generic(ty: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut vec = Vec::new();
    let mut count = 0;
    for token in ty {
        if let proc_macro2::TokenTree::Punct(_punct)  = token {
            if _punct.as_char() == '<'{
                if count == 0 {
                    vec.push(proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(':', Joint)));
                    vec.push(proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(':', Alone)));
                }
               count = count + 1;
            }
            if _punct.as_char() == '>'{
                count = count - 1;
            }
            vec.push(proc_macro2::TokenTree::Punct(_punct));
        } else {
            vec.push(token);
        }
    }
    proc_macro2::TokenStream::from_iter(vec.into_iter())
}