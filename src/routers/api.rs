extern crate iron;
extern crate router;
extern crate urlencoded;


use iron::prelude::*;
use self::urlencoded::UrlEncodedQuery;
use std::collections::HashMap;
use iron::Handler;
use utils;
use std::path::Path;

pub struct Router {
    routers: HashMap<String, Box<Handler>>
}

impl Router {
    pub fn new() -> Self {
        let mut router = Router { routers: HashMap::new() };
        router.default_router();

        router
    }

    pub fn add_route<H>(&mut self, path: String, handler: H)
        where H: Handler
    {
        self.routers.insert(path, Box::new(handler));
    }

    fn default_router(&mut self) {
        self.add_route("add".to_string(), add);
        self.add_route("list_blog".to_string(), list_blog);
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routers.get(&req.url.path.join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(iron::status::NotFound)),
        }
    }
}

fn list_blog(_: &mut Request) -> IronResult<Response> {
    let path = Path::new("./public/");
    let results = utils::ls_html(&path);
    let response = match results {
        Err(err) => {
            println!("list blog failed with error: {:?}", err);
            "can't ls blog".to_string()
        },
        Ok(files) => {
            println!("list blog with {:?}", files);
            "find blog".to_string()
        },
    };

    Ok(Response::with((iron::status::Ok, response)))
}

fn add(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(hashmap) => println!("{:?}", hashmap),
        Err(_) => println!("No query"),
    }
    let result: String = match req.url.query {
        None => "no query".to_string(),
        Some(ref s) => s.clone(),
    };
    Ok(Response::with((iron::status::Ok, result)))
}
