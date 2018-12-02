extern crate generational_arena;
use generational_arena::{Arena, Index};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct StatementValue(Vec<Value>);

pub enum QueryPlaceholder {
  Pos(usize),
  Pin(Value),
}

pub struct QueryStatement<R: Sized> {
  relation: R,
  places: Vec<QueryPlaceholder>,
}

pub struct Query<R: Sized> {
  statements: Vec<QueryStatement<R>>,
  positions: usize,
  resolve: Box<Fn(Vec<Si>, Vec<Value>)>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Literal {
  Text(String),
  Int(i64),
}

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Value {
  Literal(Literal),
  Table(Vec<(Literal, Value)>),
  List(Vec<Value>),
}

impl std::fmt::Display for Value {
  fn fmt(&self, w: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Value::Literal(lit) => {
        match lit {
          Literal::Text(text) => write!(w, "Text {:?}", text),
          Literal::Int(i) => write!(w, "Int {:?}", i),
        }
      },
      value => write!(w, "{:?}", value),
    }
  }
}

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

struct RTVM {
  // last_atom: AtomicUsize,
  // atoms: HashMap<String, usize>,
  db: RwLock<RTDB>,
}

type Ri = usize;
type Si = Index;
type Qi = Index;

struct RTDB {
  next_relation: Ri,
  relations: HashMap<String, Ri>,
  /// index used as QueryIndex / Qi
  queries: Arena<Query<Ri>>,
  /// index used as StatementIndex / Si
  statements: Arena<StatementValue>,
  /// indexed by relation usize
  relation_statements: Vec<HashSet<Si>>,
  relation_queries: Vec<HashSet<Qi>>,
  statement_dependents: HashMap<Si, (HashSet<Si>, HashSet<Qi>)>,
}

impl RTDB {
  pub fn new_relation(&mut self, template: &str) -> Ri {
    let new_relation = self.next_relation;
    self.relation_statements.push(HashSet::new());
    self.relation_queries.push(HashSet::new());
    self.next_relation = new_relation + 1;
    assert_eq!(self.next_relation, self.relation_statements.len(), "out of sync!");
    assert_eq!(self.next_relation, self.relation_queries.len(), "out of sync!");
    self.relations.insert(String::from(template), new_relation);
    new_relation
  }

  pub fn insert_statement(
    &mut self,
    depends_on: Vec<Si>,
    relation: Ri,
    value: StatementValue,
  ) -> Si {
    let si = self.statements.insert(value);
    // enter relation -> statement
    self.relation_statements
      .get_mut(relation)
      .expect("relation atom exists")
      .insert(si);
    // enter in as dependents -> statement
    for d_si in depends_on {
      self.statement_dependents
        .entry(d_si)
        .or_default()
        .0
        .insert(si.clone());
    }
    // return statement index
    si
  }

  pub fn redact_statement(&mut self, stmt: Si) {
    self.statements.remove(stmt);
    // enter in as dependents -> statement
    let sq: Option<(Vec<Si>, Vec<Qi>)> = match self.statement_dependents.get(&stmt) {
      Some((si_hs, qi_hs)) => {
        let si_v = si_hs.iter().map(|a| a.clone()).collect::<Vec<_>>();
        let qi_v = qi_hs.iter().map(|a| a.clone()).collect::<Vec<_>>();
        Some((si_v, qi_v))
        // redact_statement for each si
        // redact_query for each qi
      }
      None => None,
    };

    if let Some(sq) = sq {
      for si in sq.0 {
        self.redact_statement(si);
      }
      for qi in sq.1 {
        self.redact_query(qi);
      }
    }
  }

  pub fn redact_query(&mut self, qi: Qi) {
    if let Some(query) = self.queries.remove(qi) {
      for ri in query.statements.iter().map(|qs| qs.relation) {
        self.relation_queries
          .get_mut(ri)
          .expect("relation exists")
          .remove(&qi);
      }
    }
  }

  pub fn insert_query(&mut self, depends_on: Vec<Si>, query: Query<Ri>) -> Qi {
    let ris: Vec<Ri> = query
      .statements
      .iter()
      .map(|qs| qs.relation.to_owned())
      .collect();
    let qi = self.queries.insert(query);

    for ri in ris {
      self.relation_queries
        .get_mut(ri)
        .expect("relation exists")
        .insert(qi);
    }

    // enter in as dependents -> statement
    for d_si in depends_on {
      self.statement_dependents
        .entry(d_si)
        .or_default()
        .1
        .insert(qi.clone());
    }

    self.execute_queries(vec![&qi]);

    qi
  }

  fn execute_queries(&self, qis: Vec<&Qi>) {
    for query in qis
      .into_iter()
      .filter_map(|qi| self.queries.get(qi.clone()))
    {
      for (deps, values) in self.find_all_matches(&query) {
        (query.resolve)(deps, values);
      }
    }
  }

