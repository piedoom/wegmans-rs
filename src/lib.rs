 #![feature(conservative_impl_trait)]

extern crate reqwest;
extern crate futures;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_qs;

pub mod client;
pub mod token;
pub mod location;
