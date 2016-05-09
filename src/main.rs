/// 
/// A blog just offer static html in some folders
///

extern crate iron;
extern crate staticfile;
extern crate router;
extern crate mount;
extern crate logger;
extern crate serde_json;
extern crate urlencoded;
extern crate serde;


use iron::prelude::*;
use std::path::Path;
use staticfile::Static;
use mount::Mount;
use logger::Logger;
use std::time;

mod routers;
mod utils;

fn link_before(chain: &mut Chain) {
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
}

fn main() {
    let cache_duration = time::Duration::from_secs(7*24*60*60);
    let api_router = routers::api::Router::new();
    
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("public/")).cache(cache_duration))
        .mount("/blog/", Static::new(Path::new("blog/")).cache(cache_duration))
        .mount("/api/", api_router);
    
    let mut chain = Chain::new(mount);
    link_before(&mut chain);

    let port = utils::load_config::i64_by_key("port").unwrap_or(3000);
    let addr = format!("0.0.0.0:{}", port);
    println!("listening on {}", addr);
    Iron::new(chain).http(addr.as_str()).unwrap();
}
