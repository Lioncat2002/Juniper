use core::fmt;

use super::{builder::Builder, table::Table};

pub struct Code<T> {
    pub symbols: Vec<(usize, String)>,
    pub code: Vec<usize>,
    pub data: Vec<T>,
    pub labels: Vec<(usize, String)>,
}

impl<T> Code<T> {
    pub fn get_label_ip(&self, name: &str) -> Option<usize> {
        for label in self.labels.as_slice() {
            if label.1 == name {
                return Some(label.0);
            }
        }
        None
    }
}

impl<'a, T: std::fmt::Debug + std::cmp::PartialEq> From<Builder<'a, T>> for Code<T> {
    fn from(builder: Builder<T>) -> Code<T> {
        let symbols = builder.instruction_table.symbols();
        let code = builder.instructions;
        let data = builder.data;
        let mut labels = vec![];
        for key in builder.labels.keys() {
            let idx = builder.labels.get(&key).unwrap();
            labels.push((*idx, key.clone()));
        }
        labels.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));

        Code {
            symbols,
            code,
            data,
            labels,
        }
    }
}
