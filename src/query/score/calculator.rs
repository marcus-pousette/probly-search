use crate::index::{DocumentDetails, DocumentPointer, FieldDetails};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

pub struct TermData<'a> {
    pub query_term: &'a str,
    pub query_term_expanded: &'a str,
    pub all_query_terms: &'a Vec<String>,
}

pub struct FieldData<'a> {
    pub field_lengths: &'a [usize],
    pub fields_boost: &'a [f64],
    pub fields: &'a [FieldDetails],
}

/**
Implement this trait for creating a scoring functionality
 * typeparam `T` Document key.
 * `document_pointer` reference to a DocumentPointer (a place in the inverted index tree)
 * `idf`Invserse document frequency. Read more at https://en.wikipedia.org/wiki/Okapi_BM25
 * `field_lengths` Field lengths is an array that contains number of terms in each indexed text field.
 * `fields_boost` expected boost from query arguments
 * `expansion_boost` A number between -inf and 1 that describes how similiar the query is to a document in terms of length.
 * `field` information about the fields
*/
pub trait ScoreCalculator<T, M> {
    fn before(
        &self,
        query_term: &str,
        query_term_expanded: &str,
        document_frequency: usize,
        documents: &HashMap<T, Rc<RefCell<DocumentDetails<T>>>>,
    ) -> M;
    fn score(
        &self,
        before_output: &M,
        document_pointer: Ref<DocumentPointer<T>>,
        field_data: &FieldData,
        term_expansion: &TermData,
    ) -> Option<f64>;
}
