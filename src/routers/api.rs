extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate serde;
extern crate serde_json;


use iron::prelude::*;
use self::urlencoded::UrlEncodedQuery;
use std::collections::HashMap;
use iron::Handler;
use utils;
use std::path::{Path, PathBuf};
use iron::mime::Mime;

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

fn paths_to_json_str(files: &Vec<PathBuf>) -> Option<String> {
    let mut path_strs = Vec::new();
    for pathbuf in files {
        let file_name = pathbuf.file_name()
                .and_then(|s| s.to_str())
                .map(|os_str| os_str.to_string());
        if let Some(s) = file_name {
            path_strs.push(s);
        }
    }
    let s = serde_json::to_string(&path_strs);
    s.ok()
}

///
/// list all html in './public'
/// success {"code" : 0, "list" : []}
/// fail {"code" : -1}
///
fn list_blog(_: &mut Request) -> IronResult<Response> {
    let path = Path::new("./public/");
    let results = utils::ls_html(&path);
    let response = match results {
        Err(err) => {
            println!("list blog failed with error: {:?}", err);
            "{\"code\" : -1}".to_string()
        },
        Ok(ref files) => {
            println!("list blog with {:?}", files);
            let json_str = paths_to_json_str(files);
            if let Some(s) = json_str {
                format!("{{\"code\" : 0, \"list\" : {} }}", s)
            }
            else {
                "{\"code\" : -1}".to_string()
            }
        },
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
