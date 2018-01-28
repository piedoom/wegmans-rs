extern crate wegmans;
extern crate tokio_core;

use wegmans::client::ClientBuilder;
use tokio_core::reactor::Core;

fn main() {

	// create our futures evaluator for async possibilities
	let mut reactor = Core::new().unwrap();

	let stores = ClientBuilder::create(
		"24960d97-4fbe-433d-ab8a-efeb89aa524e",
		"A8N7VeeCdFD5N4OxeQT1gFaXNStrxieEplYl3SYdxTs=", 
		"http://localhost").and_then( |client| 
		client.store_search(StoreQuery::new().store_number(16));   
	);

	let result = reactor.run(stores).unwrap();

	println!("Our access token is: {}\n", result.access_token);

	
}