  fn trigger_relation_listeners(&self, relation: Ri, si: Si) {
    self.relation_queries
      .get(relation)
      .and_then(|q_hs| self.statements.get(si).map(|sv| (sv, q_hs)))
      .map(|(sv, q_hs): (&StatementValue, &HashSet<Qi>)| {
        println!("TODO: Found queries to update with {:?}: {:?}", sv, q_hs);
      });
  }

  /// TODO: Make tests for this code!
  fn find_all_matches(&self, query: &Query<Ri>) -> Vec<(Vec<Si>, Vec<Value>)> {
    let qss: Vec<(&QueryStatement<Ri>, Vec<(&Si, &StatementValue)>)> = query
      .statements
      .iter()
      .map(|qs: &QueryStatement<Ri>| (qs, self.find_statements_matching(qs)))
      .collect(); // we must collect so the vecs out-live

    let matches: Vec<query::Match<Si>> = query::find_matches(qss.iter(), query.positions);

    matches
      .into_iter()
      .map(|query::Match { stmts, values }| {
        (
          stmts.into_iter().map(|b_si| b_si.clone()).collect(),
          values.into_iter().map(|b_val| b_val.clone()).collect(),
        )
      })
      .collect()
  }

  fn find_statements_matching(&self, query: &QueryStatement<Ri>) -> Vec<(&Si, &StatementValue)> {
    if let Some(stmt_set) = self.relation_statements.get(query.relation) {
      let places: &[QueryPlaceholder] = query.places.as_slice();
      // let values: &[Value] = ;
      stmt_set
        .iter()
        .filter_map(|index: &Si| {
          self.statements
            .get(index.clone())
            .map(|stmts| (index, stmts))
        })
        .filter(|(si, stmt)| query::does_pins_satisfy_places(places, stmt.0.as_slice()))
        .collect()
    } else {
      Vec::new()
    }
  }
}

trait VM {
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
  fn insert_query(&self, depends_on: Vec<Self::StatementIndex>, query: Query<Self::RelationIndex>) -> Self::QueryIndex;
}

impl VM for RTVM {
  type RelationIndex = Ri;
  type StatementIndex = Si;
  type QueryIndex = Qi;

  fn lookup_relation(&self, template: &str) -> Self::RelationIndex {
    let read_db = self.db.read().expect("no poison");
    match read_db.relations.get(template) {
      Some(relation) => relation.clone(),
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

  fn insert_query(&self, depends_on: Vec<Self::StatementIndex>, query: Query<Self::RelationIndex>) -> Self::QueryIndex {
    let mut write_db = self.db.write().expect("no poison");
    write_db.insert_query(depends_on, query)
  }
}

mod query {
  use super::*;

  pub fn does_pins_satisfy_places(pins: &[QueryPlaceholder], values: &[Value]) -> bool {
    // collect positions?
    pins.iter()
      .zip(values)
      .find(|&(pin, value)| match pin {
        // find pin which doesn't match value
        QueryPlaceholder::Pos(_) => false, // doesn't matter since pin wasn't specified
        QueryPlaceholder::Pin(pin_value) => &pin_value != &value, // found inequal pair
      })
      .is_none() // no inequal pins
  }

  fn pins_satisfied_positions<'a>(
    pins: &'a [QueryPlaceholder],
    values: &'a [Value],
    start_from: Vec<Option<&'a Value>>,
  ) -> Option<Vec<Option<&'a Value>>> {
    let mut positions = start_from.clone();
    if pins
      .iter()
      .zip(values)
      .find(|&(pin, value)| match pin {
        // find pin which doesn't match value
        QueryPlaceholder::Pos(pos) => {
          let pos_usize = pos.clone() as usize;
          if let Some(Some(existing_pos_value)) = start_from.get(pos_usize) {
            existing_pos_value != &value // is inequal?
          } else {
            positions.insert(pos_usize, Some(value)); // insert unknown position
            false // pin wasn't specified / no inequal pair
          }
        }
        QueryPlaceholder::Pin(pin_value) => &pin_value != &value, // found inequal pair
      })
      .is_some()
    {
      // found inequal pins
      None
    } else {
      // no inequal pins
      Some(positions)
    }
  }

