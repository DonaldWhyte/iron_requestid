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
