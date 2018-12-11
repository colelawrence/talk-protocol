use crate::{Query, StatementValue};

pub trait VM {
    type RelationIndex;
    type StatementIndex;
    type QueryIndex;
    fn lookup_relation(&self, template: &str) -> Self::RelationIndex;
    fn insert_statement(
        &self,
        depends_on: Vec<Self::StatementIndex>,
        relation: Self::RelationIndex,
        value: StatementValue,
    ) -> Self::StatementIndex;
    fn redact_statement(&self, stmt: Self::StatementIndex);
    fn redact_query(&self, query: Self::QueryIndex);
    fn insert_query(
        &self,
        depends_on: Vec<Self::StatementIndex>,
        query: Query<Self::RelationIndex, Vec<Self::StatementIndex>>,
    ) -> Self::QueryIndex;
}
