use iron::prelude::*;
use urlencoded::UrlEncodedQuery;
use std::collections::HashMap;
use iron::Handler;
use utils;
use iron;
use iron::mime::Mime;

pub struct Router {
    routers: HashMap<String, Box<Handler>>,
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

///
/// list all html in './blog'
/// success {"code" : 0, "list" : [Content]}
/// where Content is {"url": filepath, "name": filename}
/// fail {"code" : -1}
///
fn list_blog(_: &mut Request) -> IronResult<Response> {
    let results = utils::ls_blogs();
    let response = match results {
        None => {
            println!("list blog failed");
            "{\"code\" : -1}".to_string()
        }
        Some(ref json) => {
            println!("list blog with {}", json);
            format!("{{\"code\" : 0, \"list\" : {} }}", json)
        }
    };

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, iron::status::Ok, response)))
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
