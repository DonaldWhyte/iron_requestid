//! This crate uses UUID4 generated IDs for requests. After installing the
//! middleware, request IDs can be accessed in a request handler like so:
//!
//! ```
//! fn request_handler(request: &mut Request) -> IronResult<Response> {
//!     // ...
//!     let request_id = request.extensions.get::<RequestId>().unwrap();
//!     // ...
//! }
//! ```
//!
//! An example service which returns each request ID to the client is shown
//! below:
//!
//! ```
//! extern crate iron;
//! extern crate iron_requestid;
//!
//! use iron::prelude::*;
//! use iron::status::Status;
//! use iron_requestid::{RequestId, RequestIds};
//!
//! fn main() {
//!     let mut chain = Chain::new(return_request_handler);
//!
//!     let (request_ids_before, request_ids_after) = RequestIds::new();
//!     chain.link_before(request_ids_before);
//!     chain.link_after(request_ids_after);
//!
//!     Iron::new(chain).http("127.0.0.1:3000").unwrap();
//! }
//!
//! fn return_request_handler(request: &mut Request) -> IronResult<Response> {
//!     match request.extensions.get::<RequestId>() {
//!         Some(id) => Ok(Response::with((
//!             Status::Ok,
//!             id.to_string()))),
//!         None => Ok(Response::with((
//!             Status::InternalServerError,
//!             "could not get request ID")))
//!     }
//! }
//! ```

extern crate iron;
extern crate uuid;

use iron::IronResult;
use iron::error::IronError;
use iron::middleware::{BeforeMiddleware, AfterMiddleware};
use iron::request::Request;
use iron::response::Response;
use iron::typemap::Key;
use uuid::Uuid;

pub struct RequestId { }
impl Key for RequestId {
    type Value = Uuid;
}

pub struct RequestIds { }
impl RequestIds {
    pub fn new() -> (RequestIds, RequestIds) {
        (RequestIds {}, RequestIds {})
    }
}

impl BeforeMiddleware for RequestIds {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        request.extensions.insert::<RequestId>(Uuid::new_v4());
        Ok(())
    }

    fn catch(&self, request: &mut Request, err: IronError) -> IronResult<()> {
        request.extensions.remove::<RequestId>();
        Err(err)
    }
}

impl AfterMiddleware for RequestIds {
    fn after(&self, request: &mut Request, res: Response) -> IronResult<Response> {
        request.extensions.remove::<RequestId>();
        Ok(res)
    }

    fn catch(&self, request: &mut Request, err: IronError) -> IronResult<Response> {
        request.extensions.remove::<RequestId>();
        Err(err)
    }
}
