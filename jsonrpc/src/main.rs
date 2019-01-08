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

mod lib;
use self::lib::{SerializeDebug, NewStatement};
mod parse_mixed_template;

fn invalid_params<M: Into<String>>(message: M) -> types::error::Error {
	types::error::Error::invalid_params(message)
}

#[derive(Deserialize, Debug)]
struct InsertStatement(String, Vec<Value>);

#[derive(Deserialize, Debug)]
struct InsertParams(String, InsertStatement);

fn main() {
	let mut io = IoHandler::default();
	// {"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".to_owned()))
	});

	io.add_method("insert", |params: types::params::Params| {
		let InsertParams(ref_id, new_statement) = params.parse()?;
		
		Ok(Value::String(format!("new_statement {:?}", new_statement)))
	});

	ServerBuilder::new(io).build();
}
