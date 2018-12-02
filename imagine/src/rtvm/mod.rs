use std::sync::RwLock;

use crate::VM;
mod db;
pub use self::db::*;

pub struct RTVM {
  // last_atom: AtomicUsize,
  // atoms: HashMap<String, usize>,
  db: RwLock<RTDB>,
}

impl VM for RTVM {
  type RelationIndex = Ri;
  type StatementIndex = Si;
  type QueryIndex = Qi;

  fn lookup_relation(&self, template: &str) -> Self::RelationIndex {
    let read_db = self.db.read().expect("no poison");
    match read_db.get_relation(template) {
      Some(relation) => relation,
      None => {
        drop(read_db);
        let mut write_db = self.db.write().expect("no poison");
        write_db.new_relation(template)
      }
    }
  }

  fn insert_statement(
    &self,
    depends_on: Vec<Self::StatementIndex>,
    relation: Self::RelationIndex,
    value: StatementValue,
  ) -> Self::StatementIndex {
    let mut write_db = self.db.write().expect("no poison");
    write_db.insert_statement(depends_on, relation, value)
  }

  fn redact_statement(&self, stmt: Self::StatementIndex) {
    let mut write_db = self.db.write().expect("no poison");
    write_db.redact_statement(stmt);
  }

  fn redact_query(&self, query: Self::QueryIndex) {
    let mut write_db = self.db.write().expect("no poison");
    write_db.redact_query(query);
  }

  fn insert_query(
    &self,
    depends_on: Vec<Self::StatementIndex>,
    query: Query<Self::RelationIndex, Vec<Self::StatementIndex>>,
  ) -> Self::QueryIndex {
    let mut write_db = self.db.write().expect("no poison");
    write_db.insert_query(depends_on, query)
  }
}

impl Default for RTVM {
  fn default() -> Self {
    let db = RTDB::default();
    RTVM {
      db: RwLock::new(db),
    }
  }
}
