extern crate iron;

use iron::prelude::*;

pub fn api(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Not implement")))
}
