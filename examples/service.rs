extern crate iron;
extern crate iron_requestid;

use iron::prelude::*;
use iron::status::Status;
use iron_requestid::{RequestId, RequestIds};

fn main() {
    let mut chain = Chain::new(return_request_handler);

    let (request_ids_before, request_ids_after) = RequestIds::new();
    chain.link_before(request_ids_before);
    chain.link_after(request_ids_after);

    Iron::new(chain).http("127.0.0.1:3000").unwrap();
}

fn return_request_handler(request: &mut Request) -> IronResult<Response> {
   let request_id = request.extensions.get::<RequestId>().unwrap();
   Ok(Response::with((Status::Ok, request_id.to_string())));
}
