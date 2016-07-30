extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate hyper;


use iron::prelude::*;
use iron::{status, headers};
use iron::method::Method;
use iron::modifiers::Header;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};


#[derive(RustcEncodable, RustcDecodable)]
struct Respond {
	res: String,
}

#[derive(RustcEncodable, RustcDecodable)]
struct FullName {
	first: String,
	last: String,
}

impl FullName {
	///forms a new PollRound object with 0 or empty parameters
	pub fn new() -> FullName {
		FullName {
			first: String::new(),
			last: String::new(),
		}
	}
}

fn main() {

	let name_vector: Vec<FullName> = Vec::new();
	let the_names = Arc::new(Mutex::new(name_vector));
	let names_clone1 = the_names.clone();
	let names_clone2 = the_names.clone();

	let mut router = Router::new();


	router.post("/send_name", move |r: &mut Request| receive_name(r, &mut names_clone1.lock().unwrap()));
	router.get("/get_names", move |r: &mut Request| get_names(r, &names_clone2.lock().unwrap()));

	///post a proposal to the server
	fn receive_name(request: &mut Request, names_store: &mut Vec<FullName>) -> IronResult<Response> {
		let mut payload = String::new();
		request.body.read_to_string(&mut payload).unwrap();
		let import_hold: FullName = json::decode(&payload).unwrap();
		names_store.push(import_hold);
		println!("{:?}", names_store.len());
		let encoded = json::encode(names_store).unwrap();
		println!("{:?}", encoded);

		let resp = Respond { res: "OK".to_string() };
		let resp_string = json::encode(&resp).unwrap();
		let mut response = Response::with((status::Ok, resp_string));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
		println!("OK");
		Ok(response)
	}

	fn get_names(_: &mut Request, names_list: &Vec<FullName>) -> IronResult<Response> {
		let payload = json::encode(names_list).unwrap();
		println!("data: {:?}", &payload);
		let mut response = Response::with((status::Ok, payload.to_string()));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
		println!("OK");
		Ok(response)
	}

		

    Iron::new(router).http("localhost:3100").unwrap();
	
}
