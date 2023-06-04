use crate::model::quote::Quote;
pub trait ShredDB {
    fn insert_quote(&self, content: &str);

    fn query_quotes_limit(&self, num: u64) -> Vec<Quote>;

    fn query_quotes(&self) -> Vec<Quote>;

    fn delete(&self, id: u64);

    fn update(&self, id: u64, new_content: &str);

    fn backup(&self);
}
