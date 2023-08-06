use std::marker::PhantomData;
use crate::sqlite::Table;

pub struct Query<T: Table> {
    _mark: PhantomData<T>,
}