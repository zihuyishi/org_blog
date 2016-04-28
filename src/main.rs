extern crate iron;
extern crate time;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use mount::Mount;
use std::path;
use staticfile::Static;

mod routers;

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", routers::index);
    mount.mount("/api", routers::api);
    mount.mount("/public/", Static::new(path::Path::new("target/public/")));
    
    Iron::new(mount).http("localhost:3000").unwrap();
}
