use reqwest;
use futures::Future;
use futures::future::ok;
use std::error::Error;
use token::Token;
use location;
use query::Query;


pub const AUTH_PATH : &str = "https://login.microsoftonline.com/1318d57f-757b-45b3-b1b0-9b3c3842774f/oauth2/authorize";
pub const TOKEN_PATH : &str = "https://login.microsoftonline.com/1318d57f-757b-45b3-b1b0-9b3c3842774f/oauth2/token";
pub const AUTH_RESOURCE_PATH : &str = "https://wegmans-es.azure-api.net";
pub const LOCATION_PRODUCT_PATH : &str = "https://wegmans-es.azure-api.net/locationpublic";

pub struct ClientBuilder { }

impl ClientBuilder {
	pub fn create(client_id: &str, client_secret: &str, redirect_uri: &str) -> impl Future<Item = Client, Error = Box<Error>> {
		// build our request
		let rest_client = reqwest::Client::new();
		let mut req = rest_client.post(TOKEN_PATH)
			.form( &[
				("client_id", client_id), 
				("grant_type", "client_credentials"),
				("redirect_uri", redirect_uri),
				("resource", AUTH_RESOURCE_PATH),
				("client_secret", client_secret)
			]).send().unwrap();

		// send our request
		let result: Token = req.json().unwrap();

		ok(Client { access_token: result.access_token, rest_client: rest_client })
	}
}

#[derive(Debug)]
pub struct Client {
 	access_token: String,
 	rest_client: reqwest::Client
}

impl Client {

	/// search for a wegmans store
	pub fn store_search(&self, query: location::store::StoreQuery) -> impl Future<Item = location::store::Store, Error = Box<Error>> {
	
		let mut req = self.rest_client.get(format!("{}/{}", LOCATION_PRODUCT_PATH, "stores").as_str())
			.form(&query.to_querystring()).send().unwrap();

		ok(req.json().unwrap())
	}
}