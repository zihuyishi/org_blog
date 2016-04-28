extern crate iron;
extern crate time;
extern crate staticfile;
extern crate router;
extern crate mount;
extern crate logger;

use iron::prelude::*;
use router::Router;
use std::path;
use staticfile::Static;
use mount::Mount;
use logger::Logger;

mod routers;

fn main() {
    let mut router = Router::new();

    router.get("/", routers::index);
    router.post("/api", routers::api);

    let mut mount = Mount::new();
    mount.mount("/", router)
        .mount("/public/", Static::new(path::Path::new("target/public/")));
    
    let (logger_before, logger_after) = Logger::new(None);
    
    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    Iron::new(chain).http("localhost:3000").unwrap();
}
