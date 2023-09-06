use syn::Attribute;

/// 查找属性
pub fn find_attribute(_iter: Vec<Attribute>, path: &'static str) -> Option<Attribute> {
    _iter
        .iter()
        .find(|attr| attr.path().is_ident(path))
        .map(|a| a.clone())
}