  #[derive(Debug, Clone)]
  struct PartialMatch<'a, S> {
    stmts: Vec<&'a S>,
    values: Vec<Option<&'a Value>>,
  }

  impl<'a, S> PartialMatch<'a, S> {
    fn from(stmts: Vec<&'a S>, values: Vec<Option<&'a Value>>) -> Self {
      PartialMatch { stmts, values }
    }
    fn with_values_and_push_stmt(&self, values: Vec<Option<&'a Value>>, stmt: &'a S) -> Self {
      let mut stmts = self.stmts.clone();
      stmts.push(stmt);
      PartialMatch::from(stmts, values)
    }
  }

  #[derive(Debug, Clone)]
  pub struct Match<'a, S> {
    pub stmts: Vec<&'a S>,
    pub values: Vec<&'a Value>,
  }

  impl<'a, S: std::fmt::Debug> Match<'a, S> {
    fn try_from(partial: PartialMatch<'a, S>) -> Result<Self, String> {
      let original_length = partial.values.len();
      let complete: Vec<&'a Value> = partial
        .values
        .into_iter()
        .filter_map(|value| value)
        .collect();
      if complete.len() == original_length {
        Ok(Match {
          stmts: partial.stmts,
          values: complete,
        })
      } else {
        Err(format!("Missing values!"))
      }
    }
  }

  #[allow(dead_code)]
  pub fn find_matches<'a, I, R, S>(input: I, positions: usize) -> Vec<Match<'a, S>>
  where
    // Item = query { relation, many [pin/pos] }, many [stmt { id, relation, values }]
    I: Iterator<Item = &'a (&'a QueryStatement<R>, Vec<(&'a S, &'a StatementValue)>)>,
    R: 'a,
    S: std::fmt::Debug + Clone + 'a,
  {
    partial_matcher(input, positions)
      .into_iter()
      .map(|partial_match| Match::try_from(partial_match).unwrap())
      .collect()
  }

  fn partial_matcher<'a, I, R, S>(input: I, positions: usize) -> Vec<PartialMatch<'a, S>>
  where
    // Item = query { relation, many [pin/pos] }, many [stmt { id, relation, values }]
    I: Iterator<Item = &'a (&'a QueryStatement<R>, Vec<(&'a S, &'a StatementValue)>)>,
    R: 'a,
    S: Clone + 'a,
  {
    //region create empty_solution
    let start: Vec<Option<&Value>> = Vec::with_capacity(positions);

    let empty_solution = PartialMatch::from(Vec::new(), start);
    //endregion create empty_solution
    let mut solutions: Vec<PartialMatch<'a, S>> = vec![empty_solution];

    for (qs, stmts) in input {
      let mut next_solutions: Vec<PartialMatch<'a, S>> = Vec::new();
      for partial_soln in solutions.clone() {
        for (si, StatementValue(stmt_values)) in stmts.iter() {
          if let Some(new_values) = pins_satisfied_positions(
            &qs.places,
            &stmt_values,
            partial_soln.values.clone(),
          ) {
            let new_solution = partial_soln.with_values_and_push_stmt(new_values, &si);
            next_solutions.push(new_solution);
          }
        }
      }
      if next_solutions.is_empty() {
        break;
      }
      solutions = next_solutions;
    }

    solutions
  }
}

impl Default for RTVM {
  fn default() -> Self {
    let db = RTDB {
      next_relation: 0usize,
      relations: HashMap::new(),
      /// index used as QueryIndex / Qi
      queries: Arena::new(),
      /// index used as StatementIndex / Si
      statements: Arena::new(),
      /// indexed by relation usize
      relation_statements: Vec::new(),
      relation_queries: Vec::new(),
      statement_dependents: HashMap::new(),
    };
    RTVM {
      db: RwLock::new(db),
    }
  }
}

impl Value {
  fn text(value: &str) -> Self {
    Value::Literal(Literal::Text(String::from(value)))
  }
}

fn main() {
  println!("Hello, world!");

  let rt = RTVM::default();
  // let rel2 = rt.lookup_relation("_ claims _ points _ at _.");
  // println!("Should be same first rel {:?} == {:?}", rel, rel2);
  // let si2 = rt.insert_statement(vec![], rel2, StatementValue(vec![Value::text("supporter2"), Value::text("page1"), Value::text("up"), Value::text("page2")]));

  let rel = rt.lookup_relation("_ claims _ blahblahblah.");
  let si1 = rt.insert_statement(vec![], rel, StatementValue(vec![Value::text("page1"), Value::text("page2")]));
  let si3 = rt.insert_statement(vec![], rel, StatementValue(vec![Value::text("page2"), Value::text("page3")]));
  let si4 = rt.insert_statement(vec![], rel, StatementValue(vec![Value::text("page0"), Value::text("page3")]));
  let si5 = rt.insert_statement(vec![], rel, StatementValue(vec![Value::text("page3"), Value::text("page1")]));

  let qi1 = rt.insert_query(vec![], Query {
    positions: 3,
    statements: vec![
      QueryStatement {
        relation: rt.lookup_relation("_ claims _ blahblahblah."),
        places: vec![
          QueryPlaceholder::Pos(0),
          QueryPlaceholder::Pos(1),
        ]
      },
      QueryStatement {
        relation: rt.lookup_relation("_ claims _ blahblahblah."),
        places: vec![
          QueryPlaceholder::Pos(1),
          QueryPlaceholder::Pos(2),
        ]
      }
    ],
    resolve: Box::new(|deps: Vec<Si>, vals: Vec<Value>| {
      let page_a = vals.get(0).expect("value");
      let page_b = vals.get(1).expect("value");
      let page_c = vals.get(2).expect("value");

      println!("blahblahblah: {} -> {} -> {}", page_a, page_b, page_c);
    }),
  });
}
