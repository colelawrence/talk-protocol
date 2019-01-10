#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate jsonrpc_stdio_server;
extern crate serde;

use jsonrpc_stdio_server::ServerBuilder;
use jsonrpc_stdio_server::jsonrpc_core::*;

use serde_json::Value;
use serde_json::map::Map;

mod lib;
use self::lib::{SerializeDebug};
mod parse_mixed_template;

#[derive(Deserialize, Debug)]
struct NewStatement {
	template: String,
	values: Vec<Value>,
}

#[derive(Deserialize, Debug)]
struct NewQuery {
	query: String,
	defs: Option<Map<String, Value>>,
}

fn main() {
	let mut io = IoHandler::default();

	#[derive(Deserialize, Debug)]
	struct InsertParams(String, NewStatement);
	io.add_method("insert", |params: types::params::Params| {
		let InsertParams(ref_id, new_statement) = params.parse()?;

		// returns void
		Ok(Value::String(format!("new_statement {:?}", new_statement)))
	});

	#[derive(Deserialize, Debug)]
	struct NewEntityParams(String);
	io.add_method("new_entity", |params: types::params::Params| {
		let NewEntityParams(ref_id) = params.parse()?;

		// returns entity string
		Ok(Value::String(format!("new_entity by ref: {:?}", ref_id)))
	});

	#[derive(Deserialize, Debug)]
	struct NewQueryParams(String, NewQuery);
	io.add_method("new_query", |params: types::params::Params| {
		let NewQueryParams(ref_id, new_query) = params.parse()?;

		// returns QId
		Ok(Value::String(format!("new_query {:?}", new_query)))
	});

	#[derive(Deserialize, Debug)]
	struct AwaitParams(String, Option<String>);
	io.add_method("await", |params: types::params::Params| {
		let AwaitParams(query_id, sync) = params.parse()?;

		// returns ({ has_match: boolean, match: NewMatch | null, alive: boolean, sync: string })
		Ok(Value::String(format!("await {} with sync {:?}", query_id, sync)))
	});

	ServerBuilder::new(io).build();
}
