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
use self::lib::SerializeDebug;
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
	struct InsertParams {
		#[serde(rename="ref")]
		reference: String,
		statement: NewStatement
	}
	io.add_method("say", |params: types::params::Params| {
		let InsertParams { reference, statement } = params.parse()?;

		// returns void
		Ok(Value::String(format!("say {:?} by ref: {:?}", statement, reference)))
	});

	#[derive(Deserialize, Debug)]
	struct CreateEntityParams {
		#[serde(rename="ref")]
		reference: String,
	}
	io.add_method("createEntity", |params: types::params::Params| {
		let CreateEntityParams { reference } = params.parse()?;

		// returns entity string
		Ok(Value::String(format!("createEntity by ref: {:?}", reference)))
	});

	#[derive(Deserialize, Debug)]
	struct CreateQueryParams {
		#[serde(rename="ref")]
		reference: String,
		query: NewQuery,
	}
	io.add_method("createQuery", |params: types::params::Params| {
		let CreateQueryParams { reference, query } = params.parse()?;

		// returns QId
		Ok(Value::String(format!("createQuery {:?} by ref: {:?}", query, reference)))
	});

	#[derive(Deserialize, Default, Debug)]
	#[serde(default)]
	struct AwaitParams {
		#[serde(rename="queryId")]
		query_id: String,
		sync: Option<String>
	}
	io.add_method("awaitMatches", |params: types::params::Params| {
		let AwaitParams { query_id, sync } = params.parse()?;

		// returns ({ has_match: boolean, match: NewMatch | null, alive: boolean, sync: string })
		Ok(Value::String(format!("await {} with sync {:?}", query_id, sync)))
	});

	ServerBuilder::new(io).build();
}
