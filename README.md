# iron_requestid

### Iron middleware for generating unique IDs for requests.

[![Build Status](https://travis-ci.org/DonaldWhyte/iron_requestid.svg?branch=master)](https://travis-ci.org/DonaldWhyte/iron_requestid) [![Docs](https://docs.rs/iron_requestid/badge.svg)](https://docs.rs/iron_requestid)

This crate uses UUID4 generated IDs for requests. After installing the
middleware, request IDs can be accessed in a request handler like so:

```
fn request_handler(request: &mut Request) -> IronResult<Response> {
    // ...
    let request_id = request.extensions.get::<RequestId>().unwrap();
    // ...
}
```

There is an example service that installs this middleware and retrieves a
request's ID. The code for this is in
[**examples/service.rs**](./examples/service.rs). Run the example service
and send it a request to test it:

```
cargo run --example service &
curl localhost:3000/foo
```

The example service's source code is also shown below:

```
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
    match request.extensions.get::<RequestId>() {
        Some(id) => Ok(Response::with((
            Status::Ok,
            id.to_string()))),
        None => Ok(Response::with((
            Status::InternalServerError,
            "could not get request ID")))
    }
}
```
