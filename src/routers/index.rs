extern crate iron;

use iron::prelude::*;

pub fn index(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}
