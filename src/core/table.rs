pub trait Table {
    type Item;

    fn insert(&mut self, name: &str, value: Self::Item);
    fn is_empty(&self) -> bool;
    fn contains_key(&self, name: &str) -> bool;
    fn get(&self, name: &str) -> Option<&Self::Item>;
}
