mod meta;
mod impl_table;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Field, Fields};
use darling::{Error, FromMeta};
use crate::sqlite::impl_table::impl_table;
use crate::sqlite::meta::{TableFieldInfo, TableFieldMeta, TableInfo, TableMeta};
use crate::utils::find_attribute;

/// 获取字段信息
pub fn get_table_field_info(_field: Field) -> TableFieldInfo {
    let mut field = TableFieldInfo {column: String::new(), exclude: false, id: false,
        field: _field.ident.unwrap().to_string(), ty: _field.ty.clone(),
        default: None};
    if let Some(_column_attribute) = find_attribute(_field.attrs, "column") {
        if let Ok(_meta) = TableFieldMeta::from_meta(&_column_attribute.meta) {
            if let Some(_exclude) = _meta.exclude {
                field.exclude = _exclude;
            }
            if let Some(_id) = _meta.id {
                field.id = _id;
            }
            field.default = _meta.default;
            if let Some(_column) = _meta.column {
                field.column.push_str(_column.as_str());
                return field;
            }

        }
    }
    field.column.push_str(field.field.as_str());
    field
}

pub fn get_table_meta(input: &DeriveInput) -> Result<TableInfo, Error> {
    // 获取Struct Attribute
    let mut table_info = TableInfo { table : String::new(), conn: String::new(), fields: Vec::new()};
    if let Some(_table_attribute) = find_attribute(input.attrs.clone(), "table") {
        match TableMeta::from_meta(&_table_attribute.meta) {
            Ok(mut _meta) => {
                if let Some(ref _table_name) = _meta.table {
                    table_info.table.push_str(_table_name.as_str());
                } else {
                    table_info.table.push_str(input.ident.to_string().as_str());
                }
                // conn
                if let Some(ref _conn) = _meta.conn {
                    table_info.conn.push_str(_conn.as_str());
                } else {
                    return Err(Error::custom("#[table] 格式错误, conn 配置必填 #[table(conn=\"必填\")]"));
                }

            }
            Err(_err) => {
                return Err(Error::custom("#[table] 格式错误, #[table(table = \"表名称\", conn = \"::sqlite\")]"));
            }
        }
    }
    // 解析所有字段
    if let Data::Struct(ref _struct) = input.data {
        match _struct.fields.clone() {
            Fields::Named(_fields) => {
                for _field in _fields.named {
                    table_info.fields.push(get_table_field_info(_field))
                }
            }
            Fields::Unnamed(_fields) => {
                return Err(Error::custom("无法解析此 struct 格式 请使用 Struct Named 格式"));
            }
            _=> {
                return Err(Error::custom("空 struct 请编写字段"));
            }
        }
    } else {
        return Err(Error::custom("当前版本只支持 struct 结构"));
    }
    // 获取去所有字段
    Ok(table_info)
}


/// 解析Toke
///
pub fn derive_sqlite_table(input: DeriveInput) -> TokenStream {
    match get_table_meta(&input)  {
        Ok(_info) => {
            impl_table(_info, input)
        }
        Err(_error) => {
            TokenStream::from(_error.write_errors())
        }
    }
}