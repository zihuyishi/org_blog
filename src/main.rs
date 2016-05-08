/// 
/// A blog just offer static html in some folders
///

extern crate iron;
extern crate staticfile;
extern crate router;
extern crate mount;
extern crate logger;

use iron::prelude::*;
use std::path::Path;
use staticfile::Static;
use mount::Mount;
use logger::Logger;

mod routers;
mod utils;

fn link_before(chain: &mut Chain) {
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
}

fn main() {
    let api_router = routers::api::Router::new();
    
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("public/")))
        .mount("/blog/", Static::new(Path::new("blog/")))
        .mount("/api/", api_router);
    
    let mut chain = Chain::new(mount);
    link_before(&mut chain);

    Iron::new(chain).http("localhost:3000").unwrap();
}
