///
/// before middleware to parse url querys
/// ```
/// let hashmap = req.extensions.get<BeforeParams>();
/// ```
///
extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::BeforeMiddleware;
use self::urlencoded::UrlEncodedQuery;

pub struct BeforeParams;

impl iron::typemap::Key for BeforeParams {
    type Value = urlencoded::QueryMap;
}

impl BeforeMiddleware for BeforeParams {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        match req.get::<UrlEncodedQuery>() {
            Ok(hashmap) => {
                req.extensions.insert::<BeforeParams>(hashmap);
            },
            Err(_) => (), 
        }
        Ok(())
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}
