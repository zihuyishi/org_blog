use iron::prelude::*;
use urlencoded::UrlEncodedQuery;
use std::collections::{HashMap, BTreeMap};
use iron::Handler;
use utils;
use std::path::{Path, PathBuf};
use iron;
use iron::mime::Mime;
use serde_json;
use serde_json::Value;

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

fn pathbuf_to_json(pb: &PathBuf) -> Option<Value> {
    let fullname = pb.file_name()
        .and_then(|s| s.to_str())
        .map(|os_str| os_str.to_string());
    let filename = pb.file_stem()
        .and_then(|s| s.to_str())
        .map(|os_str| os_str.to_string());
    
    if let Some(full) = fullname {
        if let Some(file) = filename {
            let mut map = BTreeMap::new();
            map.insert("url".to_string(), Value::String(full));
            map.insert("name".to_string(), Value::String(file));
            return Some(Value::Object(map));
        }
    }
    None
}

fn paths_to_json_str(files: &Vec<PathBuf>) -> Option<String> {
    let path_strs: Vec<Value> = files.iter()
                                      .filter_map(|pb| {
                                          pathbuf_to_json(&pb)
                                      })
                                      .collect();
    let s = serde_json::to_string(&path_strs);
    s.ok()
}

///
/// list all html in './blog'
/// success {"code" : 0, "list" : []}
/// fail {"code" : -1}
///
fn list_blog(_: &mut Request) -> IronResult<Response> {
    let path = Path::new("./blog/");
    let results = utils::ls_html(path);
    let response = match results {
        Err(err) => {
            println!("list blog failed with error: {:?}", err);
            "{\"code\" : -1}".to_string()
        }
        Ok(ref files) => {
            println!("list blog with {:?}", files);
            let json_str = paths_to_json_str(files);
            if let Some(s) = json_str {
                format!("{{\"code\" : 0, \"list\" : {} }}", s)
            } else {
                "{\"code\" : -1}".to_string()
            }
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
